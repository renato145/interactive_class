use awc::ws::{self, Message};
use awc::Client;
use futures::{SinkExt, StreamExt};
use interactive_class::routes::message::{ConnectionType, CupColor, QuestionInfo};
use interactive_class::{
    configuration::get_configuration,
    routes::{message::ClientMessage, CupsInfo},
    telemetry::{get_subscriber, init_subscriber},
    Application,
};
use once_cell::sync::Lazy;
use reqwest::Response;
use std::time::Duration;
use uuid::Uuid;

// Ensure that 'tracing' stack is only initialized once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

type Connection = actix_codec::Framed<awc::BoxedSocket, awc::ws::Codec>;

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn get_ws_connection(&self) -> Connection {
        let (_response, connection) = Client::new()
            .ws(format!("{}/ws", self.address))
            .connect()
            .await
            .expect("Failed to connect to websocket.");
        connection
    }

    /// Gets ws connection, room info and question info
    pub async fn get_ws_room_connection(
        &self,
        room_name: &str,
        connection_type: ConnectionType,
    ) -> (Connection, ClientMessage, Option<ClientMessage>) {
        let msg = serde_json::json!({
            "task": "RoomConnect",
            "payload": {
                "room_name": room_name,
                "connection_type": format!("{connection_type:?}")
            }
        });
        let mut connection = self.get_ws_connection().await;
        let room_info = send_ws_msg(&mut connection, msg).await;
        let questions_info = if matches!(connection_type, ConnectionType::Teacher) {
            Some(get_next_ws_msg(&mut connection).await)
        } else {
            None
        };
        (connection, room_info, questions_info)
    }

    /// Returns teacher and student connections
    pub async fn get_ws_teacher_student_connections(
        &self,
        room_name: &str,
    ) -> (Connection, Connection) {
        let mut teacher_connection = self
            .get_ws_room_connection(room_name, ConnectionType::Teacher)
            .await
            .0;
        let student_connection = self
            .get_ws_room_connection(room_name, ConnectionType::Student)
            .await
            .0;
        get_next_ws_msg(&mut teacher_connection).await;
        (teacher_connection, student_connection)
    }

    pub async fn get_route(&self, route: &str) -> reqwest::Response {
        self.api_client
            .get(format!("{}/{}", &self.address, route))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn create_cups_room(&self, name: &str) -> reqwest::Response {
        self.api_client
            .post(format!("{}/cups/create_room", &self.address))
            .json(&serde_json::json!({ "new_room": name }))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_cups_room(&self, name: &str) -> CupsInfo {
        self.api_client
            .delete(format!("{}/cups/delete_room", &self.address))
            .json(&serde_json::json!({ "room": name }))
            .send()
            .await
            .expect("Failed to execute request.")
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn get_cups_info(&self) -> CupsInfo {
        self.get_route("cups")
            .await
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}

pub async fn spawn_app_with_timeout(timeout: u64) -> TestApp {
    // Set up tracing
    Lazy::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Port 0 give us a random available port
        c.application.port = 0;
        c.websocket.heartbeat_interval = Duration::from_millis(50);
        c.websocket.client_timeout = Duration::from_millis(timeout);
        c
    };

    // Launch app as background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        api_client: client,
    }
}

pub async fn spawn_app() -> TestApp {
    spawn_app_with_timeout(2000).await
}

pub async fn get_next_ws_msg(connection: &mut Connection) -> ClientMessage {
    loop {
        match connection.next().await {
            Some(Ok(ws::Frame::Text(msg))) => {
                return serde_json::from_slice::<ClientMessage>(&msg).unwrap();
            }
            Some(_) => {}
            None => panic!("Time out waiting for ws msg."),
        }
    }
}

pub async fn send_ws_msg(connection: &mut Connection, msg: serde_json::Value) -> ClientMessage {
    connection
        .send(Message::Text(msg.to_string().into()))
        .await
        .expect("Failed to send message.");

    tokio::select! {
        msg = get_next_ws_msg(connection) => {
            msg
        }
        _ = tokio::time::sleep(Duration::from_millis(750)) => {
            panic!("send_ws_msg: Timed out")
        }
    }
}

/// Student selects a cup color
pub async fn select_cup_color(connection: &mut Connection, cup_color: CupColor) -> ClientMessage {
    let msg = serde_json::json!({
        "task": "ChooseCup",
        "payload": format!("{cup_color:?}")
    });
    send_ws_msg(connection, msg).await
}

/// Student selects a cup color
pub async fn create_question(
    connection: &mut Connection,
    title: &str,
    options: &[&str],
) -> QuestionInfo {
    let msg = serde_json::json!({
        "task": "CreateQuestion",
        "payload": {
            "title": title,
            "options": options
        }
    });
    match send_ws_msg(connection, msg).await {
        ClientMessage::QuestionsInfo(d) => d.into_iter().last().unwrap(),
        msg => {
            panic!("Invalid msg: {msg:?}");
        }
    }
}

pub async fn publish_question(connection: &mut Connection, id: Uuid, secs: usize) -> ClientMessage {
    let msg = serde_json::json!({
        "task": "PublishQuestion",
        "payload": {
            "id": id,
            "secs": secs,
        }
    });
    send_ws_msg(connection, msg).await
}

pub async fn delete_question(connection: &mut Connection, id: Uuid) -> ClientMessage {
    let msg = serde_json::json!({
        "task": "DeleteQuestion",
        "payload": id
    });
    send_ws_msg(connection, msg).await
}

pub async fn modify_question(
    connection: &mut Connection,
    id: Uuid,
    title: Option<&str>,
    options: Option<Vec<&str>>,
) -> ClientMessage {
    let msg = serde_json::json!({
        "task": "ModifyQuestion",
        "payload": {
            "id": id,
            "title": title,
            "options": options
        }
    });
    send_ws_msg(connection, msg).await
}

pub async fn answer_question(
    connection: &mut Connection,
    id: Uuid,
    answer: usize,
) -> ClientMessage {
    let msg = serde_json::json!({
        "task": "AnswerQuestion",
        "payload": {
            "id": id,
            "answer": answer,
        }
    });
    send_ws_msg(connection, msg).await
}

#[allow(unused)]
pub fn assert_is_redirect_to(response: &Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}

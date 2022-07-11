use awc::Client;
use interactive_class::{
    configuration::get_configuration,
    routes::{CupsInfo, RoomInfo},
    telemetry::{get_subscriber, init_subscriber},
    Application,
};
use once_cell::sync::Lazy;
use reqwest::Response;
use std::time::Duration;

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

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn get_ws_connection(&self) -> actix_codec::Framed<awc::BoxedSocket, awc::ws::Codec> {
        let (_response, connection) = Client::new()
            .ws(format!("{}/ws", self.address))
            .connect()
            .await
            .expect("Failed to connect to websocket.");
        connection
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

    pub async fn get_cups_info(&self) -> CupsInfo {
        self.get_route("cups")
            .await
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn get_room_info(&self, room_name: &str) -> RoomInfo {
        self.get_route(&format!("cups/{room_name}"))
            .await
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}

pub async fn spawn_app() -> TestApp {
    // Set up tracing
    Lazy::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Port 0 give us a random available port
        c.application.port = 0;
        c.websocket.heartbeat_interval = Duration::from_millis(50);
        c.websocket.client_timeout = Duration::from_millis(250);
        c
    };

    // Launch app as background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        api_client: client,
    };
    test_app
}

#[allow(unused)]
pub fn assert_is_redirect_to(response: &Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}

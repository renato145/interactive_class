use crate::helpers::{send_ws_msg, spawn_app};
use interactive_class::routes::message::ClientMessage;

#[actix_rt::test]
async fn create_question_works() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let title = "test question";
    let options = vec!["option1", "option2", "option3"];
    let connect_msg = serde_json::json!({
        "task": "RoomConnect",
        "payload": {
            "room_name": room_name,
            "connection_type": "Teacher"
        }
    });
    let msg = serde_json::json!({
        "task": "CreateQuestion",
        "payload": {
            "title": title,
            "options": options
        }
    });

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let mut connection = app.get_ws_connection().await;
    send_ws_msg(&mut connection, connect_msg).await;
    // Create question
    let msg = send_ws_msg(&mut connection, msg).await;

    // Assert
    match msg {
        ClientMessage::QuestionInfo(info) => {
            let question_state = info.0.values().collect::<Vec<_>>()[0];
            assert_eq!(question_state.title, title);
            assert_eq!(question_state.options, options);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn students_see_questions_on_publish() {
    todo!();
}

#[actix_rt::test]
async fn delete_questions_works() {
    todo!();
}

#[actix_rt::test]
async fn modify_questions_works() {
    todo!();
}

use crate::helpers::{
    create_question, delete_question, get_next_ws_msg, publish_question, spawn_app,
};
use interactive_class::routes::message::{ClientMessage, ConnectionType};

#[actix_rt::test]
async fn create_question_works() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let title = "test question";
    let options = vec!["option1", "option2", "option3"];

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut connection, _) = app
        .get_ws_room_connection(room_name, ConnectionType::Teacher)
        .await;
    // Create question
    let (_, question_state) = create_question(&mut connection, title, &options).await;

    // Assert
    assert_eq!(question_state.title, title);
    assert_eq!(question_state.options, options);
}

#[actix_rt::test]
async fn students_see_questions_on_publish() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let title = "test question";
    let options = vec!["option1", "option2", "option3"];

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut teacher_connection, mut student_connection) =
        app.get_ws_teacher_student_connections(room_name).await;
    // Create question
    let (question_id, _question_state) =
        create_question(&mut teacher_connection, title, &options).await;
    // Publish question
    publish_question(&mut teacher_connection, question_id).await;
    let msg = get_next_ws_msg(&mut student_connection).await;

    // Assert
    match msg {
        ClientMessage::QuestionPublication(question) => {
            assert_eq!(question.title, title);
            assert_eq!(question.options, options);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn delete_questions_works() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let title = "test question";
    let options = vec!["option1", "option2", "option3"];

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut connection, _) = app
        .get_ws_room_connection(room_name, ConnectionType::Teacher)
        .await;
    // Create question
    let (id, _) = create_question(&mut connection, title, &options).await;
    // Delete question
    let msg = delete_question(&mut connection, id).await;

    // Assert
    match msg {
        ClientMessage::QuestionInfo(info) => {
            assert_eq!(info.0.len(), 0);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn modify_questions_works() {
    todo!();
}

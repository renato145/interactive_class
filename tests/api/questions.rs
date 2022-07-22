use crate::helpers::{
    answer_question, create_question, delete_question, get_next_ws_msg, modify_question,
    publish_question, spawn_app,
};
use futures::SinkExt;
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
    let (mut connection, _, _) = app
        .get_ws_room_connection(room_name, ConnectionType::Teacher)
        .await;
    // Create question
    let question_info = create_question(&mut connection, title, &options).await;

    // Assert
    assert_eq!(question_info.title, title);
    assert_eq!(question_info.options, options);
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
    let question_info = create_question(&mut teacher_connection, title, &options).await;
    // Publish question
    publish_question(&mut teacher_connection, question_info.id.0, 1).await;
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
    let (mut connection, _, _) = app
        .get_ws_room_connection(room_name, ConnectionType::Teacher)
        .await;
    // Create question
    let question_info = create_question(&mut connection, title, &options).await;
    // Delete question
    let msg = delete_question(&mut connection, question_info.id.0).await;

    // Assert
    match msg {
        ClientMessage::QuestionsInfo(info) => {
            assert_eq!(info.len(), 0);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn modify_questions_works() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let title = "test question";
    let options = vec!["option1", "option2", "option3"];
    let test_cases = vec![
        (Some("new title"), None, "new title"),
        (None, Some(vec!["an option"]), "new options"),
        (
            Some("new title"),
            Some(vec!["an option"]),
            "new title and options",
        ),
    ];

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut connection, _, _) = app
        .get_ws_room_connection(room_name, ConnectionType::Teacher)
        .await;
    // Create question
    let question_info = create_question(&mut connection, title, &options).await;

    for (new_title, new_options, description) in test_cases {
        // Modify question
        let msg = modify_question(
            &mut connection,
            question_info.id.0,
            new_title,
            new_options.clone(),
        )
        .await;
        // Assert
        match msg {
            ClientMessage::QuestionsInfo(info) => {
                let question_info = info.iter().last().unwrap();
                if let Some(new_title) = new_title {
                    assert_eq!(question_info.title, new_title, "{description}");
                }
                if let Some(new_options) = new_options {
                    assert_eq!(question_info.options, new_options, "{description}");
                }
            }
            msg => panic!("Invalid msg: {msg:?}"),
        }
    }
}

#[actix_rt::test]
async fn student_can_answer_questions() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let title = "test question";
    let options = vec!["option1", "option2", "option3"];
    let answer = 1;

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut teacher_connection, mut student_connection) =
        app.get_ws_teacher_student_connections(room_name).await;
    // Create question
    let question_info = create_question(&mut teacher_connection, title, &options).await;
    // Answer questions
    answer_question(&mut student_connection, question_info.id.0, answer).await;
    let msg = get_next_ws_msg(&mut teacher_connection).await;

    // Assert
    match msg {
        ClientMessage::QuestionsInfo(info) => {
            let question = info.iter().last().unwrap();
            assert_eq!(question.answers[answer], 1)
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn modify_questions_keep_answer_information() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let title = "test question";
    let options = vec!["option1", "option2", "option3"];
    let test_cases = vec![
        (
            None,
            Some(vec!["option1", "option2"]),
            None,
            "modify when no answers",
        ),
        (
            Some(0),
            Some(vec!["option1", "option2"]),
            Some(1),
            "modify unanswered",
        ),
        (
            Some(1),
            Some(vec!["option1", "new option2"]),
            Some(0),
            "modify answered",
        ),
        (
            Some(2),
            Some(vec!["option1", "option2"]),
            None,
            "remove answered",
        ),
    ];

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut teacher_connection, mut student_connection) =
        app.get_ws_teacher_student_connections(room_name).await;
    // Create question
    let question_info = create_question(&mut teacher_connection, title, &options).await;
    let id = question_info.id.0;

    for (answer, new_options, expected, description) in test_cases {
        // Answer question
        if let Some(answer) = answer {
            answer_question(&mut student_connection, id, answer).await;
            get_next_ws_msg(&mut teacher_connection).await;
        }
        // Modify question
        let msg = modify_question(&mut teacher_connection, id, None, new_options).await;
        // Assert
        match msg {
            ClientMessage::QuestionsInfo(info) => {
                let question = info.iter().last().unwrap();
                if let Some(answer) = answer {
                    assert_eq!(
                        question.answers.get(answer).cloned(),
                        expected,
                        "{description}"
                    );
                }
            }
            msg => panic!("Invalid msg: {msg:?}"),
        }
        // Clean up: reset question
        modify_question(
            &mut teacher_connection,
            id,
            Some(title),
            Some(options.clone()),
        )
        .await;
    }
}

#[actix_rt::test]
async fn answers_get_deleted_when_student_disconnects() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let title = "test question";
    let options = vec!["option1", "option2", "option3"];
    let answer = 1;

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut teacher_connection, mut student_connection) =
        app.get_ws_teacher_student_connections(room_name).await;
    // Create question
    let question_info = create_question(&mut teacher_connection, title, &options).await;
    // Answer questions
    answer_question(&mut student_connection, question_info.id.0, answer).await;
    get_next_ws_msg(&mut teacher_connection).await;
    // Student disconnects
    student_connection.close().await.unwrap();
    get_next_ws_msg(&mut teacher_connection).await;
    let msg = get_next_ws_msg(&mut teacher_connection).await;

    // Assert
    match msg {
        ClientMessage::QuestionsInfo(info) => {
            let question = info.iter().last().unwrap();
            assert_eq!(question.answers[answer], 0)
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

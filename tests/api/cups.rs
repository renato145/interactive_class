use crate::helpers::{get_next_ws_msg, select_cup_color, spawn_app};
use futures::SinkExt;
use interactive_class::routes::{
    message::{ClientMessage, ConnectionType, CupColor},
    CupsInfo,
};
use std::collections::HashSet;

#[tokio::test]
async fn get_cups_info() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let cups_info = app.get_cups_info().await;

    // Assert
    let expected = CupsInfo::default();
    assert_eq!(cups_info, expected);
}

#[tokio::test]
async fn create_new_room() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";

    // Act
    let response = app.create_cups_room(room_name).await;

    // Assert
    response.error_for_status().unwrap();
}

#[tokio::test]
async fn get_cups_info_after_rooms_are_created() {
    // Arrange
    let app = spawn_app().await;
    app.create_cups_room("room1").await;
    app.create_cups_room("room2").await;

    // Act
    let cups_info = app.get_cups_info().await;

    // Assert
    let rooms = HashSet::from_iter(["room1".to_string(), "room2".to_string()]);
    let expected = CupsInfo { rooms };
    assert_eq!(cups_info, expected);
}

#[actix_rt::test]
async fn get_info_when_student_connects() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Student connects
    let (_, room_info, questions_info) = app
        .get_ws_room_connection(room_name, ConnectionType::Student)
        .await;

    // Assert
    match room_info {
        ClientMessage::RoomInfo(msg) => {
            assert_eq!(&msg.name, room_name);
            assert_eq!(msg.connections, 1);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }

    match questions_info {
        Some(ClientMessage::QuestionsInfo(msg)) => {
            assert_eq!(msg.len(), 0);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn fail_to_get_info_when_student_connects_to_unexisting_room() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";

    // Act
    // Student connects
    let (_, room_info, questions_info) = app
        .get_ws_room_connection(room_name, ConnectionType::Student)
        .await;

    // Assert
    match room_info {
        ClientMessage::Error(msg) => {
            assert_eq!(&msg, "Invalid room: \"test_room\".");
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
    assert!(questions_info.is_none());
}

#[actix_rt::test]
async fn get_room_info_when_second_student_connects() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut connection, _conn1) = app.get_ws_teacher_student_connections(room_name).await;
    // Second student connects
    let (_conn2, _, _) = app
        .get_ws_room_connection(room_name, ConnectionType::Student)
        .await;
    let msg = get_next_ws_msg(&mut connection).await;

    // Assert
    match dbg!(msg) {
        ClientMessage::RoomInfo(msg) => {
            assert_eq!(&msg.name, room_name);
            assert_eq!(msg.connections, 2);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn get_room_info_refreshes_when_second_student_disconnects() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut connection, _conn1) = app.get_ws_teacher_student_connections(room_name).await;
    // Second student connects
    let (mut conn2, _, _) = app
        .get_ws_room_connection(room_name, ConnectionType::Student)
        .await;
    get_next_ws_msg(&mut connection).await;
    // Second student disconnects
    conn2.close().await.unwrap();
    let msg = get_next_ws_msg(&mut connection).await;

    // Assert
    match dbg!(msg) {
        ClientMessage::RoomInfo(msg) => {
            assert_eq!(&msg.name, room_name);
            assert_eq!(msg.connections, 1);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn teacher_gets_msg_when_student_chooses_a_cup() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut teacher_connection, mut student_connection) =
        app.get_ws_teacher_student_connections(room_name).await;
    // Student chooses a cup
    select_cup_color(&mut student_connection, CupColor::Yellow).await;
    let msg = get_next_ws_msg(&mut teacher_connection).await;

    // Assert
    match msg {
        ClientMessage::RoomInfo(msg) => {
            assert_eq!(&msg.name, room_name);
            assert_eq!(msg.yellow, 1);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn choosing_cup_fails_if_not_connected_to_room() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let mut connection = app.get_ws_connection().await;
    // Student chooses a cup
    let msg = select_cup_color(&mut connection, CupColor::Yellow).await;

    // Assert
    match msg {
        ClientMessage::Error(msg) => {
            assert_eq!(&msg, "No connected to any room.");
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn changing_cup_color_works() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Start connections
    let (mut teacher_connection, mut student_connection) =
        app.get_ws_teacher_student_connections(room_name).await;
    // Student chooses a cup
    select_cup_color(&mut student_connection, CupColor::Yellow).await;
    get_next_ws_msg(&mut teacher_connection).await;
    // Student choose a different cup
    select_cup_color(&mut student_connection, CupColor::Red).await;
    let msg = get_next_ws_msg(&mut teacher_connection).await;

    // Assert
    match msg {
        ClientMessage::RoomInfo(msg) => {
            assert_eq!(&msg.name, room_name);
            assert_eq!(msg.yellow, 0);
            assert_eq!(msg.red, 1);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

use crate::helpers::{send_ws_msg, spawn_app};
use interactive_class::routes::{message::ClientMessage, CupsInfo};
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
async fn get_room_info_when_client_connects() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let msg = serde_json::json!({
        "task": "RoomConnect",
        "payload": room_name
    });

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Client connects
    let mut connection = app.get_ws_connection().await;
    let msg = send_ws_msg(&mut connection, msg).await;

    // Assert
    match msg {
        ClientMessage::RoomInfo(msg) => {
            assert_eq!(&msg.name, room_name);
            assert_eq!(msg.connections, 1);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn fail_to_get_room_info_when_client_connects_to_unexisting_room() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let msg = serde_json::json!({
        "task": "RoomConnect",
        "payload": room_name
    });

    // Act
    // Client connects
    let mut connection = app.get_ws_connection().await;
    let msg = send_ws_msg(&mut connection, msg).await;

    // Assert
    match msg {
        ClientMessage::Error(msg) => {
            assert_eq!(&msg, "Room not found \"test_room\"");
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

#[actix_rt::test]
async fn get_room_info_when_second_client_connects() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";
    let msg = serde_json::json!({
        "task": "RoomConnect",
        "payload": room_name
    });

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // First client connects
    let mut connection1 = app.get_ws_connection().await;
    send_ws_msg(&mut connection1, msg.clone()).await;
    // Second client connects
    let mut connection2 = app.get_ws_connection().await;
    let msg = send_ws_msg(&mut connection2, msg).await;

    // Assert
    match msg {
        ClientMessage::RoomInfo(msg) => {
            assert_eq!(&msg.name, room_name);
            assert_eq!(msg.connections, 2);
        }
        msg => panic!("Invalid msg: {msg:?}"),
    }
}

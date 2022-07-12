use crate::helpers::spawn_app;
use awc::ws::{self, Message};
use futures::{SinkExt, StreamExt};
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
async fn get_room_info_after_someone_connects() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";

    // Act
    // Create room
    app.create_cups_room(room_name).await;
    // Client connects
    let mut connection = app.get_ws_connection().await;
    connection
        .send(Message::Text(r#"{"task": "RoomConnect"}"#.into()))
        .await
        .expect("Failed to send message.");
    // Get room info
    let msg = match connection.next().await.unwrap().unwrap() {
        ws::Frame::Text(msg) => serde_json::from_slice::<ClientMessage>(&msg).unwrap(),
        msg => panic!("Invalid msg: {:?}", msg),
    };

    // Assert
    assert!(msg.success);
    let expected = "".to_string();
    assert_eq!(msg.payload.unwrap(), expected);
}

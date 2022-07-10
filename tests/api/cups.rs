use crate::helpers::spawn_app;
use interactive_class::routes::{CupsInfo, RoomInfo};

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
    let cups_info = app.get_room_info(room_name).await;

    // Assert
    let expected = RoomInfo::new(room_name);
    assert_eq!(cups_info, expected);
}

#[actix_rt::test]
async fn get_room_info_after_someone_connects() {
    // Arrange
    let app = spawn_app().await;
    let room_name = "test_room";

    // Act
    // Create room
    app.get_room_info(room_name).await;
    // Client connects
    // Get room info
    let cups_info = app.get_room_info(room_name).await;

    // Assert
    todo!();
    // let expected = RoomInfo::new(room_name);
    // assert_eq!(cups_info, expected);
}

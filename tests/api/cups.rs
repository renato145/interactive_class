use crate::helpers::spawn_app;
use interactive_class::routes::{CupsInfo, RoomInfo};

#[tokio::test]
async fn get_cups_info() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let cups_info: CupsInfo = app
        .get_cups_info()
        .await
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

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
    let cups_info: RoomInfo = app
        .get_route(&format!("cups/{room_name}"))
        .await
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    // Assert
    let expected = RoomInfo::new(room_name);
    assert_eq!(cups_info, expected);
}

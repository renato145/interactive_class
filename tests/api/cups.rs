use crate::helpers::spawn_app;
use serde_json::json;

#[actix_rt::test]
async fn teacher_can_get_cup_rooms() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let cups_info: serde_json::Value = app
        .get_cup_rooms()
        .await
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    // Assert
    assert_eq!(cups_info["rooms"], json!(0));
}

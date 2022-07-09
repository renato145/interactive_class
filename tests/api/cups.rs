use crate::helpers::spawn_app;

#[actix_rt::test]
async fn teacher_can_get_cup_rooms() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_cup_rooms().await.error_for_status().unwrap();
    println!("====> {:?}", response);
    // response should be json

    // Assert
    // validate that it return 0 rooms
}

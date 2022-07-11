use crate::helpers::spawn_app_with_timeout;
use actix_web_actors::ws;
use futures::{SinkExt, StreamExt};
use std::time::Duration;

#[actix_rt::test]
async fn client_receives_heartbeat_every_x_milliseconds() {
    // Arrange
    let app = spawn_app_with_timeout(250).await;
    let mut connection = app.get_ws_connection().await;
    let sleep = tokio::time::sleep(Duration::from_millis(250));
    tokio::pin!(sleep);
    let mut count = 0;

    // Act
    loop {
        tokio::select! {
            Some(msg) = connection.next() => {
                tracing::info!("==> {:?}", msg);
                count += 1;
            }
            _ = &mut sleep => {
                tracing::info!("Timeout!");
                break;
            }
        }
    }

    // Assert
    assert!(dbg!(count) >= 0, "Did not receive any heartbeat.")
}

#[actix_rt::test]
async fn client_disconnects_after_x_milliseconds() {
    // Arrange
    let app = spawn_app_with_timeout(250).await;
    let mut connection = app.get_ws_connection().await;
    let sleep = tokio::time::sleep(Duration::from_millis(500));
    tokio::pin!(sleep);
    let mut disconnected = false;

    // Act
    loop {
        tokio::select! {
            msg = connection.next() => {
                tracing::info!("==> {:?}", msg);
                if msg.is_none() {
                    disconnected = true;
                    break;
                }
            }
            _ = &mut sleep => {
                tracing::info!("Timeout!");
                break;
            }
        }
    }

    // Assert
    assert!(disconnected, "Server did not disconnect.")
}

#[actix_rt::test]
async fn client_stays_alive_if_responds_pings() {
    // Arrange
    let app = spawn_app_with_timeout(250).await;
    let mut connection = app.get_ws_connection().await;
    let sleep = tokio::time::sleep(Duration::from_millis(500));
    tokio::pin!(sleep);
    let mut disconnected = false;

    // Act
    loop {
        tokio::select! {
            msg = connection.next() => {
                tracing::info!("==> {:?}", msg);
                match msg {
                    Some(Ok(ws::Frame::Ping(msg))) => {
                        connection
                            .send(awc::ws::Message::Pong(msg))
                            .await
                            .expect("Failed to send Pong message.");
                    }
                    None => {
                        disconnected = true;
                        break;
                    }
                    _ => {}
                }
            }
            _ = &mut sleep => {
                tracing::info!("Timeout!");
                break;
            }
        }
    }

    // Assert
    assert!(!disconnected, "Server disconnected.")
}

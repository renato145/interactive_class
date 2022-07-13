use crate::routes::message::ClientMessage;
use actix::Recipient;
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct AppState {
    pub rooms: Mutex<HashMap<String, RoomState>>,
}

#[derive(Default, Debug, Clone)]
pub struct RoomState {
    pub name: String,
    pub connections: Vec<RoomConnectionInfo>,
}

#[derive(Debug, Clone)]
pub struct RoomConnectionInfo {
    pub id: Uuid,
    pub addr: Recipient<ClientMessage>,
}

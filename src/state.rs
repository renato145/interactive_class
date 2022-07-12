use crate::routes::message::ClientMessage;
use actix::Recipient;
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug)]
pub struct AppState {
    pub rooms: Mutex<HashMap<String, RoomState>>,
}

#[derive(Debug)]
pub struct RoomState {
    pub connections: Vec<RoomConnectionInfo>,
}

#[derive(Debug)]
pub struct RoomConnectionInfo {
    pub id: String,
    pub addr: Recipient<ClientMessage>,
}

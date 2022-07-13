use crate::routes::message::ClientMessage;
use actix::Recipient;
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct AppState {
    pub rooms: Mutex<HashMap<String, RoomState>>,
}

#[derive(Debug, Clone)]
pub struct RoomState {
    pub name: String,
    pub connections: HashMap<Uuid, Recipient<ClientMessage>>,
}

impl RoomState {
    pub fn new(name: String) -> Self {
        Self {
            name,
            connections: HashMap::new(),
        }
    }
}

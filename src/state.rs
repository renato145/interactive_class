use crate::routes::message::{ClientMessage, CupColor};
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
    pub student_connections: HashMap<Uuid, Recipient<ClientMessage>>,
    pub teacher_connections: HashMap<Uuid, Recipient<ClientMessage>>,
    pub green: usize,
    pub yellow: usize,
    pub red: usize,
}

impl RoomState {
    pub fn new(name: String) -> Self {
        Self {
            name,
            student_connections: HashMap::new(),
            teacher_connections: HashMap::new(),
            green: 0,
            yellow: 0,
            red: 0,
        }
    }

    pub fn add_cup(&mut self, color: CupColor) {
        match color {
            CupColor::Green => self.green += 1,
            CupColor::Yellow => self.yellow += 1,
            CupColor::Red => self.red += 1,
        }
    }
}

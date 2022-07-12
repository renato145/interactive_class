use std::{collections::HashMap, sync::Mutex};

#[derive(Default, Debug)]
pub struct AppState {
    pub rooms: Mutex<HashMap<String, usize>>,
}

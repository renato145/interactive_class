use std::{collections::HashSet, sync::Mutex};

#[derive(Default, Debug)]
pub struct AppState {
    pub rooms: Mutex<HashSet<String>>,
}

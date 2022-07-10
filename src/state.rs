use std::sync::Mutex;

#[derive(Default, Debug)]
pub struct AppState {
    pub rooms: Mutex<Vec<String>>,
}

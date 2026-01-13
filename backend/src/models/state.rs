use std::sync::Mutex;
use std::collections::HashMap;

pub struct AppState {
    pub pending_verifications: Mutex<HashMap<String, String>>,
}  
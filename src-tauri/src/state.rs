use std::collections::HashMap;
use std::sync::Mutex;

pub struct Session {
    pub username: String,
    pub privkey: [u8; 32],
    pub passphrase_hash: String,
}

pub struct AppState {
    pub session: Mutex<Option<Session>>,
    pub known_pubkeys: Mutex<HashMap<String, [u8; 32]>>,
    pub worker_url: String,
}

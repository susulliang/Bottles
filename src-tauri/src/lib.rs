mod api;
mod commands;
mod crypto;
mod state;

use std::collections::HashMap;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state::AppState {
            session: Mutex::new(None),
            known_pubkeys: Mutex::new(HashMap::new()),
            worker_url: std::env::var("WORKER_URL")
                .unwrap_or_else(|_| "https://bottles-worker.bottles-susull.workers.dev".into()),
        })
        .invoke_handler(tauri::generate_handler![
            commands::register,
            commands::login,
            commands::logout,
            commands::throw_bottle,
            commands::fetch_bottles,
            commands::open_bottle,
            commands::delete_bottle,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

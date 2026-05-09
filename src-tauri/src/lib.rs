mod commands;
mod db;
mod models;

use db::{open_database, DbState};
use std::sync::Mutex;
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let conn = open_database(&app.handle())?;
            app.manage(DbState {
                connection: Mutex::new(conn),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_todos,
            commands::create_todo,
            commands::set_completed,
            commands::delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

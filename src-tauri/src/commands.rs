use crate::db;
use crate::db::DbState;
use crate::models::{CreateTodoInput, Todo};
use chrono::Utc;
use tauri::State;

#[tauri::command]
pub fn list_todos(state: State<DbState>) -> Result<Vec<Todo>, String> {
    let conn = state
        .connection
        .lock()
        .map_err(|err| format!("Failed to lock DB connection: {err}"))?;
    db::list_todos(&conn)
}

#[tauri::command]
pub fn create_todo(state: State<DbState>, input: CreateTodoInput) -> Result<Todo, String> {
    let conn = state
        .connection
        .lock()
        .map_err(|err| format!("Failed to lock DB connection: {err}"))?;
    db::create_todo(&conn, &input)
}

#[tauri::command]
pub fn set_completed(state: State<DbState>, id: String, completed: bool) -> Result<Todo, String> {
    let conn = state
        .connection
        .lock()
        .map_err(|err| format!("Failed to lock DB connection: {err}"))?;
    db::set_completed(&conn, &id, completed)
}

#[tauri::command]
pub fn delete_todo(state: State<DbState>, id: String) -> Result<(), String> {
    let conn = state
        .connection
        .lock()
        .map_err(|err| format!("Failed to lock DB connection: {err}"))?;
    db::delete_todo(&conn, &id)
}

#[tauri::command]
pub fn get_system_time() -> String {
    Utc::now().to_rfc3339()
}

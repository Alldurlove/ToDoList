use crate::models::{CreateTodoInput, ReminderConfig, Todo};
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

pub struct DbState {
    pub connection: Mutex<Connection>,
}

pub fn open_database(app: &AppHandle) -> Result<Connection, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| format!("Failed to locate app data directory: {err}"))?;

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|err| format!("Failed to create app data directory: {err}"))?;

    let db_path: PathBuf = app_data_dir.join("todos.db");
    let conn = Connection::open(db_path).map_err(|err| format!("Failed to open database: {err}"))?;
    migrate(&conn)?;
    Ok(conn)
}

fn migrate(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS todos (
          id TEXT PRIMARY KEY,
          title TEXT NOT NULL,
          completed INTEGER NOT NULL DEFAULT 0,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL,
          due_at TEXT NULL,
          reminder_json TEXT NULL
        );
        "
    )
    .map_err(|err| format!("Database migration failed: {err}"))
}

pub fn list_todos(conn: &Connection) -> Result<Vec<Todo>, String> {
    let mut stmt = conn
        .prepare(
            "
            SELECT id, title, completed, created_at, updated_at, due_at, reminder_json
            FROM todos
            ORDER BY completed ASC, created_at DESC
            "
        )
        .map_err(|err| format!("Failed to prepare list query: {err}"))?;

    let rows = stmt
        .query_map([], |row| {
            let reminder_json: Option<String> = row.get(6)?;
            let reminder = reminder_json
                .as_deref()
                .and_then(|raw| serde_json::from_str::<ReminderConfig>(raw).ok());

            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get::<_, i64>(2)? != 0,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                due_at: row.get(5)?,
                reminder,
            })
        })
        .map_err(|err| format!("Failed to run list query: {err}"))?;

    let mut todos = Vec::new();
    for row in rows {
        todos.push(row.map_err(|err| format!("Failed to parse todo row: {err}"))?);
    }
    Ok(todos)
}

pub fn create_todo(conn: &Connection, input: &CreateTodoInput) -> Result<Todo, String> {
    let now = Utc::now().to_rfc3339();
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title: input.title.trim().to_string(),
        completed: false,
        created_at: now.clone(),
        updated_at: now,
        due_at: input.due_at.clone(),
        reminder: None,
    };

    conn.execute(
        "
        INSERT INTO todos (id, title, completed, created_at, updated_at, due_at, reminder_json)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ",
        params![
            todo.id,
            todo.title,
            if todo.completed { 1_i64 } else { 0_i64 },
            todo.created_at,
            todo.updated_at,
            todo.due_at,
            Option::<String>::None
        ]
    )
    .map_err(|err| format!("Failed to insert todo: {err}"))?;

    Ok(todo)
}

pub fn set_completed(conn: &Connection, id: &str, completed: bool) -> Result<Todo, String> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE todos SET completed = ?1, updated_at = ?2 WHERE id = ?3",
        params![if completed { 1_i64 } else { 0_i64 }, now, id]
    )
    .map_err(|err| format!("Failed to update todo: {err}"))?;

    get_todo(conn, id)
}

pub fn delete_todo(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM todos WHERE id = ?1", [id])
        .map_err(|err| format!("Failed to delete todo: {err}"))?;
    Ok(())
}

fn get_todo(conn: &Connection, id: &str) -> Result<Todo, String> {
    let mut stmt = conn
        .prepare(
            "
            SELECT id, title, completed, created_at, updated_at, due_at, reminder_json
            FROM todos
            WHERE id = ?1
            "
        )
        .map_err(|err| format!("Failed to prepare get query: {err}"))?;

    stmt.query_row([id], |row| {
        let reminder_json: Option<String> = row.get(6)?;
        let reminder = reminder_json
            .as_deref()
            .and_then(|raw| serde_json::from_str::<ReminderConfig>(raw).ok());
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get::<_, i64>(2)? != 0,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            due_at: row.get(5)?,
            reminder,
        })
    })
    .map_err(|err| format!("Failed to fetch updated todo: {err}"))
}

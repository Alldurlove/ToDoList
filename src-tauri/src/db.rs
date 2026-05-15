use crate::models::{CreateTodoInput, DueTag, PriorityTag, ReminderConfig, Todo};
use chrono::{Duration, Utc};
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
          completed_at TEXT NULL,
          due_tag TEXT NULL,
          priority_tag TEXT NULL,
          reminder_json TEXT NULL
        );
        "
    )
    .map_err(|err| format!("Database migration failed: {err}"))?;

    ensure_column(conn, "todos", "due_tag", "TEXT NULL")?;
    ensure_column(conn, "todos", "priority_tag", "TEXT NULL")?;
    ensure_column(conn, "todos", "completed_at", "TEXT NULL")?;
    Ok(())
}

fn ensure_column(conn: &Connection, table: &str, column: &str, definition: &str) -> Result<(), String> {
    let mut stmt = conn
        .prepare(&format!("PRAGMA table_info({table})"))
        .map_err(|err| format!("Failed to prepare schema query: {err}"))?;

    let columns = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|err| format!("Failed to query schema columns: {err}"))?;

    let mut exists = false;
    for col in columns {
        if col.map_err(|err| format!("Failed to parse schema column: {err}"))? == column {
            exists = true;
            break;
        }
    }

    if !exists {
        conn.execute(
            &format!("ALTER TABLE {table} ADD COLUMN {column} {definition}"),
            [],
        )
        .map_err(|err| format!("Failed to add column {column}: {err}"))?;
    }

    Ok(())
}

pub fn list_todos(conn: &Connection) -> Result<Vec<Todo>, String> {
    let mut stmt = conn
        .prepare(
            "
            SELECT id, title, completed, created_at, updated_at, due_at, completed_at, due_tag, priority_tag, reminder_json
            FROM todos
            ORDER BY completed ASC, created_at DESC
            "
        )
        .map_err(|err| format!("Failed to prepare list query: {err}"))?;

    let rows = stmt
        .query_map([], |row| {
            let due_tag_raw: Option<String> = row.get(7)?;
            let priority_tag_raw: Option<String> = row.get(8)?;
            let reminder_json: Option<String> = row.get(9)?;
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
                completed_at: row.get(6)?,
                due_tag: due_tag_raw.as_deref().and_then(DueTag::from_db_value),
                priority_tag: priority_tag_raw.as_deref().and_then(PriorityTag::from_db_value),
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
    let due_at = input
        .due_tag
        .as_ref()
        .map(|tag| compute_due_at(tag).to_rfc3339());
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title: input.title.trim().to_string(),
        completed: false,
        created_at: now.clone(),
        updated_at: now,
        due_at,
        completed_at: None,
        due_tag: input.due_tag.clone(),
        priority_tag: input.priority_tag.clone(),
        reminder: None,
    };

    conn.execute(
        "
        INSERT INTO todos (id, title, completed, created_at, updated_at, due_at, completed_at, due_tag, priority_tag, reminder_json)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        ",
        params![
            todo.id,
            todo.title,
            if todo.completed { 1_i64 } else { 0_i64 },
            todo.created_at,
            todo.updated_at,
            todo.due_at,
            Option::<String>::None,
            todo.due_tag.as_ref().map(DueTag::as_db_value),
            todo.priority_tag.as_ref().map(PriorityTag::as_db_value),
            Option::<String>::None
        ]
    )
    .map_err(|err| format!("Failed to insert todo: {err}"))?;

    Ok(todo)
}

pub fn set_completed(conn: &Connection, id: &str, completed: bool) -> Result<Todo, String> {
    let now = Utc::now().to_rfc3339();
    let completed_at = if completed { Some(now.clone()) } else { None };
    conn.execute(
        "UPDATE todos SET completed = ?1, updated_at = ?2, completed_at = ?3 WHERE id = ?4",
        params![if completed { 1_i64 } else { 0_i64 }, now, completed_at, id]
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
            SELECT id, title, completed, created_at, updated_at, due_at, completed_at, due_tag, priority_tag, reminder_json
            FROM todos
            WHERE id = ?1
            "
        )
        .map_err(|err| format!("Failed to prepare get query: {err}"))?;

    stmt.query_row([id], |row| {
        let due_tag_raw: Option<String> = row.get(7)?;
        let priority_tag_raw: Option<String> = row.get(8)?;
        let reminder_json: Option<String> = row.get(9)?;
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
            completed_at: row.get(6)?,
            due_tag: due_tag_raw.as_deref().and_then(DueTag::from_db_value),
            priority_tag: priority_tag_raw.as_deref().and_then(PriorityTag::from_db_value),
            reminder,
        })
    })
    .map_err(|err| format!("Failed to fetch updated todo: {err}"))
}

fn compute_due_at(tag: &DueTag) -> chrono::DateTime<chrono::Utc> {
    let now = Utc::now();
    match tag {
        DueTag::Today => now + Duration::hours(24),
        DueTag::Within5Hours => now + Duration::hours(5),
        DueTag::ThreeDays => now + Duration::days(3),
        DueTag::ThisWeek => now + Duration::days(7),
        DueTag::ThisMonth => now + Duration::days(30),
        DueTag::LongTerm => now + Duration::days(90),
    }
}

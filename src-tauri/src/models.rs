use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderConfig {
    pub enabled: bool,
    pub trigger_at: Option<String>,
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
    pub due_at: Option<String>,
    pub reminder: Option<ReminderConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoInput {
    pub title: String,
    pub due_at: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DueTag {
    Today,
    Within5Hours,
    ThreeDays,
    ThisWeek,
    ThisMonth,
    LongTerm,
}

impl DueTag {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::Today => "today",
            Self::Within5Hours => "within5Hours",
            Self::ThreeDays => "threeDays",
            Self::ThisWeek => "thisWeek",
            Self::ThisMonth => "thisMonth",
            Self::LongTerm => "longTerm",
        }
    }

    pub fn from_db_value(raw: &str) -> Option<Self> {
        match raw {
            "today" => Some(Self::Today),
            "within5Hours" => Some(Self::Within5Hours),
            "threeDays" => Some(Self::ThreeDays),
            "thisWeek" => Some(Self::ThisWeek),
            "thisMonth" => Some(Self::ThisMonth),
            "longTerm" => Some(Self::LongTerm),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PriorityTag {
    Low,
    Medium,
    High,
}

impl PriorityTag {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    pub fn from_db_value(raw: &str) -> Option<Self> {
        match raw {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }
}

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
    pub due_tag: Option<DueTag>,
    pub priority_tag: Option<PriorityTag>,
    pub reminder: Option<ReminderConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoInput {
    pub title: String,
    pub due_tag: Option<DueTag>,
    pub priority_tag: Option<PriorityTag>,
}

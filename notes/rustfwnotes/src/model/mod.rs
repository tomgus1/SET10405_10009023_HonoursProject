use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

impl Note {
    pub fn new(id: u64, title: impl Into<String>, content: impl Into<String>, created_at: NaiveDateTime) -> Self {
        Self {
            id,
            title: title.into(),
            content: content.into(),
            created_at,
        }
    }

    pub fn with_id(self, new_id: u64) -> Self {
        Self { id: new_id, ..self }
    }
}

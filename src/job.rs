
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Job {
    pub id: String,
    pub command: String,
    pub status: String,
    pub result: Option<String>,
    pub retries: i32,
}

impl Job {
    pub fn new(command: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            command,
            status: "queued".into(),
            result: None,
            retries: 0,
        }
    }
}


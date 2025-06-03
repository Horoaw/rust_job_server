use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub command: String,
    pub status: String,
    pub retries: i32,
    pub result: Option<String>,
}


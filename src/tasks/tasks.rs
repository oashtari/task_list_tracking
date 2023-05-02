use chrono::{DateTime, NaiveDateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub user_id: i32,
    pub task_name: String,
    pub repeat: bool,
    pub created_at: DateTime<Local>,
    pub due_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
}

use chrono::Local;

impl Task {
    pub fn new(user_id: i32, task_name: String, repeat: bool, due_at: DateTime<Local>) -> Self {
        Task {
            user_id,
            task_name,
            repeat,
            created_at: Local::now(),
            due_at,
            completed_at: None,
        }
    }
}

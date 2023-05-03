use chrono::{DateTime, NaiveDateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
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

    // let task = Task::new(1, 123, "Buy groceries".to_string(), false, Local::today().and_hms(18, 0, 0));

    pub fn complete(&mut self) {
        self.completed_at = Some(Local::now());
    }

    // task.complete();

}

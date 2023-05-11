use chrono::{DateTime, NaiveDateTime, Local};
// use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::Queryable;

// #[derive(Queryable)] will generate all of the code needed to load a Post struct from a SQL query.
// Using #[derive(Queryable)] assumes that the order of fields on the Post struct 
// matches the columns in the posts table, so make sure to define them in the order seen in the schema.rs file.

// #[derive(Queryable)]
#[derive(Debug, Queryable)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub task_name: String,
    pub task_description: String,
    pub repeat: bool,
    pub created_at: DateTime<Local>,
    pub due_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
    pub completed: bool,
}


impl Task {
    pub fn new(id: i32, user_id: i32, task_name: String,task_description: String, repeat: bool, due_at: DateTime<Local>) -> Self {
        Task {
            id, 
            user_id,
            task_name,
            task_description,
            repeat,
            created_at: Local::now(),
            due_at,
            completed_at: None,
            completed: false,
        }
    }

    // let task = Task::new(1, 123, "Buy groceries".to_string(), false, Local::today().and_hms(18, 0, 0));

    pub fn complete(&mut self) {
        self.completed_at = Some(Local::now());
    }

    // task.complete();

}

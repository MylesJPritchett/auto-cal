use crate::prelude::*;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub estimated_time: u32,
    pub due_date: NaiveDate,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    UnStarted,
    InProgress,
    Completed,
    OnHold,
    Deleted,
}

pub fn schedule_tasks(tasks: &mut [Task]) {
    tasks.sort_by_key(|task| task.due_date);
}

pub fn create_task(name: String, estimated_time: u32, due_date: NaiveDate) -> Result<Task> {
    let task = Task {
        name,
        estimated_time,
        due_date,
        status: Status::UnStarted,
    };

    Ok(task)
}

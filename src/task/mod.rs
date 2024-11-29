use crate::prelude::*;
use core::panic;
use std::str::FromStr;

pub mod create;
pub mod display;
pub mod edit;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub time_remaining: u32,
    pub elapsed_time: u32,
    pub due_date: NaiveDate,
    pub status: Status,
    pub created_date: DateTime<Utc>,
    pub priority_level: Priority,
    pub minimum_chunk_size: Option<u32>,
    pub work_intervals: Vec<(DateTime<Utc>, Option<DateTime<Utc>>)>,
}

pub struct TaskEditPayload {
    pub name: Option<String>,
    pub time_remaining: Option<u32>,
    pub due_date: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub minimum_chunk_size: Option<u32>,
    pub elapsed_time: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    UnStarted,
    InProgress,
    Completed,
    OnHold,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Priority {
    Urgent = 1,
    High = 2,
    Medium = 3,
    Low = 4,
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "unstarted" => Ok(Status::UnStarted),
            "inprogress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            "onhold" => Ok(Status::OnHold),
            "deleted" => Ok(Status::Deleted),
            _ => Err(Error::Generic(format!("Invalid status: {}", s))),
        }
    }
}

impl Status {
    pub fn from_option(s: Option<String>) -> Result<Option<Status>> {
        match s {
            Some(status_str) => Status::from_str(&status_str).map(Some),
            None => Ok(None),
        }
    }
}

impl Priority {
    fn from_input(input: Option<String>) -> Self {
        match input
            .and_then(|s| s.parse::<u8>().ok()) // Parse input to `u8` if provided
        {
            Some(1) => Priority::Urgent,
            Some(2) => Priority::High,
            Some(3) => Priority::Medium,
            Some(4) => Priority::Low,
            _ => Priority::Medium, // Default to Medium
        }
    }
}

pub fn parse_priority(input: Option<String>) -> Option<Priority> {
    input.and_then(|s| match s.parse::<u8>() {
        Ok(1) => Some(Priority::Urgent),
        Ok(2) => Some(Priority::High),
        Ok(3) => Some(Priority::Medium),
        Ok(4) => Some(Priority::Low),
        _ => None, // Invalid input
    })
}

pub fn schedule_tasks(tasks: &mut [Task]) {
    tasks.sort_by(|a, b| {
        // First compare by priority level (ascending)
        a.priority_level
            .cmp(&b.priority_level)
            // Then compare by due date if priority levels are equal
            .then_with(|| a.due_date.cmp(&b.due_date))
    });
}

pub fn get_task(tasks: &[Task], search_string: &str) -> Option<Task> {
    let mut matching_tasks = tasks
        .iter()
        .filter(|task| task.id.to_string().starts_with(search_string))
        .collect::<Vec<_>>();

    if matching_tasks.len() == 1 {
        Some(matching_tasks[0].clone())
    } else {
        None
    }
}

pub fn chunks_remaining(task: &Task) -> u32 {
    task.minimum_chunk_size
        .map_or(0, |chunk_size| task.time_remaining / chunk_size)
        + 1
}

impl Task {
    pub fn start_work(&mut self) {
        if self.status == Status::InProgress {
            panic!("Task is already in progress");
        }
        self.status = Status::InProgress;
        self.work_intervals.push((Utc::now(), None));
    }
    pub fn stop_work(&mut self) {
        if self.status != Status::InProgress {
            panic!("Task is not currently in progress");
        }
        self.status = Status::OnHold;

        if let Some((start, end_time)) = self.work_intervals.last_mut() {
            let now = Utc::now();
            *end_time = Some(now);

            let elapsed_minutes = (now - *start).num_minutes() as u32;

            self.elapsed_time += elapsed_minutes;
            self.time_remaining = self.time_remaining.saturating_sub(elapsed_minutes);
        }
    }

    pub fn is_complete(&self) -> bool {
        self.time_remaining == 0
    }
}

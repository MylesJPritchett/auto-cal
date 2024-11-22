use crate::prelude::*;
use chrono::{NaiveDate, Utc};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub estimated_time: u32,
    pub due_date: NaiveDate,
    pub status: Status,
    pub created_date: DateTime<Utc>,
    pub priority_level: Priority,
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

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // prints the variant as a string
    }
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

// Implement Display for Task
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nTask: {}\nEstimated Time: {} minutes\nDue Date: {}\nStatus: {}\nPriority: {}\n",
            self.id, self.name, self.estimated_time, self.due_date, self.status, self.priority_level
        )
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // prints the variant as a string
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

pub fn update_status(task: &mut Task, status: Status) {
    task.status = status;
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

pub fn update_task_in_list(tasks: &mut [Task], updated_task: Task) -> Result<()> {
    if let Some(index) = tasks.iter().position(|task| task.id == updated_task.id) {
        tasks[index] = updated_task;
        Ok(())
    } else {
        Err(Error::Generic("Task not found in list".to_string()))
    }
}

pub fn add_task(
    name: String,
    estimated_time: u32,
    due_date: NaiveDate,
    priority_level: Priority,
    file_path: &str,
) -> Result<()> {
    let new_task = create_task(name, estimated_time, due_date, priority_level)?;

    let mut tasks = read_tasks(file_path)?;

    // Append the new task
    tasks.push(new_task.clone());

    schedule_tasks(&mut tasks);

    write_tasks_to_yaml(&mut tasks, file_path);

    Ok(())
}

pub fn read_tasks(file_path: &str) -> Result<Vec<Task>> {
    // Open the file for reading
    let mut file = File::open(file_path).map_err(|e| {
        Error::IO(e) // Automatically converts std::io::Error to Error::IO
    })?;

    let mut contents = std::fs::read_to_string(file_path).unwrap_or_else(|_| String::new()); // Default to empty string if file doesn't exist

    // Deserialize the YAML string into a Vec<Task>
    let tasks: Vec<Task> = serde_yaml::from_str(&contents).map_err(|e| {
        Error::Generic(format!(
            "Failed to deserialize tasks from YAML. Error: {}",
            e
        ))
    })?;

    Ok(tasks)
}

pub fn list_non_complete_tasks(tasks: &[Task]) -> Result<()> {
    tasks
        .iter()
        .filter(|task| task.status != Status::Completed)
        .for_each(|task| println!("{}", task));
    Ok(())
}

pub fn list_all_tasks(tasks: &mut Vec<Task>) -> Result<()> {
    for task in tasks {
        println!("{}", task);
    }
    Ok(())
}

pub fn create_task(
    name: String,
    estimated_time: u32,
    due_date: NaiveDate,
    priority_level: Priority,
) -> Result<Task> {
    let current_date_time = Utc::now();
    let task = Task {
        id: Uuid::new_v4(),
        name,
        estimated_time,
        due_date,
        status: Status::UnStarted,
        created_date: current_date_time,
        priority_level,
    };

    Ok(task)
}

pub fn write_tasks_to_yaml(tasks: &mut Vec<Task>, file_path: &str) -> Result<()> {
    let yaml_string = serde_yaml::to_string(&tasks)
        .map_err(|e| Error::Generic(format!("Failed to serialize tasks to YAML. Error: {}", e)))?;

    // Write the updated YAML content to the file
    std::fs::write(file_path, yaml_string).map_err(Error::IO)?;

    Ok(())
}

pub fn append_task_to_yaml(task: &Task, file_path: &str) -> Result<()> {
    let mut tasks = read_tasks(file_path)?;

    // Append the new task
    tasks.push(task.clone());

    write_tasks_to_yaml(&mut tasks, file_path)
}

pub fn edit_task(
    old_task: &Task,
    name: Option<String>,
    estimated_time: Option<u32>,
    due_date: Option<NaiveDate>,
    status: Option<Status>,
    priority_level: Option<Priority>,
) -> Result<Task> {
    let task = Task {
        id: old_task.id,
        name: name.unwrap_or(old_task.name.clone()),
        estimated_time: estimated_time.unwrap_or(old_task.estimated_time),
        due_date: due_date.unwrap_or(old_task.due_date),
        status: status.unwrap_or(old_task.status.clone()),
        created_date: old_task.created_date,
        priority_level: priority_level.unwrap_or(old_task.priority_level.clone()),
    };

    Ok(task)
}

use crate::prelude::*;
use chrono::{NaiveDate, Utc};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub estimated_time: u32,
    pub due_date: NaiveDate,
    pub status: Status,
    pub created_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    UnStarted,
    InProgress,
    Completed,
    OnHold,
    Deleted,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // prints the variant as a string
    }
}

// Implement Display for Task
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nTask: {}\nEstimated Time: {} minutes\nDue Date: {}\nStatus: {}\n",
            self.id, self.name, self.estimated_time, self.due_date, self.status
        )
    }
}

pub fn schedule_tasks(tasks: &mut [Task]) {
    tasks.sort_by_key(|task| task.due_date);
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
    file_path: &str,
) -> Result<()> {
    let new_task = create_task(name, estimated_time, due_date)?;

    let mut tasks = read_tasks(file_path)?;

    // Append the new task
    tasks.push(new_task.clone());

    calculate_task_order(&mut tasks);

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

pub fn create_task(name: String, estimated_time: u32, due_date: NaiveDate) -> Result<Task> {
    let current_date_time = Utc::now();
    let task = Task {
        id: Uuid::new_v4(),
        name,
        estimated_time,
        due_date,
        status: Status::UnStarted,
        created_date: current_date_time,
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

pub fn calculate_task_order(tasks: &mut [Task]) {
    tasks.sort_by_key(|task| task.due_date);
}

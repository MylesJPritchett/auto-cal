use crate::prelude::*;

pub fn add_task(
    name: String,
    estimated_time: u32,
    due_date: NaiveDate,
    priority_level: Priority,
    minimum_chunk_size: Option<u32>,
    file_path: &str,
) -> Result<()> {
    let new_task = create_task(
        name,
        estimated_time,
        due_date,
        priority_level,
        minimum_chunk_size,
    )?;

    let mut tasks = read_tasks(file_path)?;

    // Append the new task
    tasks.push(new_task.clone());

    schedule_tasks(&mut tasks);

    write_tasks_to_yaml(&mut tasks, file_path);

    Ok(())
}

pub fn create_task(
    name: String,
    estimated_time: u32,
    due_date: NaiveDate,
    priority_level: Priority,
    minimum_chunk_size: Option<u32>,
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
        minimum_chunk_size,
    };

    Ok(task)
}

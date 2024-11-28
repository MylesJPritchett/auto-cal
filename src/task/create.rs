use crate::prelude::*;

pub fn add_task(
    name: String,
    time_remaining: u32,
    due_date: NaiveDate,
    priority_level: Priority,
    minimum_chunk_size: Option<u32>,
    file_path: &str,
) -> Result<()> {
    let new_task = create_task(
        name,
        time_remaining,
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
    time_remaining: u32,
    due_date: NaiveDate,
    priority_level: Priority,
    minimum_chunk_size: Option<u32>,
) -> Result<Task> {
    let current_date_time = Utc::now();
    let task = Task {
        id: Uuid::new_v4(),
        name,
        time_remaining,
        due_date,
        status: Status::UnStarted,
        created_date: current_date_time,
        priority_level,
        minimum_chunk_size,
        elapsed_time: 0,
        work_intervals: Vec::new(),
    };

    Ok(task)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task_valid() {
        let task_name = "Test Task".to_string();
        let time_remaining = 3;
        let due_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let priority_level = Priority::High;
        let minimum_chunk_size = None;

        let task = create_task(
            task_name.clone(),
            time_remaining,
            due_date,
            priority_level,
            minimum_chunk_size,
        )
        .unwrap();

        assert_eq!(task.name, task_name);
        assert_eq!(task.time_remaining, time_remaining);
        assert_eq!(task.due_date, due_date);
        assert_eq!(task.priority_level, Priority::High);
        assert_eq!(task.status, Status::UnStarted);
    }
}

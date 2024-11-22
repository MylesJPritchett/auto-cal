use crate::prelude::*;

pub fn update_status(task: &mut Task, status: Status) {
    task.status = status;
}

pub fn update_task_in_list(tasks: &mut [Task], updated_task: Task) -> Result<()> {
    if let Some(index) = tasks.iter().position(|task| task.id == updated_task.id) {
        tasks[index] = updated_task;
        Ok(())
    } else {
        Err(Error::Generic("Task not found in list".to_string()))
    }
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

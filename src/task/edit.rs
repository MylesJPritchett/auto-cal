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

pub fn edit_task(old_task: &Task, payload: &TaskEditPayload) -> Result<Task> {
    let due_date = payload
        .due_date
        .as_ref()
        .and_then(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok());

    let status = Status::from_option(payload.status.clone())?;
    let priority = parse_priority(payload.priority.clone());

    let task = Task {
        id: old_task.id,
        name: payload.name.clone().unwrap_or(old_task.name.clone()),
        time_remaining: payload.time_remaining.unwrap_or(old_task.time_remaining),
        due_date: due_date.unwrap_or(old_task.due_date),
        status: status.unwrap_or(old_task.status.clone()),
        created_date: old_task.created_date,
        priority_level: priority.unwrap_or(old_task.priority_level.clone()),
        minimum_chunk_size: payload.minimum_chunk_size.or(old_task.minimum_chunk_size),
        work_intervals: old_task.work_intervals.clone(),
        elapsed_time: payload.elapsed_time.unwrap_or(old_task.elapsed_time),
    };

    Ok(task)
}

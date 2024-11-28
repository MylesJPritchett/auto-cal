use crate::prelude::*;

pub fn handle_create(
    name: String,
    time: u32,
    due_date: String,
    priority: Option<String>,
    minimum_chunk_size: Option<u32>,
) -> Result<()> {
    let due_date = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d").map_err(|_| {
        Error::Generic(format!(
            "Could not parse the due date: {}. Expected format: YYYY-MM-DD",
            due_date
        ))
    })?;

    let priority = parse_priority(priority).unwrap_or(Priority::Medium);

    add_task(
        name,
        time,
        due_date,
        priority,
        minimum_chunk_size,
        "tasks.yaml",
    )?;
    println!("Created task");
    Ok(())
}

pub fn handle_list(all: bool) -> Result<()> {
    // Logic to list tasks (replace with your actual implementation)
    if all {
        println!("Listing all tasks...");
        list_tasks(&mut read_tasks("tasks.yaml")?)?;
    } else {
        print!("Listing all tasks that are not complete...");
        list_tasks(&mut filter_out_completed_tasks(&read_tasks("tasks.yaml")?)?);
    }

    Ok(())
}

pub fn handle_start(id: String) -> Result<()> {
    println!("Searching for Task to Start");
    let mut tasks = read_tasks("tasks.yaml")?;
    match get_task(&tasks, &id) {
        Some(mut task) => {
            update_status(&mut task, Status::InProgress);
            println!("Starting Task: {}", task);
            update_task_in_list(&mut tasks, task);
            write_tasks_to_yaml(&mut tasks, "tasks.yaml");
        }
        None => println!("No Single Task Found"),
    }
    Ok(())
}

pub fn handle_stop(id: String) -> Result<()> {
    println!("Searching for Task to Stop");
    let mut tasks = read_tasks("tasks.yaml")?;
    match get_task(&tasks, &id) {
        Some(mut task) => {
            update_status(&mut task, Status::OnHold);
            println!("Stopping Task: {}", task);
            update_task_in_list(&mut tasks, task);
            write_tasks_to_yaml(&mut tasks, "tasks.yaml");
        }
        None => println!("No Single Task Found"),
    }

    Ok(())
}

pub fn handle_complete(id: String) -> Result<()> {
    println!("Searching for Task to Complete");
    let mut tasks = read_tasks("tasks.yaml")?;
    match get_task(&tasks, &id) {
        Some(mut task) => {
            update_status(&mut task, Status::Completed);
            println!("Completed Task: {}", task);
            update_task_in_list(&mut tasks, task);
            write_tasks_to_yaml(&mut tasks, "tasks.yaml");
        }
        None => println!("No Single Task Found"),
    }
    Ok(())
}

pub fn handle_edit(
    id: String,
    name: Option<String>,
    time: Option<u32>,
    due_date: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    minimum_chunk_size: Option<u32>,
) -> Result<()> {
    println!("Searching for Task to Edit");
    let mut tasks = read_tasks("tasks.yaml")?;
    match get_task(&tasks, &id) {
        Some(mut task) => {
            let due_date: Option<NaiveDate> = due_date.and_then(|s| {
                NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                    .map_err(|_| {
                        Error::Generic(format!(
                            "Invalid due date format: '{}'. Expected format: YYYY-MM-DD",
                            s
                        ))
                    })
                    .ok() // If parse fails, returns None
            });

            let priority = parse_priority(priority);
            let status = Status::from_option(status).map_err(|e| {
                // Handle error if parsing status failed
                Error::Generic(format!("Failed to parse status: {}", e))
            })?;

            let task = edit_task(
                &task,
                name,
                time,
                due_date,
                status,
                priority,
                minimum_chunk_size,
            )?;
            println!("Edited Task: {}", task);
            update_task_in_list(&mut tasks, task);
            write_tasks_to_yaml(&mut tasks, "tasks.yaml");
        }
        None => println!("No Single Task Found"),
    }
    Ok(())
}

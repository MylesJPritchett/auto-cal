use crate::prelude::*;

pub fn handle_create(
    name: String,
    time: u32,
    due_date: String,
    priority: Option<String>,
    file_path: &str,
) -> Result<()> {
    let due_date = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d").map_err(|_| {
        Error::Generic(format!(
            "Could not parse the due date: {}. Expected format: YYYY-MM-DD",
            due_date
        ))
    })?;

    let priority = parse_priority(priority).unwrap_or(Priority::Medium);

    add_task(name, time, due_date, priority, file_path)?;
    println!("Created task");
    Ok(())
}

pub fn handle_list(all: bool, file_path: &str) -> Result<()> {
    // Logic to list tasks (replace with your actual implementation)
    if all {
        println!("Listing all tasks...");
        list_tasks(&mut read_tasks(file_path)?)?;
    } else {
        print!("Listing all tasks that are not complete...");
        list_tasks(&mut filter_out_completed_tasks(&read_tasks(file_path)?)?);
    }

    Ok(())
}

pub fn handle_start(id: String, file_path: &str) -> Result<()> {
    println!("Searching for Task to Start");
    let mut tasks = read_tasks(file_path)?;
    match get_task(&tasks, &id) {
        Some(mut task) => {
            update_status(&mut task, Status::InProgress);
            println!("Starting Task: {}", task);
            update_task_in_list(&mut tasks, task);
            write_tasks_to_yaml(&mut tasks, file_path);
        }
        None => println!("No Single Task Found"),
    }
    Ok(())
}

pub fn handle_stop(id: String, file_path: &str) -> Result<()> {
    println!("Searching for Task to Stop");
    let mut tasks = read_tasks(file_path)?;
    match get_task(&tasks, &id) {
        Some(mut task) => {
            update_status(&mut task, Status::OnHold);
            println!("Stopping Task: {}", task);
            update_task_in_list(&mut tasks, task);
            write_tasks_to_yaml(&mut tasks, file_path);
        }
        None => println!("No Single Task Found"),
    }

    Ok(())
}

pub fn handle_complete(id: String, file_path: &str) -> Result<()> {
    println!("Searching for Task to Complete");
    let mut tasks = read_tasks(file_path)?;
    match get_task(&tasks, &id) {
        Some(mut task) => {
            update_status(&mut task, Status::Completed);
            println!("Completed Task: {}", task);
            update_task_in_list(&mut tasks, task);
            write_tasks_to_yaml(&mut tasks, file_path);
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
    file_path: &str,
) -> Result<()> {
    println!("Searching for Task to Edit");
    let mut tasks = read_tasks(file_path)?;
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

            let task = edit_task(&task, name, time, due_date, status, priority)?;
            println!("Edited Task: {}", task);
            update_task_in_list(&mut tasks, task);
            write_tasks_to_yaml(&mut tasks, file_path);
        }
        None => println!("No Single Task Found"),
    }
    Ok(())
}

// TODO: finish these tests to make them all work as intended
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use tempfile::NamedTempFile;

    fn create_temp_file_with_tasks() -> String {
        let temp_file = "test_tasks.yaml";
        let yaml_content = r#"
        - id: "5275239b-bc94-467a-b34c-141498417c7d"
          name: "high priority"
          estimated_time: 23
          due_date: "2024-11-15"
          status: "UnStarted"
          created_date: "2024-11-22T01:21:52.020546800Z"
          priority_level: "Urgent"
        - id: "62688812-f3bd-418c-a5db-42457d63a3a7"
          name: "high priority"
          estimated_time: 23
          due_date: "2024-11-15"
          status: "Completed"
          created_date: "2024-11-22T01:13:39.845179100Z"
          priority_level: "High"
        - id: "49b03535-5595-4f96-80de-dc2b8a325add"
          name: "low priority"
          estimated_time: 23
          due_date: "2024-11-26"
          status: "UnStarted"
          created_date: "2024-11-22T01:27:48.823644200Z"
          priority_level: "High"
        "#;

        fs::write(temp_file, yaml_content).expect("Failed to write YAML file");
        temp_file.to_string()
    }

    fn clean_temp_file() {
        let temp_file = "test_tasks.yaml";
        fs::remove_file(temp_file).expect("Failed to remove test file");
    }

    #[test]
    fn creates_task_successfully_with_fake_file() {
        let temp_file_path = create_temp_file_with_tasks();

        let task_name = "Test Task";
        let task_time = 5;
        let task_due_date = "2024-12-01".to_string();
        let task_priority = Some("High".to_string());

        let result = handle_create(
            task_name.to_string(),
            task_time,
            task_due_date,
            task_priority,
            &temp_file_path,
        );

        match result {
            Ok(_) => {
                let tasks =
                    read_tasks(&temp_file_path).expect("Failed to read tasks from temp file");
                assert!(tasks.iter().any(|task| task.name == task_name));
            }
            Err(e) => panic!("Test failed with error: {}", e),
        }
        clean_temp_file();
    }

    #[test]
    fn fails_on_invalid_due_date_format() {
        let task_name = "Invalid Task";
        let task_time = 5;
        let invalid_due_date = "2024-31-12".to_string();
        let task_priority = Some("High".to_string());
        let temp_file_path = create_temp_file_with_tasks();

        let result = handle_create(
            task_name.to_string(),
            task_time,
            invalid_due_date,
            task_priority,
            &temp_file_path,
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Could not parse the due date"));
    }

    #[test]
    fn lists_all_tasks_from_fake_file() {
        let temp_file_path = create_temp_file_with_tasks();

        let result = handle_list(true, &temp_file_path);
        assert!(result.is_ok());

        let tasks = read_tasks(&temp_file_path).expect("Failed to read tasks from temp file");
        assert_eq!(tasks.len(), 3);

        clean_temp_file();
    }

    #[test]
    fn lists_non_completed_tasks() {
        let temp_file_path = create_temp_file_with_tasks();

        let result = handle_list(false, &temp_file_path);
        assert!(result.is_ok());

        let tasks = read_tasks(&temp_file_path).expect("Failed to read tasks from temp file");
        assert_eq!(tasks.len(), 2); // Assuming only 2 tasks are non-completed
        clean_temp_file();
    }

    #[test]
    fn starts_task_successfully() {
        let temp_file_path = create_temp_file_with_tasks();

        let mut tasks = read_tasks(&temp_file_path).expect("Failed to read tasks");
        let task_id = tasks[0].id.to_string(); // Get the id of the first task

        let result = handle_start(task_id, &temp_file_path);
        assert!(result.is_ok());

        let tasks = read_tasks(&temp_file_path).expect("Failed to read tasks from temp file");
        assert!(tasks.iter().any(|task| task.status == Status::InProgress));
        clean_temp_file();
    }

    #[test]
    fn fails_when_task_not_found_on_start() {
        let task_id = "invalid_task_id".to_string();
        let temp_file_path = create_temp_file_with_tasks();

        let result = handle_start(task_id, &temp_file_path);
        assert!(result.is_ok());
    }

    #[test]
    fn stops_task_successfully() {
        let temp_file_path = create_temp_file_with_tasks();

        let mut tasks = read_tasks(&temp_file_path).expect("Failed to read tasks");
        let task_id = tasks[0].id.to_string();

        let result = handle_stop(task_id, &temp_file_path);
        assert!(result.is_ok());

        let tasks = read_tasks(&temp_file_path).expect("Failed to read tasks from temp file");
        assert!(tasks.iter().any(|task| task.status == Status::OnHold));
        clean_temp_file();
    }

    #[test]
    fn completes_task_successfully() {
        let temp_file_path = create_temp_file_with_tasks();

        let mut tasks = read_tasks(&temp_file_path).expect("Failed to read tasks");
        let task_id = tasks[0].id.to_string();

        let result = handle_complete(task_id, &temp_file_path);
        assert!(result.is_ok());

        let tasks = read_tasks(&temp_file_path).expect("Failed to read tasks from temp file");
        assert!(tasks.iter().any(|task| task.status == Status::Completed));
        clean_temp_file();
    }

    #[test]
    fn edits_task_successfully() {
        let temp_file_path = create_temp_file_with_tasks();

        let mut tasks = read_tasks(&temp_file_path).expect("Failed to read tasks");
        let task_id = tasks[0].id.to_string();

        let result = handle_edit(
            task_id,
            Some("Updated Task".to_string()),
            Some(10),
            Some("2024-12-15".to_string()),
            Some("Completed".to_string()),
            Some("Low".to_string()),
            &temp_file_path,
        );
        assert!(result.is_ok());

        let tasks = read_tasks(&temp_file_path).expect("Failed to read tasks from temp file");
        let edited_task = tasks.iter().find(|task| task.name == "Updated Task");
        assert!(edited_task.is_some());
        assert_eq!(edited_task.unwrap().priority_level, Priority::Low);
        clean_temp_file();
    }

    #[test]
    fn fails_when_task_not_found_on_edit() {
        let task_id = "invalid_task_id".to_string();
        let temp_file_path = create_temp_file_with_tasks();

        let result = handle_edit(task_id, None, None, None, None, None, &temp_file_path);
        assert!(result.is_ok());
    }
}

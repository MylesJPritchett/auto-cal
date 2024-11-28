use crate::io::*;
use crate::prelude::*;

pub fn write_tasks_to_yaml(tasks: &mut Vec<Task>, file_path: &str) -> Result<()> {
    let yaml_string = serde_yaml::to_string(&tasks)
        .map_err(|e| Error::Generic(format!("Failed to serialize tasks to YAML. Error: {}", e)))?;

    // Write the updated YAML content to the file
    std::fs::write(file_path, yaml_string).map_err(Error::IO)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn save_tasks_to_yaml_file() {
        // Temporary file path for testing
        let temp_file = "test_output.yaml";

        // Sample tasks
        let mut tasks = vec![
            Task {
                id: uuid::Uuid::new_v4(),
                name: "Sample Task 1".to_string(),
                estimated_time: 5,
                due_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 30).unwrap(),
                status: Status::UnStarted,
                created_date: chrono::Utc::now(),
                priority_level: Priority::High,
                minimum_chunk_size: Some(30),
            },
            Task {
                id: uuid::Uuid::new_v4(),
                name: "Sample Task 2".to_string(),
                estimated_time: 8,
                due_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
                status: Status::InProgress,
                created_date: chrono::Utc::now(),
                priority_level: Priority::Urgent,
                minimum_chunk_size: None,
            },
        ];

        // Call the function
        let result = write_tasks_to_yaml(&mut tasks, temp_file);
        assert!(result.is_ok());

        // Read the written file and verify content
        let written_content = fs::read_to_string(temp_file).expect("Failed to read output file");
        assert!(written_content.contains("Sample Task 1"));
        assert!(written_content.contains("Sample Task 2"));
        assert!(written_content.contains("UnStarted"));
        assert!(written_content.contains("High"));

        // Clean up
        fs::remove_file(temp_file).expect("Failed to delete test file");
    }

    #[test]
    fn saves_empty_tasks_to_yaml_file() {
        let temp_file = "test_empty_output.yaml";
        let mut tasks: Vec<Task> = vec![];

        let result = write_tasks_to_yaml(&mut tasks, temp_file);
        assert!(result.is_ok());

        let written_content =
            std::fs::read_to_string(temp_file).expect("Failed to read output file");
        assert_eq!(written_content.trim(), "[]");

        std::fs::remove_file(temp_file).expect("Failed to delete test file");
    }

    #[test]
    fn returns_error_when_writing_to_invalid_path() {
        let invalid_path = "/invalid_directory/test_output.yaml";
        let mut tasks = vec![Task {
            id: uuid::Uuid::new_v4(),
            name: "Sample Task".to_string(),
            estimated_time: 5,
            due_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 30).unwrap(),
            status: Status::UnStarted,
            created_date: chrono::Utc::now(),
            priority_level: Priority::High,
            minimum_chunk_size: None,
        }];

        let result = write_tasks_to_yaml(&mut tasks, invalid_path);
        assert!(result.is_err());
    }
}

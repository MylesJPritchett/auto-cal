use crate::io::*;
use crate::prelude::*;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn read_tasks_from_yaml_file() {
        // Write the provided YAML to a temporary file
        let temp_file = "test_tasks.yaml";
        let yaml_content = r#"
        - id: "5275239b-bc94-467a-b34c-141498417c7d"
          name: "high priority"
          time_remaining: 23
          due_date: "2024-11-15"
          status: "UnStarted"
          created_date: "2024-11-22T01:21:52.020546800Z"
          priority_level: "Urgent"

        - id: "62688812-f3bd-418c-a5db-42457d63a3a7"
          name: "high priority"
          time_remaining: 23
          due_date: "2024-11-15"
          status: "UnStarted"
          created_date: "2024-11-22T01:13:39.845179100Z"
          priority_level: "High"
        - id: "49b03535-5595-4f96-80de-dc2b8a325add"
          name: "low priority"
          time_remaining: 23
          due_date: "2024-11-26"
          status: "UnStarted"
          created_date: "2024-11-22T01:27:48.823644200Z"
          priority_level: "High"
        "#;

        fs::write(temp_file, yaml_content).expect("Failed to write YAML file");

        // Test reading tasks
        let result = read_tasks(temp_file);
        assert!(result.is_ok());
        let tasks = result.unwrap();

        // Assert number of tasks read
        assert_eq!(tasks.len(), 3);

        // Assert individual task fields
        assert_eq!(tasks[0].name, "high priority");
        assert_eq!(tasks[0].priority_level, Priority::Urgent);
        assert_eq!(
            tasks[2].due_date,
            NaiveDate::from_ymd_opt(2024, 11, 26).unwrap()
        );

        // Clean up
        fs::remove_file(temp_file).expect("Failed to delete test file");
    }
}

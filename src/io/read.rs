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

use crate::io::*;
use crate::prelude::*;

pub fn write_tasks_to_yaml(tasks: &mut Vec<Task>, file_path: &str) -> Result<()> {
    let yaml_string = serde_yaml::to_string(&tasks)
        .map_err(|e| Error::Generic(format!("Failed to serialize tasks to YAML. Error: {}", e)))?;

    // Write the updated YAML content to the file
    std::fs::write(file_path, yaml_string).map_err(Error::IO)?;

    Ok(())
}

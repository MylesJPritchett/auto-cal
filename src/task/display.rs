use crate::prelude::*;

pub fn list_non_complete_tasks(tasks: &[Task]) -> Result<()> {
    tasks
        .iter()
        .filter(|task| task.status != Status::Completed)
        .for_each(|task| println!("{}", task));
    Ok(())
}

pub fn list_all_tasks(tasks: &mut Vec<Task>) -> Result<()> {
    for task in tasks {
        println!("{}", task);
    }
    Ok(())
}

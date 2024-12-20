use crate::prelude::*;

use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nTask: {}\nTime Remaining: {} minutes\nDue Date: {}\nStatus: {}\nPriority: {}\nChunks Remaining: {}",
            self.id, self.name, self.time_remaining, self.due_date, self.status, self.priority_level, chunks_remaining(self)
        )
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // prints the variant as a string
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // prints the variant as a string
    }
}

pub fn filter_out_completed_tasks(tasks: &[Task]) -> Result<Vec<Task>> {
    let filtered_tasks: Vec<Task> = tasks
        .iter()
        .filter(|task| task.status != Status::Completed)
        .cloned() // Clone each `Task` since the input slice borrows them
        .collect();
    Ok(filtered_tasks)
}

pub fn list_tasks(tasks: &[Task], count: Option<u32>) -> Result<()> {
    let num_to_display = count.map(|c| c as usize).unwrap_or(tasks.len());

    for task in tasks.iter().take(num_to_display) {
        println!("{}", task);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_tasks() -> Vec<Task> {
        vec![
            Task {
                id: Uuid::new_v4(),
                name: "Complete Rust project".to_string(),
                time_remaining: 120,
                due_date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
                status: Status::InProgress,
                created_date: Utc::now(),
                priority_level: Priority::High,
                minimum_chunk_size: None,
                elapsed_time: 0,
                work_intervals: vec![(chrono::Utc::now(), None)],
            },
            Task {
                id: Uuid::new_v4(),
                name: "Read documentation".to_string(),
                time_remaining: 60,
                due_date: NaiveDate::from_ymd_opt(2024, 11, 25).unwrap(),
                status: Status::UnStarted,
                created_date: Utc::now(),
                priority_level: Priority::Medium,
                minimum_chunk_size: None,
                elapsed_time: 0,
                work_intervals: vec![(chrono::Utc::now(), None)],
            },
            Task {
                id: Uuid::new_v4(),
                name: "Prepare presentation".to_string(),
                time_remaining: 90,
                due_date: NaiveDate::from_ymd_opt(2024, 11, 29).unwrap(),
                status: Status::Completed,
                created_date: Utc::now(),
                priority_level: Priority::Urgent,
                minimum_chunk_size: Some(30),
                elapsed_time: 0,
                work_intervals: vec![(chrono::Utc::now(), None)],
            },
        ]
    }

    #[test]
    fn filter_completed_out_of_list() {
        let tasks = create_sample_tasks();

        let result = filter_out_completed_tasks(&tasks).expect("Filtering failed");
        assert_eq!(result.len(), 2); // Only 2 tasks are not completed
        assert!(result.iter().all(|task| task.status != Status::Completed));
    }
}

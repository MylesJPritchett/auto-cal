#![allow(unused)] // For beginning only.

use crate::prelude::*;
use clap::{Parser, Subcommand};

mod error;
mod prelude;
mod task;
mod utils;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // Handle "create" command
        Command::Create {
            name,
            time,
            due_date,
            priority,
        } => {
            let due_date = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d").map_err(|_| {
                Error::Generic(format!(
                    "Could not parse the due date: {}. Expected format: YYYY-MM-DD",
                    due_date
                ))
            })?;

            let priority = parse_priority(priority).unwrap_or(Priority::Medium);

            add_task(name, time, due_date, priority, "tasks.yaml")?;
            println!("Created task");
        }

        // Handle "list" command
        Command::List { all } => {
            // Logic to list tasks (replace with your actual implementation)
            if all {
                println!("Listing all tasks...");
                list_all_tasks(&mut read_tasks("tasks.yaml")?)?;
            } else {
                print!("Listing all tasks that are not complete...");
                list_non_complete_tasks(&read_tasks("tasks.yaml")?)?;
            }
        }
        Command::Start { id } => {
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
        }
        Command::Stop { id } => {
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
        }
        Command::Complete { id } => {
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
        }
        Command::Edit {
            id,
            name,
            time,
            due_date,
            status,
            priority,
        } => {
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

                    let task = edit_task(&task, name, time, due_date, status, priority)?;
                    println!("Edited Task: {}", task);
                    update_task_in_list(&mut tasks, task);
                    write_tasks_to_yaml(&mut tasks, "tasks.yaml");
                }
                None => println!("No Single Task Found"),
            }
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "Task Manager")]
#[command(about = "A simple task management CLI application", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Create {
        /// Task name
        #[arg(short, long)]
        name: String,

        /// Task duration in minutes
        #[arg(short, long)]
        time: u32,

        /// Task due date in YYYY-MM-DD format
        #[arg(short, long)]
        due_date: String,

        /// Task Priority from 1 for urgent to 4 for Low
        #[arg(short, long)]
        priority: Option<String>,
    },
    List {
        #[arg(short, long, action)]
        all: bool,
    },
    Start {
        #[arg(short, long)]
        id: String,
    },
    Stop {
        #[arg(short, long)]
        id: String,
    },
    Complete {
        #[arg(short, long)]
        id: String,
    },
    Edit {
        #[arg(short, long)]
        id: String,

        /// Task name
        #[arg(short, long)]
        name: Option<String>,

        /// Task duration in minutes
        #[arg(short, long)]
        time: Option<u32>,

        /// Task due date in YYYY-MM-DD format
        #[arg(short, long)]
        due_date: Option<String>,

        #[arg(short, long)]
        status: Option<String>,

        /// Task Priority from 1 for urgent to 4 for Low
        #[arg(short, long)]
        priority: Option<String>,
    },
}

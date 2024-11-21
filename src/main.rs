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
        } => {
            let due_date = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d").map_err(|_| {
                Error::Generic(format!(
                    "Could not parse the due date: {}. Expected format: YYYY-MM-DD",
                    due_date
                ))
            })?;
            add_task(name, time, due_date, "tasks.yaml")?;
            println!("Created task");
        }

        // Handle "list" command
        Command::List => {
            // Logic to list tasks (replace with your actual implementation)
            println!("Listing all tasks...");
            list_tasks(&mut read_tasks("tasks.yaml")?)?;
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
    },
    List,
    Start {
        #[arg(short, long)]
        id: String,
    },
    Stop {
        #[arg(short, long)]
        id: String,
    },
}

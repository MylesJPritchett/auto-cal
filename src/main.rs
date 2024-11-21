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
            let task = create_task(name, time, due_date)?;
            append_task_to_yaml(&task, "tasks.yaml");
            println!("Created task: {:?}", task);
        }

        // Handle "list" command
        Command::List => {
            // Logic to list tasks (replace with your actual implementation)
            println!("Listing all tasks...");
            list_tasks(&mut read_tasks("tasks.yaml")?)?;
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
}

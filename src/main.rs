#![allow(unused)] // For beginning only.

use crate::prelude::*;
use clap::Parser;

mod error;
mod prelude;
mod task;
mod utils;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Parse the due date into a NaiveDate
    let due_date = NaiveDate::parse_from_str(&cli.due_date, "%Y-%m-%d").map_err(|_| {
        Error::Generic(format!(
            "Could not parse the due date: {}. Expected format: YYYY-MM-DD",
            cli.due_date
        ))
    })?;
    // Create a task
    let task = create_task(cli.name, cli.time, due_date);
    println!("Task: {:?}", task?);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "Task Manager")]
#[command(about = "A simple task management CLI application", version = "1.0")]
struct Cli {
    /// Task name
    #[arg(short, long)]
    name: String,

    /// Task duration in minutes
    #[arg(short, long)]
    time: u32,

    /// Task due date in YYYY-MM-DD format
    #[arg(short, long)]
    due_date: String,
}

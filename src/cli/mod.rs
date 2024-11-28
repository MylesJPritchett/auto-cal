use crate::{Error, Priority, Status, Task};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Task Manager")]
#[command(about = "A simple task management CLI application", version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
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

        #[arg(short, long)]
        chunk_size: Option<u32>,
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

        #[arg(short, long)]
        chunk_size: Option<u32>,
    },
}

impl Cli {
    pub fn parse_cli() -> Self {
        Cli::parse()
    }
}

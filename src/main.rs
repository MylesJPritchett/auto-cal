#![allow(unused)] // For beginning only.

use crate::cli::{Cli, Command};
use crate::command_handlers::*;
use crate::prelude::*;

mod cli;
mod command_handlers;
mod error;
mod io;
mod prelude;
mod task;
mod utils;

fn main() -> Result<()> {
    let cli = Cli::parse_cli(); // Use the parse_cli function
    let file_path = "tasks.yaml";

    match cli.command {
        Command::Create {
            name,
            time,
            due_date,
            priority,
        } => handle_create(name, time, due_date, priority, file_path)?,
        Command::List { all } => handle_list(all, file_path)?,
        Command::Start { id } => handle_start(id, file_path)?,
        Command::Stop { id } => handle_stop(id, file_path)?,
        Command::Complete { id } => handle_complete(id, file_path)?,
        Command::Edit {
            id,
            name,
            time,
            due_date,
            status,
            priority,
        } => handle_edit(id, name, time, due_date, status, priority, file_path)?,
    }
    Ok(())
}

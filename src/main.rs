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

    match cli.command {
        Command::Create {
            name,
            time,
            due_date,
            priority,
            chunk_size,
        } => handle_create(name, time, due_date, priority, chunk_size)?,
        Command::List { all } => handle_list(all)?,
        Command::Start { id } => handle_start(id)?,
        Command::Stop { id } => handle_stop(id)?,
        Command::Complete { id } => handle_complete(id)?,
        Command::Edit {
            id,
            name,
            time,
            due_date,
            status,
            priority,
            chunk_size,
        } => handle_edit(id, name, time, due_date, status, priority, chunk_size)?,
    }
    Ok(())
}

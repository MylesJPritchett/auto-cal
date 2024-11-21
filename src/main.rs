#![allow(unused)] // For beginning only.

use crate::prelude::*;

mod error;
mod prelude;
mod task;
mod utils;

fn main() -> Result<()> {
    let task = create_task(
        String::from("Test Task"),
        120,
        chrono::NaiveDate::from_ymd_opt(2024, 11, 24).unwrap(),
    );

    println!("Task: {:?}", task);
    Ok(())
}

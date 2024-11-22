use crate::prelude::*;

pub mod read;
pub mod write;

pub use std::fs::{File, OpenOptions};
pub use std::io::{Read, Write};

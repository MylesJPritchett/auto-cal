//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple strcut for newtype pattern
pub struct W<T>(pub T);

pub use chrono::{DateTime, NaiveDate, Utc};
pub use serde::{Deserialize, Serialize};
pub use uuid::Uuid;

pub use crate::task::create::*;
pub use crate::task::display::*;
pub use crate::task::edit::*;
pub use crate::task::*;

pub use crate::io::read::*;
pub use crate::io::write::*;

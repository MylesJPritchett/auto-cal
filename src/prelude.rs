//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple strcut for newtype pattern
pub struct W<T>(pub T);

pub use chrono::{DateTime, NaiveDate};
pub use serde::{Deserialize, Serialize};

pub use crate::task::*;

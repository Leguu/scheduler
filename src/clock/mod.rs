//! This module defines all the time-related structures, such as for dates and 24hr times.

use chrono::prelude::Local;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Contains the Date struct and related functions.
mod date;
/// Contains the Day struct and related functions.
mod day;
/// Contains the Time struct and related functions.
mod time;

pub use date::Date;
pub use day::Day;
pub use time::Time;

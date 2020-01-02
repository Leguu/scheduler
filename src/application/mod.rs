use crate::clock::*;
use serde::{Deserialize, Serialize};

// We want all of these to be private, so we're doing `mod _` instead of `pub mod _`
mod application;
mod course;
mod task;

// However, since `Application` is the only thing that's needed, we're making it public here.
pub use application::Application;
// We're making these public so that you can read the documentation!
// These aren't actually meant to be public.
pub use course::Course;
pub use task::Task;

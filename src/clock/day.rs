use super::*;
use chrono::Weekday::*;

// See date.rs to understand #[derive()]
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Day {
	Sunday,
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
}

impl Day {
	/// Gets what day it is today
	/// It's basically a wrapper for an external library, making it nicer for internal use
	pub fn today() -> Self {
		// We're depending on a foreign library for this
		// Because, turns out, it's super difficult finding the day of a certain date
		// This small function would be 10x longer if I had to implement this myself
		// Thanks, developers of Chrono!
		let localtime = Local::now();
		match localtime.weekday() {
			Sun => Self::Sunday,
			Mon => Self::Monday,
			Tue => Self::Tuesday,
			Wed => Self::Wednesday,
			Thu => Self::Thursday,
			Fri => Self::Friday,
			Sat => Self::Saturday,
		}
	}

	pub fn as_str(&self) -> &str {
		match self {
			Self::Sunday => "Sunday",
			Self::Monday => "Monday",
			Self::Tuesday => "Tuesday",
			Self::Wednesday => "Wednesday",
			Self::Thursday => "Thursday",
			Self::Friday => "Friday",
			Self::Saturday => "Saturday",
		}
	}
}

impl fmt::Display for Day {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

// Since this is only an enum, we don't need tests for it
// What would you even test? assert!(Sunday != Monday);

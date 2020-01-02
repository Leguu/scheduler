use super::*;

// This struct is very boring, not much complexity going on
// See date.rs to understand #[derive()]
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone, Copy)]
/// Time struct, for times. In 24hr style.
pub struct Time {
	hour: u8,
	minute: u8,
}

impl Time {
	pub fn new(hour: u8, minute: u8) -> Result<Self, &'static str> {
		if hour > 23 || minute > 59 {
			Err("Hours have to be less than 24 and minutes have to be less than 60")
		} else {
			Ok(Self { hour, minute })
		}
	}

	pub fn as_string(self) -> String {
		format!("{:02}:{:02}", self.hour, self.minute)
	}

	/// Checks if the time is in between two other times.
	pub fn is_between(&self, start: &Time, end: &Time) -> bool {
		start < self && self < end
	}
}

impl fmt::Display for Time {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:02}:{:02}", self.hour, self.minute)
	}
}

//  XXXXXXXX XXXXXX   XXXXX XXXXXXXX  XXXXX
//     XX    XX      XX        XX    XX
//     XX    XXXXX   XXXXXX    XX    XXXXXX
//     XX    XX          XX    XX        XX
//     XX    XXXXXX  XXXXX     XX    XXXXX

#[cfg(test)]
mod time_tests {
	use crate::clock::*;

	// Why do I even need this many tests for such a small structure?

	#[test]
	fn validation() {
		assert!(Time::new(12, 0).is_ok());
		assert!(Time::new(0, 0).is_ok());
		assert!(Time::new(23, 59).is_ok());
		assert!(Time::new(24, 0).is_err());
		assert!(Time::new(0, 60).is_err());
	}

	#[test]
	fn comparison() {
		let time1 = Time::new(9, 0).unwrap();
		let time2 = Time::new(10, 0).unwrap();
		assert!(time1 < time2);
		let time3 = Time::new(13, 0).unwrap();
		assert!(time3 > time2 && time3 > time1);
	}

	#[test]
	fn is_between() {
		let start = Time::new(0, 0).unwrap();
		let end = Time::new(6, 0).unwrap();
		let time = Time::new(5, 0).unwrap();

		assert!(time.is_between(&start, &end));

		let other_time = Time::new(12, 0).unwrap();

		assert!(!other_time.is_between(&start, &end));
	}
}

use super::*;
use std::convert::TryFrom;

// For the #[derive()] statement explanation, see application.rs
// PartialEq automatically implements == for us
// PartialOrd implements < and > for us
// Copy is just Clone but automatically called whenever necessary, unlike Clone
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone, Copy)]
/// The date struct contains data for a date, such as 1984-01-24.
pub struct Date {
	year: u16,
	month: u8,
	day: u8,
}

impl Date {
	pub fn new(year: u16, month: u8, day: u8) -> Result<Self, ()> {
		// Returns the number of days in a specific month
		// This array is 1-indexed, so you can access it with the normal month number
		// Example, if self.month is 2 (February), then there is 28 days
		let days_in_months = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

		// 1970 is the base year for no reason
		// I assume my client is not a time traveller
		// Maybe I should do another interview to check
		if day == 0 || month == 0 || year < 1970 || month > 12 {
			Err(())
		// Checks if the day of the month is greater than that month allows
		// [0, 31, 28,...] are the days in each month, and I'm accessing it with [month]
		} else if day > days_in_months[month as usize] {
			Err(())
		} else {
			Ok(Self { year, month, day })
		}
	}

	/// Checks if the date is in between two other dates.
	pub fn is_between(&self, start: &Date, end: &Date) -> bool {
		start < self && self < end
	}

	/// Returns the date of today.
	pub fn today() -> Self {
		// We don't want to mess with different OS interfaces, so I'm just going to depend on Chrono for this one
		let localtime = Local::now();
		Self::new(
			localtime.year() as u16,
			localtime.month() as u8,
			localtime.day() as u8,
		)
		.unwrap()
	}

	pub fn as_string(self) -> String {
		format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
	}
}

// I am implementing TryFrom for the String class
// This means we can *try* to convert any String into a Date
// If this conversion fails, this method will return an Error
impl TryFrom<String> for Date {
	type Error = ();
	/// The correct format here is YYYY-MM-DD, no other is accepted.
	fn try_from(string: String) -> Result<Self, ()> {
		// First, split the string by '-', so we get 3 parts
		let segments: Vec<&str> = string.split('-').collect();

		// If the number of parts is less than 3, then return error
		// If the number of parts is more than 3, you messed something up and I don't care
		// (It'll probably still return an error later)
		if segments.len() < 3 {
			return Err(());
		}

		// Weird construct, only possible in Rust
		// If we can parse each element correctly, then we return a new date containing it
		// Otherwise we return an error
		if let (Ok(year), Ok(month), Ok(day)) = (
			segments[0].parse(),
			segments[1].parse(),
			segments[2].parse(),
		) {
			Date::new(year, month, day)
		} else {
			Err(())
		}
	}
}

// Implementing Add for Date, so we can do Date + Date
// This function isn't that useful and is used rarely, so I won't comment it
impl std::ops::Add<usize> for Date {
	type Output = Date;

	fn add(mut self, other: usize) -> Date {
		let day_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][self.month as usize];

		self.day += other as u8;
		if self.day / day_in_month > 1 {
			self.month += 1;
		}
		self.day %= day_in_month;

		self
	}
}

// We can now display the Date in the format YYYY-MM-DD
impl fmt::Display for Date {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
	}
}

//  XXXXXXXX XXXXXX   XXXXX XXXXXXXX  XXXXX
//     XX    XX      XX        XX    XX
//     XX    XXXXX   XXXXXX    XX    XXXXXX
//     XX    XX          XX    XX        XX
//     XX    XXXXXX  XXXXX     XX    XXXXX

#[cfg(test)]
mod tests {
	use crate::clock::*;

	#[test]
	fn validation() {
		// Here we check if the dates are valid.
		// Various test statements

		// Read: `assert that 2019-10-28 is_ok`
		assert!(Date::new(2019, 10, 28).is_ok());
		assert!(Date::new(2000, 1, 1).is_ok());

		// Read: `assert that 2019-10-32 is NOT ok`
		assert!(!Date::new(2019, 10, 32).is_ok());
		assert!(!Date::new(2019, 13, 1).is_ok()); // Array out of bounds error!, fixed
	}

	#[test]
	fn comparison() {
		// Just checking that the < and > operators work
		// This was implemented with the #derive statement
		let date1 = Date::new(1990, 1, 1);
		let date2 = Date::new(2000, 1, 1);
		assert!(date1 < date2);
	}

	#[test]
	fn is_between() {
		let start = Date::new(2020, 1, 1).unwrap();
		let end = Date::new(2020, 12, 1).unwrap();
		let date = Date::new(2020, 6, 1).unwrap();

		assert!(date.is_between(&start, &end));

		let other_date = Date::new(2021, 1, 1).unwrap();

		assert!(!other_date.is_between(&start, &end));
	}
}

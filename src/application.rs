use super::*;
use course::*;
use std::fs::File;
use std::io::*;

// The #[derive()] statement automatically implements some traits for us
// For example, here, we can serialize and deserialize this object
// We can also clone it (basically copying the object to another location of memory, useful for passing to functions
// And debug, which allows us to see a nice representation of the object using a print statement
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Application {
	/// A list of all the courses the user has
	pub courses: Vec<Course>,
	/// A list of holidays made up of (starting date, ending date)
	pub holidays: Vec<(Date, Date)>,
}

impl Application {
	pub fn new() -> Self {
		Self {
			courses: Vec::new(),
			holidays: Vec::new(),
		}
	}

	pub fn new_course(&mut self) {
		self.courses.push(Course::new("Name", "Teacher", "Room"));
	}

	pub fn add_course(&mut self, course: Course) {
		self.courses.push(course);
	}

	pub fn rm_course(&mut self, index: usize) {
		self.courses.remove(index);
	}

	/// Horrendous function that checks every holiday and finds overlaps, removing them if necessary
	/// "If it works but looks absolutely terrible, refactor it later"
	/// - Me, 2020-01-01
	fn recheck_holidays(&mut self) {
		let mut len = self.holidays.len();
		for i in 0..len {
			for j in 0..len {
				len = self.holidays.len();

				if i >= len || j >= len {
					return;
				} else if i == j {
					continue;
				}

				let (start_other, end_other) = self.holidays[j];
				let (start_i, end_i) = &mut self.holidays[i];

				if *start_i < start_other && start_other < *end_i {
					if *end_i < end_other {
						*end_i = end_other;
					}
					self.holidays.remove(j);
				}
			}
		}
	}

	/// Checks if `date` is a holiday or not.
	pub fn is_holiday(&self, date: Date) -> bool {
		for &(start, end) in &self.holidays {
			if start < date && date < end {
				return true;
			}
		}
		false
	}

	/// Creates a new default holiday from today to today, and adds it to the list of holidays
	pub fn new_holiday(&mut self) {
		self.holidays.push((Date::today(), Date::today()));
	}

	pub fn add_holiday(&mut self, start_date: Date, end_date: Date) {
		self.holidays.push((start_date, end_date));
		// Have to check it twice here, or else some edge cases won't catch
		self.recheck_holidays();
		self.recheck_holidays();
	}

	/// Removes holiday at index
	/// Useful because the GUI module returns an index when a GUI element in a list is selected
	pub fn rm_holiday(&mut self, index: usize) {
		self.holidays.remove(index);
	}

	/// Returns the memory representation of this object
	/// Useful for when writing to a file, as it is tiny in comparison to JSON and the like
	pub fn serialize(&self) -> Vec<u8> {
		bincode::serialize(self).unwrap()
	}

	/// Saves this application to `location`
	/// Uses its in-memory location, see serialize
	pub fn save(&self, location: &str) {
		if File::open(location).is_err() {
			let mut file = File::create(location).unwrap();
			file.write_all(&self.serialize()).unwrap();
		} else {
			std::fs::remove_file(location).unwrap();
			self.save(location);
		}
	}

	/// Tries to load an application from a file, returns an error if unsuccessful
	pub fn load(location: &str) -> std::result::Result<Self, ()> {
		if let Ok(mut file) = File::open(location) {
			let mut buf = Vec::new();
			file.read_to_end(&mut buf).unwrap();
			Ok(Application::from(buf))
		} else {
			Err(())
		}
	}

	/// Tries to load an application, and returns an empty one if it fails
	pub fn load_or_default(location: &str) -> Self {
		if let Ok(new) = Self::load(location) {
			new
		} else {
			Self::new()
		}
	}
}

impl From<Vec<u8>> for Application {
	/// Creating an Application from a byte array
	/// Useful, as the memory representation is a byte array
	/// Basically the `deserialize` method
	fn from(buf: Vec<u8>) -> Self {
		bincode::deserialize(&buf).unwrap()
	}
}

//  //////// //////  ////// //////// //////
//     //    //      //        //    //
//     //    //////  //////    //    //////
//     //    //          //    //        //
//     //    //////  //////    //    //////

#[cfg(test)]
mod tests {
	use super::*;

	/// Helper function, returns a Date and asserts it is real
	fn date(year: u16, month: u8, day: u8) -> Date {
		Date::new(year, month, day).unwrap()
	}

	#[test]
	fn recheck() {
		let mut application = Application::new();

		application.add_holiday(date(2010, 6, 1), date(2010, 8, 1));
		application.add_holiday(date(2010, 7, 1), date(2010, 9, 1));

		assert_eq!(
			application.holidays,
			vec![(date(2010, 6, 1), date(2010, 9, 1))]
		);

		application.add_holiday(date(2010, 5, 1), date(2010, 10, 1));

		assert_eq!(
			application.holidays,
			vec![(date(2010, 5, 1), date(2010, 10, 1))]
		);

		application.add_holiday(date(2009, 5, 1), date(2020, 10, 1));

		assert_eq!(
			application.holidays,
			vec![(date(2009, 5, 1), date(2020, 10, 1))]
		);
	}

	#[test]
	fn is_holiday() {
		let mut application = Application::new();

		application.add_holiday(date(2010, 6, 1), date(2010, 8, 1));

		assert!(application.is_holiday(date(2010, 7, 1)));
	}
}

use super::*;
use task::Task;

// See application.rs to understand #[derive()]
#[derive(Serialize, Deserialize, Clone, Debug)]
/// This struct contains details for a course.
pub struct Course {
	/// The name, teacher, and room of this Course as Strings.
	pub name: String,
	pub teacher: String,
	pub room: String,
	/// A list of the timings for the course, as in (Sunday, from 10:30, to 12:30).
	pub times: Vec<(Day, Time, Time)>,
	/// A list of all the tasks for that course.
	pub tasks: Vec<Task>,
}

impl Course {
	// type T is anything that can be turned into `String`
	// Useful, since you can pass in anything that implements the Into<String> trait
	pub fn new<T: Into<String>>(name: T, teacher: T, room: T) -> Self {
		Self {
			name: name.into(),
			teacher: teacher.into(),
			room: room.into(),
			times: Vec::new(),
			tasks: Vec::new(),
		}
	}

	pub fn add_task<T: Into<String>>(&mut self, name: T, desc: T, due: Date) {
		self.tasks.push(Task::new(name, desc, due));
	}

	pub fn rm_task(&mut self, index: usize) {
		self.tasks.remove(index);
	}

	/// Horrendous function that checks every time and finds overlaps, removing them if necessary.
	///
	/// "If it works but looks absolutely terrible, refactor it later"
	///  - Me, 2020-01-01
	///
	/// This function is similar to `recheck_holidays` in application.rs, so if you read that you can skip this.
	fn recheck_times(&mut self) {
		let mut len = self.times.len();
		// Since we don't know whether there are duplicates, we need to check every time against every other
		// Therefore, we have a nested for loop that iterates over the index of every element in the Vec
		for i in 0..len {
			for j in 0..len {
				// We reset the len variable because we're removing items in the loop
				len = self.times.len();

				// We need to check if we're out of the loop now
				// Also we check if we're comparing the same items, in which case we skip
				if i >= len || j >= len {
					return;
				} else if i == j {
					continue;
				}

				// We get the elements at the array at this point
				// start_i and end_i are mutable variables, which means we can change them
				// start_other and end_other is what we're checking against
				let (day_other, start_other, end_other) = self.times[j];
				let (day_i, start_i, end_i) = &mut self.times[i];

				// If the days aren't the same, we can't check them
				if day_other != *day_i {
					continue;
				}

				// If the start of the other date is in between the current element
				if start_other.is_between(start_i, end_i) {
					// Then we check if the current end is less than the end of the other one
					// A helpful illustration (sorry if you're using non-monospaced fonts):

					//    is       os         ie       oe
					//    v        v          v        v
					// <--|--------|----------|xxxxxxxx|-->

					// is and ie stands for "i start" and "i end"
					// os and oe stands for "other start" and "other end"
					// If oe is further ahead than ie, then we have to move ie to oe
					// Otherwise we will lose the area marked with the 'x's when we remove eo

					if *end_i < end_other {
						*end_i = end_other;
					}

					// If this case isn't true, then the illustration would look like this:

					//    is       os         oe       ie
					//    v        v          v        v
					// <--|--------|----------|--------|-->

					// And we can safely remove so and eo without having to move anything
					self.times.remove(j);
				}
			}
		}
	}

	pub fn add_time(&mut self, new_day: Day, new_start: Time, new_end: Time) {
		self.times.push((new_day, new_start, new_end));
		// Have to check it twice here, or else some edge cases won't catch
		// TODO: Is there a way around this?
		self.recheck_times();
		self.recheck_times();
	}

	pub fn rm_time(&mut self, index: usize) {
		self.times.remove(index);
	}

	/// Checks if there is a lesson on a specific day
	pub fn is_on_day(&self, expected: Day) -> bool {
		self.times.iter().any(|&(day, _, _)| day == expected)
	}
}

//  XXXXXXXX XXXXXX   XXXXX XXXXXXXX  XXXXX
//     XX    XX      XX        XX    XX
//     XX    XXXXX   XXXXXX    XX    XXXXXX
//     XX    XX          XX    XX        XX
//     XX    XXXXXX  XXXXX     XX    XXXXX

#[cfg(test)]
mod tests {
	use super::*;

	fn time(hour: u8, minute: u8) -> Time {
		Time::new(hour, minute).unwrap()
	}

	#[test]
	fn recheck() {
		let mut math = Course::new("Math", "Willis", "S37");
		math.add_time(Day::Sunday, time(9, 0), time(11, 0));
		math.add_time(Day::Sunday, time(10, 0), time(12, 0));

		assert_eq!(math.times, vec![(Day::Sunday, time(9, 0), time(12, 0),)]);
	}

	#[test]
	fn is_on_day() {
		let mut math = Course::new("Math", "Willis", "S37");
		math.add_time(Day::Sunday, time(9, 0), time(11, 0));

		assert!(math.is_on_day(Day::Sunday));
		assert!(!math.is_on_day(Day::Monday));
	}

	#[test]
	fn add_time() {
		let mut math = Course::new("Math", "Willis", "S37");

		math.add_time(Day::Sunday, time(9, 0), time(10, 0));
		math.add_time(Day::Sunday, time(9, 30), time(12, 0));

		assert_eq!(math.times, vec![(Day::Sunday, time(9, 0), time(12, 0),)]);

		math.add_time(Day::Sunday, time(13, 0), time(15, 0));

		// Current Math Times on Sunday:
		// 9 to 12
		// 13 to 15

		math.add_time(Day::Sunday, time(11, 0), time(14, 0));

		// Expected Math Times on Sunday:
		// 9 to 15

		assert_eq!(math.times, vec![(Day::Sunday, time(9, 0), time(15, 0),)]);

		math.add_time(Day::Sunday, time(14, 0), time(16, 0));
		math.add_time(Day::Sunday, time(14, 0), time(17, 0));
		math.add_time(Day::Sunday, time(7, 0), time(13, 0));
		math.add_time(Day::Sunday, time(6, 0), time(13, 0));

		assert_eq!(math.times, vec![(Day::Sunday, time(6, 0), time(17, 0),)]);
	}
}

use super::*;
use task::Task;

// See application.rs to understand #[derive()]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Course {
	/// The name, teacher, and room of this Course as Strings
	pub name: String,
	pub teacher: String,
	pub room: String,
	/// A list of the timings for the course, as in (Sunday, from 10:30, to 12:30)
	pub times: Vec<(Day, Time, Time)>,
	/// A list of all the tasks for that course
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

	pub fn remove_task(&mut self, index: usize) {
		self.tasks.remove(index);
	}

	/// Horrendous function that checks every time and finds overlaps, removing them if necessary
	/// "If it works but looks absolutely terrible, refactor it later"
	/// - Me, 2020-01-01
	fn recheck_times(&mut self) {
		let mut len = self.times.len();
		for i in 0..len {
			for j in 0..len {
				len = self.times.len();
				if i >= len || j >= len {
					return;
				} else if i == j {
					continue;
				}

				let (_, start_other, end_other) = self.times[j];
				let (_, start_i, end_i) = &mut self.times[i];

				if *start_i < start_other && start_other < *end_i {
					if *end_i < end_other {
						*end_i = end_other;
					}
					self.times.remove(j);
				}
			}
		}
	}

	pub fn add_time(&mut self, new_day: Day, new_start: Time, new_end: Time) {
		self.times.push((new_day, new_start, new_end));
		self.recheck_times();
		self.recheck_times();
	}

	pub fn rm_time(&mut self, index: usize) {
		self.times.remove(index);
	}

	/// Checks if there is a lesson on a specific day
	pub fn is_on_day(&self, expected: Day) -> bool {
		// For every day the course is on,
		// Return True if the day is expected
		for (day, _, _) in &self.times {
			if *day == expected {
				return true;
			}
		}
		false
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

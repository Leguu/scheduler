use super::*;

// See application.rs to understand #[derive()]
#[derive(Serialize, Deserialize, Clone, Debug)]
/// This struct contains all the data necessary for a task (in a course).
pub struct Task {
	pub name: String,
	pub desc: String,
	// "due: Date" sounds nice, doesn't it?
	pub due: Date,
	/// A list of the steps for that task, as in (is it completed, name of step).
	pub steps: Vec<(bool, String)>,
}

impl Task {
	pub fn new<T: Into<String>>(name: T, desc: T, due: Date) -> Self {
		// Creates a dummy step, because is_complete() depends on there being steps in the task
		let steps = vec![(false, "Completed?".to_string())];
		Self {
			name: name.into(),
			desc: desc.into(),
			due,
			steps,
		}
	}

	/// Returns whether the entire task is complete.
	/// Checks all the steps and returns whether or not there are any incomplete steps.
	pub fn is_complete(&self) -> bool {
		// Functional programming for the win!
		// Checks if any() steps are !complete
		// If even one step is incomplete, then this function will return false
		!self.steps.iter().any(|(complete, _)| !complete)
	}

	/// Sets a step as complete or not complete.
	pub fn toggle_step(&mut self, index: usize) {
		if let Some((complete, _)) = self.steps.get_mut(index) {
			*complete = !*complete;
		}
	}

	/// Add a step to the task, incomplete by default.
	/// If it's the first step in the task, removes the dummy step.
	pub fn add_step<T: Into<String>>(&mut self, description: T) {
		if self.steps[0] == (false, "Completed?".to_string()) {
			self.steps.remove(0);
		}
		self.steps.push((false, description.into()));
	}

	/// Removes a step to the task.
	/// If it's the last step in the task, adds a new dummy step.
	pub fn rm_step(&mut self, index: usize) {
		self.steps.remove(index);
		if self.steps.is_empty() {
			self.steps.push((false, "Completed?".to_string()))
		}
	}

	/// Returns a string based on whether it's complete or not.
	/// Useful only for the GUI.
	pub fn is_complete_str(&self) -> &str {
		if self.is_complete() {
			"Done"
		} else {
			"Not Done"
		}
	}
}

impl Default for Task {
	fn default() -> Self {
		Self::new("Name", "Description", Date::today())
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

	#[test]
	fn is_complete() {
		let mut task = Task::new("", "", Date::new(2000, 1, 1).unwrap());
		assert!(!task.is_complete());
		task.steps = vec![(true, "".to_string())];
		assert!(task.is_complete());
	}

	#[test]
	fn toggle() {
		let mut task = Task::new("", "", Date::new(2000, 1, 1).unwrap());
		assert!(!task.is_complete());
		task.toggle_step(0);
		assert!(task.is_complete());
	}
}

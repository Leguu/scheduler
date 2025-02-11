use super::*;

/// Pop-up window for a specific course.
pub(super) fn course(
	gui_app: &gtk::Application,
	index: usize,
	application: Rc<RefCell<Application>>,
) {
	let course = &application.borrow().courses[index];
	let window = ApplicationWindow::new(gui_app);

	let t1 = text_with_default(&course.name, None);
	let t2 = text_with_default(&course.teacher, None);
	let t3 = text_with_default(&course.room, None);

	let f1 = frame_with_text("Course Name", &t1);
	let f2 = frame_with_text("Teacher Name", &t2);
	let f3 = frame_with_text("Room ID", &t3);

	let listbox_times = ListBox::new();
	for (day, start, end) in &course.times {
		listbox_times.insert(
			&Label::new(Some(&format!("{}: From {} to {}", day, start, end))),
			-1,
		);
	}
	let f4 = frame_with_text("Times", &listbox_times);
	f4.set_hexpand(true);
	f4.set_vexpand(true);

	let listbox_tasks = ListBox::new();
	for task in &course.tasks {
		listbox_tasks.insert(
			&Label::new(Some(&format!("{}: {}", task.is_complete_str(), task.name))),
			-1,
		);
	}
	let f5 = frame_with_text("Tasks", &listbox_tasks);
	f5.set_vexpand(true);

	let button_add_time = Button::new_with_label("Add Time");
	button_add_time.connect_clicked(
		clone!(@weak listbox_times, @weak application, @weak window => move |_| {
			application.borrow_mut().courses[index].new_time();
			let time = *application.borrow().courses[index].times.last().unwrap();
			listbox_times.insert(
				&Label::new(Some(&format!("{}: From {} to {}", time.0, time.1, time.2))),
				-1,
			);
			window.show_all();
		}),
	);
	let button_rm_time = Button::new_with_label("Rm Time");
	button_rm_time.connect_clicked(
		clone!( @weak listbox_times, @weak application, @weak window => move |_| {
			if let Some(row) = listbox_times.get_selected_row() {
				let time_index = row.get_index();
				application.borrow_mut().courses[index as usize].times.remove(time_index as usize);
				listbox_times.remove(&row);
				window.show_all();
			}
		}),
	);
	let button_edit_time = Button::new_with_label("Edit Time");
	button_edit_time.connect_clicked(
		clone!(@weak listbox_times, @weak application, @weak gui_app => move |_| {
			if let Some(row) = listbox_times.get_selected_row() {
				let time_index = row.get_index() as usize;
				window::time_dialog(time_index, index, application, &gui_app)
			}
		}),
	);
	let button_add_task = Button::new_with_label("Add Task");
	button_add_task.connect_clicked(
		clone!(@weak listbox_tasks, @weak application, @weak window => move |_| {
			let course = &mut application.borrow_mut().courses[index];
			course.new_task();
			let task = course.tasks.last().unwrap();
			listbox_tasks.insert(
				&Label::new(Some(&format!("{}: {}", task.is_complete_str(), task.name))),
				-1,
			);
			window.show_all();
		}),
	);
	let button_rm_task = Button::new_with_label("Rm Task");
	button_rm_task.connect_clicked(
		clone!( @weak listbox_tasks, @weak application, @weak window => move |_| {
			if let Some(row) = listbox_tasks.get_selected_row() {
				let task_index = row.get_index();
				application.borrow_mut().courses[index as usize].tasks.remove(task_index as usize);
				listbox_tasks.remove(&row);
				window.show_all();
			}
		}),
	);
	let button_edit_task = Button::new_with_label("Edit Task");
	button_edit_task.connect_clicked(
		clone!(@weak listbox_tasks, @weak application, @weak gui_app => move |_| {
			if let Some(row) = listbox_tasks.get_selected_row() {
				let task_index = row.get_index() as usize;
				window::task(index, task_index, &gui_app, application)
			}
		}),
	);
	let button_save = Button::new_with_label("Save");
	button_save.connect_clicked(
		clone!(@weak t1, @weak t2, @weak t3, @weak application => move |_| {
			let course = &mut application.borrow_mut().courses[index];
			course.name = get_string_from_text!(t1);
			course.teacher = get_string_from_text!(t2);
			course.room = get_string_from_text!(t3);
		}),
	);

	let grid = Grid::new();
	grid.attach(&f1, 0, 0, 2, 1);
	grid.attach(&f2, 0, 1, 2, 1);
	grid.attach(&f3, 0, 2, 2, 1);
	grid.attach(&f4, 0, 3, 2, 1);

	grid.attach(&button_add_time, 0, 4, 1, 1);
	grid.attach(&button_rm_time, 1, 4, 1, 1);
	grid.attach(&button_edit_time, 0, 5, 2, 1);

	grid.attach(&f5, 0, 6, 2, 1);

	grid.attach(&button_add_task, 0, 7, 1, 1);
	grid.attach(&button_rm_task, 1, 7, 1, 1);
	grid.attach(&button_edit_task, 0, 8, 2, 1);

	grid.attach(&button_save, 0, 9, 2, 1);

	window.add(&grid);
	window.show_all();
}

/// Pop-up window for editing a holiday.
pub(super) fn holiday(
	gui_app: &gtk::Application,
	index: usize,
	application: Rc<RefCell<Application>>,
) {
	let holiday = &application.borrow().holidays[index];
	let window = ApplicationWindow::new(gui_app);

	let t1 = text_with_default(&holiday.0.as_string(), None);
	let t2 = text_with_default(&holiday.1.as_string(), None);

	let f1 = frame_with_text("Start Date", &t1);
	f1.set_hexpand(true);

	let f2 = frame_with_text("End Date", &t2);
	f2.set_hexpand(true);

	let button_save = Button::new_with_label("Save");
	button_save.connect_clicked(clone!(@weak t1, @weak t2, @weak application => move |_| {
		if let (Ok(start), Ok(end)) = (
			Date::try_from(get_string_from_text!(t1)),
			Date::try_from(get_string_from_text!(t2)),
		) {
			if start > end {
				message_dialog("Start date greater than end date!");
			} else {
				application.borrow_mut().holidays[index] = (start, end);
			}
		} else {
			message_dialog("Date entry invalid. Recheck entry, and only use numbers in format: 'YYYY-MM-DD'.");
		}
	}));

	let grid = Grid::new();
	grid.attach(&f1, 0, 1, 1, 1);
	grid.attach(&f2, 0, 2, 1, 1);

	grid.attach(&button_save, 0, 3, 1, 1);

	window.add(&grid);
	window.show_all();
}

/// Pop-up window for editing a task.
pub(super) fn task(
	course_index: usize,
	task_index: usize,
	gui_app: &gtk::Application,
	application: Rc<RefCell<Application>>,
) {
	let window = ApplicationWindow::new(gui_app);

	let course = &mut application.borrow_mut().courses[course_index];
	let task = &mut course.tasks[task_index];

	let t1 = text_with_default(&task.name, None);
	let t2 = text_with_default(&task.desc, Some(WrapMode::Word));
	let t3 = text_with_default(&task.due.to_string(), None);

	let f1 = frame_with_text("Task Name", &t1);
	let f2 = frame_with_text("Description", &t2);
	f2.set_hexpand(true);
	let f3 = frame_with_text("Due Date", &t3);

	let listbox = ListBox::new();
	for (step_index, (done, desc)) in task.steps.iter().enumerate() {
		let done_desc = if *done { "Done" } else { "Undone" };

		let done_button = Button::new_with_label(done_desc);
		done_button.connect_clicked(clone!(@weak application, @weak window => move |but| {
			let course = &mut application.borrow_mut().courses[course_index];
			let task = &mut course.tasks[task_index];
			task.toggle_step(step_index);
			match but.get_label().unwrap().as_str() {
				"Done" => but.set_label("Undone"),
				"Undone" => but.set_label("Done"),
				_ => ()
			}
			window.show_all();
		}));

		let grid = Grid::new();
		grid.attach(&done_button, 0, 0, 1, 1);
		grid.attach(&Label::new(Some(desc)), 1, 0, 1, 1);
		listbox.insert(&grid, -1);
	}
	let f4 = frame_with_text("Task Steps", &listbox);
	f4.set_vexpand(true);

	let button_add_step = Button::new_with_label("Add Step");
	button_add_step.connect_clicked(
		clone!(@weak listbox, @weak application, @weak window => move |_| {
			let course = &mut application.borrow_mut().courses[course_index];
			let task = &mut course.tasks[task_index];
			task.new_step();
			let step = task.steps.last().unwrap();
			let step_index = task.steps.len() -1;

			// This code is repeated above... For now I'll leave it as-is
			let done_button = Button::new_with_label("Undone");
			done_button.connect_clicked(clone!(@weak application, @weak window => move |but| {
				let course = &mut application.borrow_mut().courses[course_index];
				let task = &mut course.tasks[task_index];
				task.toggle_step(step_index);
				match but.get_label().unwrap().as_str() {
					"Done" => but.set_label("Undone"),
					"Undone" => but.set_label("Done"),
					_ => ()
				}
				window.show_all();
			}));

			let grid = Grid::new();
			grid.attach(&done_button, 0, 0, 1, 1);
			grid.attach(&Label::new(Some(&step.1)), 1, 0, 1, 1);
			listbox.insert(&grid, -1);

			window.show_all();
		}),
	);
	let button_rm_step = Button::new_with_label("Rm Step");
	button_rm_step.connect_clicked(
		clone!( @weak listbox, @weak application, @weak window => move |_| {
			if let Some(row) = listbox.get_selected_row() {
				let step_index = row.get_index();
				application.borrow_mut().courses[course_index as usize].tasks[task_index].rm_step(step_index as usize);
				listbox.remove(&row);
				window.show_all();
			}
		}),
	);
	let button_edit_step = Button::new_with_label("Edit Step");
	button_edit_step.connect_clicked(clone!( @weak gui_app, @weak application => move |_| {
		if let Some(row) = listbox.get_selected_row() {
			let step_index = row.get_index() as usize;
			window::step_dialog(course_index, task_index, step_index, &gui_app, application);
		}
	}));
	let button_save = Button::new_with_label("Save");
	button_save.connect_clicked(
		clone!(@weak t1, @weak t2, @weak t3, @weak application => move |_| {
			let course = &mut application.borrow_mut().courses[course_index];
			let task = &mut course.tasks[task_index];

			task.name = get_string_from_text!(t1);
			task.desc = get_string_from_text!(t2);

			if let Ok(due) = Date::try_from(get_string_from_text!(t3)) {
				task.due = due;
			} else {
				message_dialog("Date entry invalid. Recheck entry, and only use numbers in format: 'YYYY-MM-DD'.");
			}
		}),
	);

	let grid = Grid::new();
	grid.attach(&f1, 0, 0, 2, 1);
	grid.attach(&f2, 0, 1, 2, 1);
	grid.attach(&f3, 0, 2, 2, 1);
	grid.attach(&f4, 0, 3, 2, 1);

	grid.attach(&button_add_step, 0, 5, 1, 1);
	grid.attach(&button_rm_step, 1, 5, 1, 1);
	grid.attach(&button_edit_step, 0, 6, 2, 1);

	grid.attach(&button_save, 0, 7, 2, 1);

	window.add(&grid);
	window.show_all();
}

/// A small dialog for editing a step.
pub(super) fn step_dialog(
	course_index: usize,
	task_index: usize,
	step_index: usize,
	gui_app: &gtk::Application,
	application: Rc<RefCell<Application>>,
) {
	let course = &mut application.borrow_mut().courses[course_index];
	let task = &mut course.tasks[task_index];
	let step = &mut task.steps[step_index];

	let text_box = text_with_default(&step.1, None);
	let button_save = Button::new_with_label("Save");
	button_save.connect_clicked(clone!(@weak text_box, @weak application => move |_| {
		let course = &mut application.borrow_mut().courses[course_index];
		let task = &mut course.tasks[task_index];
		task.steps[step_index].1 = get_string_from_text!(text_box);
	}));

	let grid = Grid::new();
	grid.attach(&text_box, 0, 0, 1, 1);
	grid.attach(&button_save, 0, 1, 1, 1);

	let window = ApplicationWindow::new(gui_app);
	window.add(&grid);
	window.show_all();
}

/// A small dialog for editing a time.
pub(super) fn time_dialog(
	time_index: usize,
	course_index: usize,
	application: Rc<RefCell<Application>>,
	gui_app: &gtk::Application,
) {
	let course = &application.borrow().courses[course_index];
	let (day, start, end) = course.times[time_index];

	let t1 = text_with_default(day.as_str(), None);
	t1.set_left_margin(3);
	t1.set_right_margin(3);

	let t2 = text_with_default(&start.to_string(), None);
	t2.set_left_margin(3);
	t2.set_right_margin(3);

	let t3 = text_with_default(&end.to_string(), None);
	t3.set_left_margin(3);
	t3.set_right_margin(3);

	let button_save = Button::new_with_label("Save");
	button_save.connect_clicked(
		clone!(@weak t1, @weak t2, @weak t3, @weak application => move |_| {
			let course = &mut application.borrow_mut().courses[course_index];

			if let (Ok(day), Ok(start), Ok(end)) = (
				Day::try_from(get_string_from_text!(t1)),
				Time::try_from(get_string_from_text!(t2)),
				Time::try_from(get_string_from_text!(t3)),
			) {
				if start > end {
					message_dialog("Start time greater than end time!");
				} else {
					course.times[time_index] = (day, start, end);
				}
			} else {
				message_dialog("Time/Day entry invalid. Recheck entry, and only use numbers in format: 'HH-MM'.");
			}
		}),
	);

	let grid = Grid::new();
	grid.attach(&t1, 0, 0, 1, 1);
	grid.attach(&t2, 1, 0, 1, 1);
	grid.attach(&t3, 2, 0, 1, 1);

	grid.attach(&button_save, 0, 1, 3, 1);

	let window = ApplicationWindow::new(gui_app);
	window.add(&grid);
	window.show_all();
}

use super::*;

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

	let listbox_tasks = ListBox::new();
	for task in &course.tasks {
		listbox_tasks.insert(
			&Label::new(Some(&format!("{}: {}", task.is_complete_str(), task.desc))),
			-1,
		);
	}
	let f5 = frame_with_text("Tasks", &listbox_tasks);

	let grid = Grid::new();
	grid.attach(&f1, 0, 0, 2, 1);
	grid.attach(&f2, 0, 1, 2, 1);
	grid.attach(&f3, 0, 2, 2, 1);
	grid.attach(&f4, 0, 3, 2, 1);
	let button_add_time = Button::new_with_label("Add Time");
	let listbox_weak = listbox_times.downgrade();
	let application_weak = Rc::downgrade(&application);
	let gui_app_weak = gui_app.downgrade();
	button_add_time.connect_clicked(move |_| {
		if let Some(row) = up!(listbox_weak).get_selected_row() {
			let time_index = row.get_index();
			let application = application_weak.upgrade().unwrap();
			let gui_app = gui_app_weak.upgrade().unwrap();
			time_dialog(time_index as usize, index, application.clone(), &gui_app);
		}
	});
	grid.attach(&button_add_time, 0, 4, 1, 1);
	grid.attach(&Button::new_with_label("Rm Time"), 1, 4, 1, 1);
	grid.attach(&Button::new_with_label("Edit Time"), 0, 5, 2, 1);
	grid.attach(&f5, 0, 6, 2, 1);
	grid.attach(&Button::new_with_label("Add Task"), 0, 7, 1, 1);
	grid.attach(&Button::new_with_label("Rm Task"), 1, 7, 1, 1);
	grid.attach(&Button::new_with_label("Edit Task"), 0, 8, 2, 1);

	let button_save = Button::new_with_label("Save");
	let application_weak = Rc::downgrade(&application);
	let (t1_weak, t2_weak, t3_weak) = (t1.downgrade(), t2.downgrade(), t3.downgrade());
	button_save.connect_clicked(move |_| {
		let application = application_weak.upgrade().unwrap();
		let course = &mut application.borrow_mut().courses[index];

		let (t1, t2, t3) = (up!(t1_weak), up!(t2_weak), up!(t3_weak));

		course.name = get_string_from_text!(t1);
		course.teacher = get_string_from_text!(t2);
		course.room = get_string_from_text!(t3);
	});
	grid.attach(&button_save, 0, 8, 2, 1);

	window.add(&grid);
	window.show_all();
}

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

	let grid = Grid::new();
	grid.attach(&f1, 0, 1, 1, 1);
	grid.attach(&f2, 0, 2, 1, 1);
	let button_save = Button::new_with_label("Save");
	let application_weak = Rc::downgrade(&application);
	let t1_weak = t1.downgrade();
	let t2_weak = t2.downgrade();
	button_save.connect_clicked(move |_| {
		let application = application_weak.upgrade().unwrap();
		let holiday = &mut application.borrow_mut().holidays[index];

		let (t1, t2) = (up!(t1_weak), up!(t2_weak));

		if let (Ok(start), Ok(end)) = (
			Date::try_from(get_string_from_text!(t1)),
			Date::try_from(get_string_from_text!(t2)),
		) {
			holiday.0 = start;
			holiday.1 = end;
		} else {
			message_dialog("Date entry invalid. Use numbers only in format: YYYY-MM-DD.");
		}
	});
	grid.attach(&button_save, 0, 3, 1, 1);

	window.add(&grid);
	window.show_all();
}

/// TODO: INCOMPLETE
pub(super) fn time_dialog(
	time_index: usize,
	course_index: usize,
	application: Rc<RefCell<Application>>,
	gui_app: &gtk::Application,
) {
	let course = &application.borrow().courses[course_index];
	let (day, start, end) = course.times[time_index];

	let grid = Grid::new();
	let t1 = text_with_default(day.as_str(), None);
	t1.set_left_margin(3);
	t1.set_right_margin(3);
	let t2 = text_with_default(&start.to_string(), None);
	t2.set_left_margin(3);
	t2.set_right_margin(3);
	let t3 = text_with_default(&end.to_string(), None);
	t3.set_left_margin(3);
	t3.set_right_margin(3);
	grid.attach(&t1, 0, 0, 1, 1);
	grid.attach(&t2, 1, 0, 1, 1);
	grid.attach(&t3, 2, 0, 1, 1);

	let window = ApplicationWindow::new(gui_app);
	window.add(&grid);
	window.show_all();
}

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
			&Label::new(Some(&format!(
				"{}: {}",
				if task.is_complete() {
					"Done"
				} else {
					"Not Done"
				},
				task.desc
			))),
			-1,
		);
	}
	let f5 = frame_with_text("Tasks", &listbox_tasks);

	let grid = Grid::new();
	grid.attach(&f1, 0, 0, 2, 1);
	grid.attach(&f2, 0, 1, 2, 1);
	grid.attach(&f3, 0, 2, 2, 1);
	grid.attach(&f4, 0, 3, 2, 1);
	grid.attach(&Button::new_with_label("Add Time"), 0, 4, 1, 1);
	grid.attach(&Button::new_with_label("Rm Time"), 1, 4, 1, 1);
	grid.attach(&f5, 0, 5, 2, 1);
	grid.attach(&Button::new_with_label("Add Task"), 0, 6, 1, 1);
	grid.attach(&Button::new_with_label("Rm Task"), 1, 6, 1, 1);
	grid.attach(&Button::new_with_label("Edit Task"), 0, 7, 2, 1);

	let button_save = Button::new_with_label("Save");
	let application_weak = Rc::downgrade(&application);
	let (t1_weak, t2_weak, t3_weak) = (t1.downgrade(), t2.downgrade(), t3.downgrade());
	button_save.connect_clicked(move |_| {
		let application = application_weak.upgrade().unwrap();
		let course = &mut application.borrow_mut().courses[index];

		let t1 = t1_weak.upgrade().unwrap();
		let t2 = t2_weak.upgrade().unwrap();
		let t3 = t3_weak.upgrade().unwrap();

		course.name = t1
			.get_buffer()
			.unwrap()
			.get_text(
				&t1.get_buffer().unwrap().get_bounds().0,
				&t1.get_buffer().unwrap().get_bounds().1,
				false,
			)
			.unwrap()
			.to_string();
		course.teacher = t2
			.get_buffer()
			.unwrap()
			.get_text(
				&t2.get_buffer().unwrap().get_bounds().0,
				&t2.get_buffer().unwrap().get_bounds().1,
				false,
			)
			.unwrap()
			.to_string();
		course.room = t3
			.get_buffer()
			.unwrap()
			.get_text(
				&t3.get_buffer().unwrap().get_bounds().0,
				&t3.get_buffer().unwrap().get_bounds().1,
				false,
			)
			.unwrap()
			.to_string();
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

		let t1 = t1_weak.upgrade().unwrap();
		let t2 = t2_weak.upgrade().unwrap();

		if let (Ok(start), Ok(end)) = (
			Date::try_from(
				t1.get_buffer()
					.unwrap()
					.get_text(
						&t1.get_buffer().unwrap().get_bounds().0,
						&t1.get_buffer().unwrap().get_bounds().1,
						false,
					)
					.unwrap()
					.to_string(),
			),
			Date::try_from(
				t2.get_buffer()
					.unwrap()
					.get_text(
						&t2.get_buffer().unwrap().get_bounds().0,
						&t2.get_buffer().unwrap().get_bounds().1,
						false,
					)
					.unwrap()
					.to_string(),
			),
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

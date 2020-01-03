use super::*;

/// Open the "main" menu with today's courses and this week's tasks.
pub(super) fn main(grid: &Grid, application: Rc<RefCell<Application>>) {
	clear(grid);

	let courses = ListBox::new();
	if application.borrow().is_holiday(Date::today()) {
		courses.insert(&Label::new(Some("Today's a holiday - no lessons!")), -1);
	} else {
		for course in &application.borrow().courses {
			for time in &course.times {
				if time.0 == Day::today() {
					courses.insert(
						&Label::new(Some(
							format!("{}: {} to {}", course.name, time.1, time.2).as_str(),
						)),
						-1,
					);
				}
			}
		}
	}
	let courses_frame = FrameBuilder::new().label("Today's Courses").build();
	courses_frame.add(&courses);
	courses_frame.set_hexpand(true);
	courses_frame.set_vexpand(true);

	let hw = ListBox::new();
	for course in &application.borrow().courses {
		for task in &course.tasks {
			if Date::today() <= task.due && task.due <= Date::today() + 7 {
				hw.insert(
					&Label::new(Some(format!("{}: {}", task.name, task.desc).as_str())),
					-1,
				);
			}
		}
	}
	let hw_frame = FrameBuilder::new().label("Week's Tasks").build();
	hw_frame.add(&hw);
	hw_frame.set_hexpand(true);
	hw_frame.set_vexpand(true);

	grid.attach(&courses_frame, 1, 0, 1, 1);
	grid.attach(&hw_frame, 2, 0, 1, 1);
}

/// Open the weekly courses page. Has the weekdays Sunday-Thursday.
pub(super) fn weekly(grid: &Grid, application: Rc<RefCell<Application>>) {
	clear(grid);

	let mut vec = Vec::new();
	vec.push(ListBox::new());
	vec.push(ListBox::new());
	vec.push(ListBox::new());
	vec.push(ListBox::new());
	vec.push(ListBox::new());

	for course in &application.borrow().courses {
		for (day, start, end) in &course.times {
			if *day as usize > 4 {
				continue;
			}
			vec[*day as usize].insert(
				&Label::new(Some(
					format!("{}: {} to {}", course.name, start, end).as_str(),
				)),
				-1,
			);
		}
	}

	for ((i, day), listbox) in (1..6).zip(Day::weekdays().iter()).zip(vec.drain(..)) {
		let hw_frame = frame_with_text(day.as_str(), &listbox);
		listbox.set_selection_mode(SelectionMode::None);
		hw_frame.set_hexpand(true);
		hw_frame.set_vexpand(true);
		grid.attach(&hw_frame, i, 0, 1, 1);
	}
}

/// Shows the list of all courses.
pub(super) fn courses(
	grid: &Grid,
	gui_app: &gtk::Application,
	window: &gtk::ApplicationWindow,
	application: Rc<RefCell<Application>>,
) {
	clear(grid);

	let listbox = ListBox::new();
	for course in &application.borrow().courses {
		listbox.insert(&Label::new(Some(&course.name)), -1);
	}
	let frame2 = frame_with_text("Courses", &listbox);
	frame2.set_hexpand(true);
	frame2.set_vexpand(true);

	let add_button = Button::new_with_label("Add");
	add_button.connect_clicked(
		clone!( @weak gui_app, @weak grid, @weak application, @weak window => move |_| {
			application.borrow_mut().new_course();
			// let len = application.borrow().courses.len();
			// window::course(&gui_app_weak.upgrade().unwrap(), len - 1, application.clone());
			courses(&grid, &gui_app, &window, application);
		}),
	);

	let remove_button = Button::new_with_label("Remove");
	remove_button.connect_clicked(
		clone!( @weak listbox, @weak application, @weak grid, @weak gui_app, @weak window => move |_| {
			if let Some(row) = listbox.get_selected_row() {
				let index = row.get_index();
				application.borrow_mut().rm_course(index as usize);
				courses(&grid, &gui_app, &window, application);
			}
		}),
	);

	let edit_button = Button::new_with_label("View / Edit");
	edit_button.connect_clicked(clone!( @weak listbox, @weak gui_app => move |_| {
		if let Some(row) = listbox.get_selected_row() {
			let index = row.get_index();
			window::course(&gui_app, index as usize, application.clone());
		}
	}));

	grid.attach(&frame2, 1, 0, 2, 1);
	grid.attach(&add_button, 1, 1, 1, 1);
	grid.attach(&remove_button, 2, 1, 1, 1);
	grid.attach(&edit_button, 1, 2, 2, 1);

	window.show_all();
}

/// Shows a list of all holidays.
pub(super) fn holidays(
	gui_app: &gtk::Application,
	grid: &Grid,
	window: &gtk::ApplicationWindow,
	application: Rc<RefCell<Application>>,
) {
	clear(grid);

	let listbox = ListBox::new();
	for (start, end) in &application.borrow().holidays {
		listbox.insert(
			&Label::new(Some(format!("{} to {}", start, end).as_str())),
			-1,
		);
	}
	let frame2 = frame_with_text("Holidays", &listbox);
	frame2.set_hexpand(true);
	frame2.set_vexpand(true);

	let add_button = Button::new_with_label("Add");
	add_button.connect_clicked(
		clone!(@weak gui_app, @weak grid, @weak window, @weak application => move |_| {
			application.borrow_mut().new_holiday();
			// let len = application.borrow().holidays.len();
			// window::holiday(&gui_app_weak.upgrade().unwrap(), len - 1, application.clone());
			holidays(&gui_app, &grid, &window, application);
		}),
	);

	let remove_button = Button::new_with_label("Remove");
	remove_button.connect_clicked(
		clone!(@weak gui_app, @weak grid, @weak listbox, @weak window, @weak application => move |_| {
			if let Some(row) = listbox.get_selected_row() {
				let index = row.get_index();
				application.borrow_mut()
					.rm_holiday(index as usize);
				holidays(&gui_app, &grid, &window, application);
			}
		}),
	);

	let edit_button = Button::new_with_label("View / Edit");
	edit_button.connect_clicked(clone!(@weak listbox, @weak gui_app => move |_| {
		if let Some(row) = listbox.get_selected_row() {
			let index = row.get_index();
			window::holiday(&gui_app, index as usize, application.clone());
		}
	}));

	grid.attach(&frame2, 1, 0, 2, 1);
	grid.attach(&add_button, 1, 1, 1, 1);
	grid.attach(&remove_button, 2, 1, 1, 1);
	grid.attach(&edit_button, 1, 2, 2, 1);

	window.show_all();
}

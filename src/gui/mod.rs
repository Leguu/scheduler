use gtk::prelude::*;
use gtk::*;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::clock::*;
use crate::Application;
use util::*;

pub mod menu;
pub mod util;
pub mod window;

// Welcome brave soul to the `gui` module.
// This module is split into 3 different components
// The first is the one you're in, and is the entry-point

// The function below sets up the "main menu", with the 4 buttons on the side
// Also, this function is probably one of the nicer ones in this module, so you have been warned
// menu.rs contains the functions that set up the GUI for each of the 4 side-buttons
// window.rs contains those that pop up a new window

// The functions and macros in util.rs are... merely for utility purposes
// They reduce the boilerplate by quite a bit, thankfully

// I have not bothered commenting most of this junk because 90% of the code is boilerplate anyway
// I sincerely hope I will never have to see this module ever again in my life
// That is all.

pub fn build_ui(app: &gtk::Application, application: Rc<RefCell<Application>>) {
	let window = ApplicationWindow::new(app);
	let grid = Grid::new();

	let left_menu = ListBox::new();
	left_menu.set_selection_mode(SelectionMode::None);

	let button_holidays = Button::new_with_label("Holidays");
	let grid_weak = grid.downgrade();
	let window_weak = window.downgrade();
	let application_weak = Rc::downgrade(&application);
	let gui_app_weak = app.downgrade();
	button_holidays.connect_clicked(move |_| {
		menu::holidays(
			&gui_app_weak.upgrade().unwrap(),
			&grid_weak.upgrade().unwrap(),
			&window_weak.upgrade().unwrap(),
			application_weak.upgrade().unwrap(),
		);
	});
	left_menu.insert(&button_holidays, 0);

	let button_courses = Button::new_with_label("Courses");
	let grid_weak = grid.downgrade();
	let window_weak = window.downgrade();
	let application_weak = Rc::downgrade(&application);
	let gui_app_weak = app.downgrade();
	button_courses.connect_clicked(move |_| {
		menu::courses(
			&grid_weak.upgrade().unwrap(),
			&gui_app_weak.upgrade().unwrap(),
			&window_weak.upgrade().unwrap(),
			application_weak.upgrade().unwrap(),
		);
	});
	left_menu.insert(&button_courses, 0);

	let button_weekly = Button::new_with_label("Weekly");
	let grid_weak = grid.downgrade();
	let window_weak = window.downgrade();
	let application_weak = Rc::downgrade(&application);
	button_weekly.connect_clicked(move |_| {
		menu::weekly(
			&grid_weak.upgrade().unwrap(),
			application_weak.upgrade().unwrap(),
		);
		window_weak.upgrade().unwrap().show_all();
	});
	left_menu.insert(&button_weekly, 0);

	let button_main = Button::new_with_label("Main");
	let grid_weak = grid.downgrade();
	let window_weak = window.downgrade();
	let application_weak = Rc::downgrade(&application);
	button_main.connect_clicked(move |_| {
		menu::main(
			&grid_weak.upgrade().unwrap(),
			application_weak.upgrade().unwrap(),
		);
		window_weak.upgrade().unwrap().show_all();
	});
	left_menu.insert(&button_main, 0);

	let left_menu_frame = FrameBuilder::new().label("Menu").build();
	left_menu_frame.add(&left_menu);
	left_menu_frame.set_vexpand(true);

	grid.attach(&left_menu_frame, 0, 0, 1, 2);

	window.add(&grid);
	window.show_all();
}

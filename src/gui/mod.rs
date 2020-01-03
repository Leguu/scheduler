use glib::clone;
use gtk::prelude::*;
use gtk::*;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::application::Application;
use crate::clock::*;
use util::*;

#[macro_use]
mod util;
mod menu;
mod window;

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

/// The function used to create the UI. Calls a whole lot of other functions in the GUI module.
pub fn build_ui(app: &gtk::Application, application: Rc<RefCell<Application>>) {
	let window = ApplicationWindow::new(app);
	let grid = Grid::new();

	// The #1 reason why Rust makes it difficult to make UIs is in the lines below
	// References can't be passed around willy-nilly like in C-derivative languages
	// So, in order to pass around a reference to the application class, I need to downgrade it first
	// This is only necessary for when we're creating closures that are called when (e.g.) someone presses a button
	// Unfortunately, this means lots of `_weak` references and a lot of `.upgrade()`s
	// I wanted to make a macro for this sort of work, but that was also very difficult

	// As a warning to those (ie, me) who try to fix this, please increment the following counter:
	//    attempts_at_fixing_this = 3

	// Ideas: wait for rust to update its inefficient macro system?
	//        maybe just port the entire project into a different language
	//        try macros again

	// UPDATE 2020-01-02
	// There's a new update for the GTK library that allows us to use the `clone` macro.
	// This macro makes everything, so, so much easier.
	// An example of how it makes things better:

	// let button_courses = Button::new_with_label("Courses");
	// let grid_weak = grid.downgrade();
	// let window_weak = window.downgrade();
	// let application_weak = Rc::downgrade(&application);
	// let gui_app_weak = app.downgrade();
	// button_courses.connect_clicked(move |_| {
	// 	menu::courses(
	// 		&grid_weak.upgrade().unwrap(),
	// 		&gui_app_weak.upgrade().unwrap(),
	// 		&window_weak.upgrade().unwrap(),
	// 		application_weak.upgrade().unwrap(),
	// 	);
	// });

	// Becomes...

	// let button_courses = Button::new_with_label("Courses");
	// button_courses.connect_clicked(clone!(
	// 	@weak grid, @weak window, @weak application, @weak app => move |_| {
	// 		menu::courses(&grid, &app, &window, application);
	// 	}
	// ));

	let button_main = Button::new_with_label("Main");
	button_main.connect_clicked(
		clone!(@weak grid, @weak application, @weak window => move |_| {
			menu::main(&grid, application);
			window.show_all();
		}),
	);

	let button_weekly = Button::new_with_label("Weekly");
	button_weekly.connect_clicked(clone!(@weak grid, @weak window, @weak application
	=> move |_| {
		menu::weekly(&grid, application);
		window.show_all();
	}));

	let button_courses = Button::new_with_label("Courses");
	button_courses.connect_clicked(
		clone!(@weak grid, @weak window, @weak application, @weak app
		=> move |_| {
			menu::courses(&grid, &app, &window, application);
		}),
	);

	let button_holidays = Button::new_with_label("Holidays");
	button_holidays.connect_clicked(
		clone!(@weak grid, @weak window, @weak application, @weak app
		=> move |_| {
			menu::holidays(&app, &grid, &window, application,);
		}),
	);

	let button_save = Button::new_with_label("Save");
	button_save.connect_clicked(clone!(@weak application => move |_| {
		application.borrow().save("edited_scheduler.bin");
	}));

	let left_menu = ListBox::new();
	left_menu.set_selection_mode(SelectionMode::None);
	left_menu.insert(&button_main, -1);
	left_menu.insert(&button_weekly, -1);
	left_menu.insert(&button_courses, -1);
	left_menu.insert(&button_holidays, -1);
	left_menu.insert(&button_save, -1);

	let left_menu_frame = frame_with_text("Menu", &left_menu);
	left_menu_frame.set_vexpand(true);

	grid.attach(&left_menu_frame, 0, 0, 1, 2);

	window.add(&grid);
	window.show_all();
}

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use gio::prelude::*;
use serde::{Deserialize, Serialize};

use crate::application::Application;
use crate::clock::*;

// Welcome to main!
// This is pretty simple, and only exists to call other functions
fn main() {
	// The location where we want our save file to be
	// It's relative to the directory Cargo.toml is
	let location = "scheduler.bin";

	// we HAVE to surround Application in an Rc and RefCell for our GUI to work
	// Rust's ownership system makes GUI development difficult
	// Rc means "Reference Counted", allowing us to create as many references to application as we want
	// RefCell allows us to modify application from wherever we want, using only an immutable reference
	// Note that this is not allowed by the language by default, but is nonetheless necessary for GUI apps
	// Seasoned Rust developers such as myself do not use Rc and RefCell unless absolutely necessary
	let application = Rc::new(RefCell::new(application::Application::load_or_default(
		location,
	)));

	// The code below initializes the GUI application (GTK) and launches it
	// See the `gui` module (directory) for all the GUI-related components
	// Otherwise, see application.rs to understand the backend of the app
	let gui_app = gtk::Application::new(None, Default::default()).unwrap();

	// When the gui app is ready, create the UI
	gui_app.connect_activate(move |app| {
		gui::build_ui(&app, application.clone());
	});

	// And run!
	gui_app.run(&[]);
}

// Making these modules accessible
// Rust has a weird module system
// Rust is just weird in general
mod application;
mod clock;
mod course;
mod gui;
mod task;









// Random stuff:

// let mut math = Course::new("Math", "Bob", "SF14");
// math.add_time(
// 	Day::Thursday,
// 	Time::new(5, 0).unwrap(),
// 	Time::new(12, 0).unwrap(),
// );
// math.add_task("Say hi", "Idk", Date::new(2019, 11, 24).unwrap());
// math.add_task("why do I do this?", "sdf", Date::new(2020, 11, 24).unwrap());
// let mut science = Course::new("Science", "John", "SF18");
// science.add_time(
// 	Day::Thursday,
// 	Time::new(2, 0).unwrap(),
// 	Time::new(6, 0).unwrap(),
// );
// application.borrow_mut().add_course(math);
// application.borrow_mut().add_course(science);

// application.borrow_mut().save(location);

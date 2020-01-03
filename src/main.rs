// Stops compiler warnings for dead code
// TODO: Remove this once the project is complete
#![allow(dead_code)]

use gio::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::application::Application;

// Welcome to main!
// This is pretty simple, and only exists to call other functions
fn main() {
	let location = "edited_scheduler.bin";

	let application = Application::load_or_default(location);

	gui_init(application);
	// TODO: cli_init(application);
}

/// Initializer for our GUI app.
fn gui_init(application: Application) {
	// we HAVE to surround Application in an Rc and RefCell for our GUI to work
	// Rust's ownership system makes GUI development difficult
	// Rc means "Reference Counted", allowing us to create as many references to application as we want
	// RefCell allows us to modify application from wherever we want, using only an immutable reference
	// Note that this is not allowed by the language by default, but is nonetheless necessary for GUI apps
	// It's best not to use Rc and RefCell unless absolutely necessary
	let application = Rc::new(RefCell::new(application));

	// The code below initializes the GUI application (GTK) and launches it
	// See the `gui` module (directory) for all the GUI-related components
	// Otherwise, see application.rs to understand the backend of the app
	let gui_app = gtk::Application::new(None, Default::default()).unwrap();

	gui_app.connect_activate(move |app| {
		gui::build_ui(&app, application.clone());
	});

	gui_app.run(&[]);
}

// TODO:
fn cli_init(_application: Application) {
	unimplemented!()
}

// Making these modules accessible
// Rust has a weird module system
// Rust is just weird in general
pub mod application;
pub mod clock;
pub mod gui;

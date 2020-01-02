use super::*;

/// Simple macro that retrieves the text from a textbox.
/// It really shouldn't be this complicated, but the powers that be (devs of GTK) decided that this is the way.
/// If you don't know, a macro is a fancy compiler instruction.
/// In this case, `$t` will be replaced by whatever you put in.
#[macro_export]
macro_rules! get_string_from_text {
	( $t:expr ) => {{
		let buffer = $t.get_buffer().unwrap();
		let (start, end) = &buffer.get_bounds();
		buffer.get_text(start, end, false).unwrap().to_string()
		}};
}

/// For upgrading weak references.
/// Shorter than `reference.upgrade().unwrap()`.
#[macro_export]
macro_rules! up {
	( $t:expr ) => {{
		$t.upgrade().unwrap()
		}};
}

// Pops up a simple dialog, press the X to close it
pub(super) fn message_dialog(text: &str) {
	MessageDialog::new(
		None::<&Window>,
		DialogFlags::empty(),
		MessageType::Info,
		ButtonsType::None,
		text,
	)
	.run();
}

// Takes a grid and clears it, very not efficient
pub(super) fn clear(grid: &Grid) {
	for i in 1..100 {
		grid.remove_column(i);
	}
	for i in 1..100 {
		grid.remove_column(i);
	}
	for i in 1..100 {
		grid.remove_column(i);
	}
}

// Creates a grid with 3 pieces of text
pub(super) fn triple_grid(text1: &str, text2: &str, text3: &str) -> Grid {
	let grid = Grid::new();
	let t1 = text_with_default(text1, None);
	t1.set_left_margin(3);
	t1.set_right_margin(3);
	let t2 = text_with_default(text2, None);
	t2.set_left_margin(3);
	t2.set_right_margin(3);
	let t3 = text_with_default(text3, None);
	t3.set_left_margin(3);
	t3.set_right_margin(3);
	grid.attach(&t1, 0, 0, 1, 1);
	grid.attach(&t2, 1, 0, 1, 1);
	grid.attach(&t3, 2, 0, 1, 1);
	grid
}

// Create a button with a toggle button next to it
pub(super) fn toggle_button(text: &str, btn: &str) -> Grid {
	let grid = Grid::new();
	let button = Button::new_with_label(btn);
	grid.attach(&button, 0, 0, 1, 1);
	grid.attach(&text_with_default(text, None), 1, 0, 1, 1);
	grid
}

// Creates a textbox with default text in it
pub(super) fn text_with_default(text: &str, wrapmd: Option<WrapMode>) -> TextView {
	let temp = TextView::new();
	temp.get_buffer().unwrap().set_text(text);
	if let Some(md) = wrapmd {
		temp.set_wrap_mode(md);
	}
	temp
}

// Creates a frame with text inside it
pub(super) fn frame_with_text<P: IsA<Widget>>(text: &str, widget: &P) -> Frame {
	let tmp = Frame::new(Some(text));
	tmp.add(widget);
	tmp
}

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

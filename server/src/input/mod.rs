//! Input devices are the set of devices that allow users to interact with the system.
//!
//! An input device is either:
//! - A keyboard
//! - A mouse
//! - A touchpad
//! - A touchscreen

pub mod device;

use crate::poll::PollHandler;
use device::InputDevice;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;

/// The path to the directory containing evdev device files.
const EV_DEV_DIR: &str = "/dev/input";

/// A keycode.
pub type Keycode = u16;

/// Enumeration of mouse button.
#[derive(Debug)]
pub enum MouseButton {
	/// Left click.
	Button1,
	/// Right click.
	Button2,
	/// Middle click.
	Button3,
	/// Scroll up.
	Button4,
	/// Scroll down.
	Button5,
}

// TODO Specify units in doc
/// An enumeration of input actions.
#[derive(Debug)]
pub enum Input {
	/// Keyboard key press.
	KeyPress(Keycode),
	/// Keyboard key release.
	KeyRelease(Keycode),
	/// Keyboard key repeat.
	KeyRepeat(Keycode),

	/// Moving the cursor relative to the previous position.
	RelativeMove {
		/// The X delta relative to the previous position.
		delta_x: i32,
		/// The Y delta relative to the previous position.
		delta_y: i32,
	},

	/// Moving the cursor to an absolute position.
	AbsoluteMove {
		/// The X position.
		x: u32,
		/// The Y position.
		y: u32,
	},

	/// Mouse button press.
	ButtonPress(MouseButton),
	/// Mouse button release.
	ButtonRelease(MouseButton),

	/// Multitouch move event.
	MultitouchMove {
		/// The slot number.
		slot: u32,

		/// The X position.
		x: u32,
		/// The Y position.
		y: u32,
	},
}

/// Structure managing input devices.
pub struct InputManager {
	/// The list of devices.
	devs: Vec<InputDevice>,
}

impl InputManager {
	/// Creates a new instance.
	///
	/// The function registers devices to the given poll handler in order to wake it up when a
	/// device is ready for reading.
	pub fn new(poll: &mut PollHandler) -> io::Result<Self> {
		let mut devs = vec![];
		for ent in fs::read_dir(EV_DEV_DIR)? {
			let ent = ent?;
			let ent_type = ent.file_type()?;

			let exclude = ent_type.is_dir()
				|| !ent.file_name().as_bytes().starts_with(b"event");

			if exclude {
				continue;
			}

			let path = ent.path();
			let result = InputDevice::from_path(&path);

			let dev = match result {
				Ok(dev) => {
					println!("Acquired input: {}", path.display());
					dev
				}

				Err(e) => {
					eprintln!("Cannot acquire input `{}`: {}", path.display(), e);
					continue;
				}
			};

			poll.add_fd(&dev);
			devs.push(dev);
		}

		// TODO Init inotify (hotplug)

		Ok(Self {
			devs,
		})
	}

	/// Consumes and returns the next input.
	///
	/// `fds` is the list of file descriptors of the devices to read from.
	///
	/// If no input is available, the function returns None.
	pub fn next(&mut self, fds: &[i32]) -> io::Result<Option<Input>> {
		self.devs.iter_mut()
			.filter(|dev| fds.contains(&dev.as_raw_fd()))
			.filter_map(|dev| dev.next().transpose())
			.next()
			.transpose()
	}
}

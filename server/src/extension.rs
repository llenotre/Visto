//! Extensions can be loaded by the server to handle more features than only the basics.
//!
//! Each extension is represented by a shared library, and identified with a name.
//!
//! A file allows to specify associations names to shared libraries.

use crate::ctx::Context;
use crate::id_allocator::IDAllocator;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

/// The path to the list of extensions.
pub const LIST_PATH: &str = "extensions"; // TODO

lazy_static! {
	/// The list of extensions. The key is the name of the extension and the value is the path to
	/// it.
	static ref EXTENSIONS: Mutex<HashMap<String, PathBuf>> = Mutex::new(HashMap::new());

	/// The list of loaded extensions, by name.
	static ref LOADED_EXTENSIONS: Mutex<HashMap<String, Arc<Mutex<Extension>>>>
		= Mutex::new(HashMap::new());

	/// Allocator used to get a major opcode for each extension.
	static ref MAJOR_OPCODE_ALLOCATOR: Mutex<IDAllocator<u8>>
		= Mutex::new(IDAllocator::from_range_inclusive(128..=255));
	// TODO event allocator
	// TODO error allocator
}

/// Loads the list of extensions from the file at the given path.
///
/// If the file doesn't exist, the function does nothing.
pub fn load_extensions_list(path: &Path) -> Result<(), Box<dyn Error>> {
	let content = match fs::read_to_string(path) {
		Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
		Err(e) => return Err(e.into()),
		Ok(c) => c,
	};

	let mut exts = EXTENSIONS.lock().unwrap();
	exts.clear();

	// TODO Handle comments
	for line in content.split('\n') {
		let line = line.trim();
		if line.is_empty() {
			continue;
		}

		if let Some((name, path)) = line.split_once(',') {
			exts.insert(name.to_owned(), PathBuf::from(path));
		} else {
			exts.clear();
			return Err(format!("Invalid extension entry `{}`", line).into());
		}
	}

	Ok(())
}

/// A loaded extension.
pub struct Extension {
	/// The name of the extension.
	name: String,

	/// The loaded shared library.
	lib: libloading::Library,

	/// Major opcode allocated to the extension.
	major_opcode: u8,
	/// First event allocated to the extension.
	first_event: u8,
	/// First error allocated to the extension.
	first_error: u8,
}

impl Extension {
	/// Loads the extentions with the given name and path.
	///
	/// `ctx` is the current context.
	pub fn load(
		ctx: &mut Context,
		name: String,
		path: &Path,
	) -> Result<Arc<Mutex<Self>>, Box<dyn Error>> {
		let lib = unsafe { libloading::Library::new(path) }?;

		// TODO Allocate only if required by the extension
		let major_opcode = MAJOR_OPCODE_ALLOCATOR
			.lock()
			.unwrap()
			.alloc()
			.ok_or("Failed to allocate a major opcode!")?;
		let first_event = 0;
		let first_error = 0;

		let ext = Self {
			name: name.clone(),

			lib,

			major_opcode,
			first_event,
			first_error,
		};

		let init_func = unsafe {
			ext.lib
				.get::<libloading::Symbol<extern "C" fn(&mut Context, &Self) -> bool>>(b"init")
		}?;
		if !init_func(ctx, &ext) {
			// TODO Error
			todo!();
		}

		LOADED_EXTENSIONS
			.lock()
			.unwrap()
			.insert(name.clone(), Arc::new(Mutex::new(ext)));
		Ok(Self::get(&name).unwrap())
	}

	/// Returns the extension with the given name.
	///
	/// If the extension is not loaded, the function returns None.
	pub fn get(name: &str) -> Option<Arc<Mutex<Self>>> {
		LOADED_EXTENSIONS.lock().unwrap().get(name).cloned()
	}

	/// Returns the major opcode allocated to the extension.
	pub fn get_major_opcode(&self) -> u8 {
		self.major_opcode
	}

	/// Returns the first event allocated to the extension.
	pub fn get_first_event(&self) -> u8 {
		self.first_event
	}

	/// Returns the first error allocated to the extension.
	pub fn get_first_error(&self) -> u8 {
		self.first_error
	}
}

impl Drop for Extension {
	fn drop(&mut self) {
		let fini_func: Result<libloading::Symbol<extern "C" fn()>, _> =
			unsafe { self.lib.get(b"fini") };
		if let Ok(fini_func) = fini_func {
			fini_func();
		}

		// TODO Free major opcode
		// TODO Free first event
		// TODO Free first error
	}
}

/// Queries for the extension with the given name.
///
/// `ctx` is the current context.
///
/// If not loaded, the function tries to load the module with the given name.
/// If the extension doesn't exist, the function returns None.
pub fn query(
	ctx: &mut Context,
	name: &str,
) -> Result<Option<Arc<Mutex<Extension>>>, Box<dyn Error>> {
	if let Some(ext) = Extension::get(name) {
		return Ok(Some(ext));
	}

	match EXTENSIONS.lock().unwrap().get(name) {
		Some(ext_path) => {
			// Loading the extension
			let ext = Extension::load(ctx, name.to_owned(), Path::new(ext_path))?;
			Ok(Some(ext))
		}

		None => Ok(None),
	}
}

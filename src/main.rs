mod walkdir;
mod statfs;
use walkdir::dir_iter;
use std::ffi::CString;
mod utils;
mod disk;
mod args;
mod handlers;
use std::{io, env};
use args::{parse_args, EntryType};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::args::ArgumentActions;

// Do not run dir_iter while -d was passed
static ARG_DISPLAY_BARE_SET: AtomicBool = AtomicBool::new(false);
// Format names in lowercase if -l was passed
static ARG_DISPLAY_LOWERCASE: AtomicBool = AtomicBool::new(false);

fn main() -> io::Result<()>
{
	let parsed = parse_args();
	let mut path = String::new();

	if let Some((paths, named)) = parsed
	{
		let mut entry_type = EntryType::Unknown;
		if paths.is_none()
		{
			// if path is not provided, defaulting to current directory
			path = env::current_dir()?.as_path().to_str().unwrap().to_string();
			entry_type = EntryType::Dir;

			if named.is_some()
			{
				for arg in named.unwrap()
				{
					match arg
					{
						ArgumentActions::DisplayHelp => handlers::display_help(),
						ArgumentActions::DisplayBare =>
						{
							ARG_DISPLAY_BARE_SET.store(true, Ordering::Relaxed);
							handlers::display_bare(&path).unwrap()
						},
						ArgumentActions::DisplayLowercase =>
						{
							ARG_DISPLAY_LOWERCASE.store(true, Ordering::Relaxed);
						}
						ArgumentActions::Unknown => {}
					}		
				}
			}

			if !ARG_DISPLAY_BARE_SET.load(Ordering::Relaxed)
			{
				let c_path = CString::new(path).unwrap();
				dir_iter(&c_path, entry_type, &ARG_DISPLAY_LOWERCASE);
			}
		}
		else
		{
			// Otherwise, running dir_iter or arguments through all assigned paths
			// Will be reworked later accordingly to actual "dir" behaviour
			for path_entry in paths.unwrap()
			{
				let mut entry_p = path_entry.0;
				if named.is_some()
				{
					for arg in named.as_ref().unwrap()
					{
						match arg
						{
							ArgumentActions::DisplayHelp => handlers::display_help(),
							ArgumentActions::DisplayBare =>
							{
								ARG_DISPLAY_BARE_SET.store(true, Ordering::Relaxed);
								handlers::display_bare(&entry_p).unwrap()
							},
							ArgumentActions::DisplayLowercase =>
							{
								ARG_DISPLAY_LOWERCASE.store(true, Ordering::Relaxed);
							}
							ArgumentActions::Unknown => {}
						}		
					}
				}

				if !ARG_DISPLAY_BARE_SET.load(Ordering::Relaxed)
				{
					let c_path = CString::new(entry_p).unwrap();
					dir_iter(&c_path, path_entry.1, &ARG_DISPLAY_LOWERCASE);
				}
			}
		}
	}
	else
	{
		// No arguments passed, defaulting to current directory and just running plain dir_iter
		path = env::current_dir()?.as_path().to_str().unwrap().to_string();
		let entry_type = EntryType::Dir;

		let c_path = CString::new(path).unwrap();
		dir_iter(&c_path, entry_type, &ARG_DISPLAY_LOWERCASE);
	}
	Ok(())
}

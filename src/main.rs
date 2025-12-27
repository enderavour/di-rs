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

use crate::args::ArgumentActions;


fn main() -> io::Result<()>
{
	let parsed = parse_args();
	let mut path = String::new();

	if let Some((args, entp)) = (*parsed).downcast_ref::<(Option<String>, EntryType)>()
	{
		let mut entry_type = *entp;
		if args.is_none()
		{
			// if path is not provided, defaulting to current directory
			if let EntryType::EmptyPath = entry_type
			{
				path = env::current_dir()?.as_path().to_str().unwrap().to_string();
				entry_type = EntryType::Dir;
			}
		
		}
		else
		{
			// Otherwise, assigning provided path to directory
			path = args.clone().unwrap();
		}
	
		let directory_path = CString::new(path).unwrap();
		dir_iter(&directory_path, entry_type)
	}
	else if let Some(arg) = (*parsed).downcast_ref::<ArgumentActions>()
	{
		match arg
		{
			ArgumentActions::DisplayHelp => handlers::display_help(),
			ArgumentActions::DisplayBare => handlers::display_bare(&env::current_dir()?.as_path().to_str().unwrap().to_string()).unwrap(),
			ArgumentActions::Unknown => {}
		}
	}
	Ok(())
}

mod walkdir;
mod statfs;
use walkdir::dir_iter;
use std::ffi::CString;
mod utils;
mod disk;
mod args;
use std::{io, env};
use args::{Args, parse_args, EntryType};
use clap::Parser;

fn main() -> io::Result<()> 
{
	let parsed = Args::parse();
	let mut path = String::new();

	let (args, mut entry_type) = parse_args(&parsed);

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
		path = args.unwrap().path.unwrap();
	}
	
	let directory_path = CString::new(path).unwrap();
	dir_iter(&directory_path, entry_type);
	Ok(())
}

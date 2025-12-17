mod walkdir;
mod statfs;
use walkdir::dir_iter;
use std::ffi::CString;
mod utils;
mod disk;
use std::{io, env};

fn main() -> io::Result<()> 
{
	let directory_path = CString::new(env::current_dir()?.as_path().to_str().unwrap())?;
	dir_iter(&directory_path);
	Ok(())
}

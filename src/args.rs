use clap::Parser;
use std::fs;

#[derive(Parser, Debug, Clone)]
pub struct Args
{
	pub path: Option<String>
}

pub enum EntryType
{
	Unknown,
	Dir,
	EmptyPath,
	File
}

pub fn parse_args(arg: &Args) -> (Option<Args>, EntryType)
{
	if arg.path.is_none()
	{
		return (None, EntryType::EmptyPath);
	}
	
	let path_info = fs::metadata(arg.path.clone().unwrap());

	if !path_info.is_ok() { return (None, EntryType::Unknown) }

	let path = path_info.unwrap();
	
	let mut entry_type = EntryType::Dir;
	if path.is_file()
	{
		entry_type = EntryType::File;
	}

	(Some(arg.clone()), entry_type)
}


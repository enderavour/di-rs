use std::fs;
use std::env;
use std::any::Any;

pub fn parse_args() -> Box<dyn Any>
{
	let args = env::args().collect::<Vec<String>>();

	let mut res: Box<dyn Any> = Box::new({});

	if args.len() > 1
	{
		if args[1].starts_with("-")
		{
			res = Box::new(parse_named(&args[1]));
		}
		else
		{
			res = Box::new(parse_path(&args[1]));
		}
		return res;
	}

	// Passing empty string to trigger current path printing
	res = Box::new(parse_path(&String::new()));
	
	res
}

#[derive(Clone, Copy)]
pub enum EntryType
{
	Unknown,
	Dir,
	EmptyPath,
	File
}

pub enum ArgumentActions
{
	DisplayHelp,
	Unknown,
}

fn parse_path(arg: &String) -> (Option<String>, EntryType)
{
	if arg.is_empty()
	{
		return (None, EntryType::EmptyPath);
	}
	
	let path_info = fs::metadata(arg);

	if !path_info.is_ok() { return (Some(arg.clone()), EntryType::Unknown) }

	let path = path_info.unwrap();
	
	let mut entry_type = EntryType::Dir;
	if path.is_file()
	{
		entry_type = EntryType::File;
	}

	(Some(arg.clone()), entry_type)
}

fn parse_named(arg: &String) -> ArgumentActions
{
	// Skip /
	let parsed = &arg[1..];

	let action = match parsed
	{
		"?" => ArgumentActions::DisplayHelp,
		&_ => ArgumentActions::Unknown
	};

	action
}

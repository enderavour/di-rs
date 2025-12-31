use std::fs;
use std::env;
use std::cmp::PartialEq;

pub fn parse_args() -> Option<(Option<Vec<(String, EntryType)>>, Option<Vec<ArgumentActions>>)>
{
	let args = env::args().skip(1).collect::<Vec<String>>();

	// No arguments were passed
	if args.is_empty() { return None; }

	// Finding path arguments
	let path_args = args.iter().filter(|arg| !(*arg).starts_with("-")).collect::<Vec<&String>>();

	// Evaluate path existence and entry types
	let parsed_paths = path_args.iter().map(|path| parse_path(*path)).collect::<Vec<(String, EntryType)>>();

	// Parsing named arguments
	let named_args = parse_named(&args);

	let mut return_tuple = (Some(parsed_paths), Some(named_args.clone()));

	if path_args.is_empty()
	{
		return_tuple.0 = None;
	}

	if named_args.is_empty()
	{
		return_tuple.1 = None;
	}
	
	Some(return_tuple)
}

#[derive(Clone, Copy)]
pub enum EntryType
{
	Unknown,
	Dir,
	EmptyPath,
	File
}

#[derive(Clone, Copy, PartialEq)]
pub enum ArgumentActions
{
	DisplayHelp, // -?
	DisplayBare, // -b
	DisplayLowercase, // -l
	Unknown,
}

fn parse_path(arg: &String) -> (String, EntryType)
{
	if arg.is_empty()
	{
		return (String::new(), EntryType::EmptyPath);
	}
	
	let path_info = fs::metadata(arg);

	if !path_info.is_ok() { return (arg.clone(), EntryType::Unknown) }

	let path = path_info.unwrap();
	
	let mut entry_type = EntryType::Dir;
	if path.is_file()
	{
		entry_type = EntryType::File;
	}

	(arg.clone(), entry_type)
}

fn parse_named(args: &Vec<String>) -> Vec<ArgumentActions>
{
	let mut argument_actions = Vec::new();
		
	for arg in args
	{
		if arg.starts_with("-")
		{
			// Skip /	
			let parsed = &arg[1..];
			
			let action = match parsed
			{
				"?" => ArgumentActions::DisplayHelp,
				"b" => ArgumentActions::DisplayBare,
				"l" => ArgumentActions::DisplayLowercase,
				&_ => ArgumentActions::Unknown
			};

			// Insert /l as the first in order to modify entries
			if action == ArgumentActions::DisplayLowercase
			{
				argument_actions.insert(0, action);
				continue;
			}
			
			argument_actions.push(action);
		}
	}
	argument_actions
}

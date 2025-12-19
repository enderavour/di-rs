use std::fs;
use std::io;
use chrono::{DateTime, Local};
use std::ffi::CString;
use crate::utils;
use std::sync::Mutex;
use num_format::{format::Locale, ToFormattedString};

pub static TOTAL_FILE_SIZE: Mutex<u64> = Mutex::new(0);
pub static TOTAL_DIR_SIZE: Mutex<u64> = Mutex::new(0);

fn dir_util_walk_dir(path: &str) -> io::Result<(u32, u32)>
{
	let mut file_count = 0;
	let mut dir_count = 0;

	// Manually adding . and .. entries
	let cur_dir_metadata = fs::metadata(".")?;
	let parent_dir_metadata = fs::metadata("..")?;

	let cur_modified: DateTime<Local> = cur_dir_metadata.modified()?.into();
	let parent_modified: DateTime<Local> = parent_dir_metadata.modified()?.into();
 
	println!("{} {}    <DIR>                   .", cur_modified.format("%d.%m.%Y"), cur_modified.format("%H:%M"));
	println!("{} {}    <DIR>                   ..", parent_modified.format("%d.%m.%Y"), parent_modified.format("%H:%M"));
	
	for obj  in fs::read_dir(path)?
	{
		let entry = obj?;
		let entry_metadata = entry.metadata()?;

		let dt_local: DateTime<Local> = entry_metadata.modified()?.into();
		
		if entry_metadata.is_dir()
		{			
			println!(
				"{} {}    <DIR>                   {}",
				dt_local.format("%d.%m.%Y"),
				dt_local.format("%H:%M"),
				entry.path().into_os_string().into_string().unwrap()
			);
			*TOTAL_DIR_SIZE.lock().unwrap() += entry_metadata.len();
			dir_count += 1;
		}
		else
		{
			println!(
				"{} {}              {:>12}  {}",
				dt_local.format("%d.%m.%Y"),
				dt_local.format("%H:%M"),
				entry_metadata.len(),
				entry.path().into_os_string().into_string().unwrap()
			);
			*TOTAL_FILE_SIZE.lock().unwrap() += entry_metadata.len();
			file_count += 1;
		}
	}
	Ok((dir_count, file_count))
}

pub fn dir_iter(path: &CString)
{
	if let Ok((ref part_unstr, ref mnt_unstr)) = utils::dname_and_mp(path)
	{
		// Stripping the returned buffers
		let part = utils::strip_buf_zeros(part_unstr);
		let  mnt = utils::strip_buf_zeros(mnt_unstr);
	
		// Getting drive name
		let mut dev = part.clone();
		let _ = dev.split_off(part.len() - 2);
		
		println!(" Volume in drive {} is {}", part, mnt);
		println!(" Volume Serial Number is {}\n", utils::d_serial_num(&dev));
		println!(" Directory of {}\n", path.clone().into_string().unwrap());

		let (dir_count, file_count) = dir_util_walk_dir(path.clone().into_string().unwrap().as_str()).unwrap();

		println!("      {:>10} {:<8} {:>15} bytes",
				 file_count, "File(s)",  &(*TOTAL_FILE_SIZE.lock().unwrap().to_formatted_string(&Locale::fr)));
		// Calculating free space on the disk based on mountpoint
		println!("      {:>10} {:<8} {:>15} bytes free",
				 dir_count, "Dir(s)", &(utils::d_free_space(&CString::new(mnt).unwrap()).to_formatted_string(&Locale::fr)));
	}				  
}

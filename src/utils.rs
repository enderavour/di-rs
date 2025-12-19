use std::ffi::CString;
use crate::statfs;
use std::{mem::zeroed, process::exit, io};
use crate::disk;
use std::fs::{self, File};
use std::os::fd::AsRawFd;

// Function to get drive name and mountpoint
pub fn dname_and_mp(path: &CString) -> io::Result<(String, String)>
{
    let mut st: statfs::statfs = unsafe { zeroed() };
	// For receiving drive name and mount point
	let res = unsafe { statfs::statfs(path.as_ptr(), &mut st) };

	if res != 0
	{
		eprintln!("statfs: {}", io::Error::last_os_error().raw_os_error().unwrap());
		exit(res);
	}
	
	let dev = str::from_utf8(&st.f_mntfromname).unwrap();
	let mnt = str::from_utf8(&st.f_mnttonname).unwrap(); 
    Ok((dev.to_owned(), mnt.to_owned()))
}

// Function to get drive serial number. 
pub fn d_serial_num(disk: &str) -> String
{
	let mut buf = [0u8; disk::DISK_IDENT_SIZE];
	let dev = File::open(disk).unwrap();
	let outp = unsafe { disk::ioctl(dev.as_raw_fd(), disk::DIOCGIDENT, buf.as_mut_ptr()) };
	if outp == -1
	{
		eprintln!("ioctl(): {}", io::Error::last_os_error().raw_os_error().unwrap());
		exit(-1);
	}

	String::from_utf8_lossy(&buf).to_string()
}

pub fn d_free_space(disk: &CString) -> u64
{
	let mut st: statfs::statfs = unsafe { zeroed() };
	let res = unsafe { statfs::statfs(disk.as_ptr(), &mut st) };
	if res != 0
	{
		eprintln!("statfs(): {}", io::Error::last_os_error().raw_os_error().unwrap());
		exit(-1);
	}

	st.f_bsize * (st.f_bavail as u64)
}

// Remove zeros in buffers, filled by system calls ioctl/statfs
pub fn strip_buf_zeros(line: &String) -> String
{
	let mut stripped_string = String::new();
	for ch in line.chars()
	{
		if ch == '\0' { break; }
		stripped_string.push(ch);
	}
	stripped_string
}



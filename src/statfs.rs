
const MFSNAMELEN: usize = 16;
const MNAMELEN: usize = 1024;

#[repr(C)]
struct fsid_t
{
	val: [i32; 2]
}


#[repr(C)]
pub struct statfs
{
	f_version: u32,
	f_type: u32,
	f_flags: u64,
	pub f_bsize: u64,
	f_iosize: u64,
	f_blocks: u64,
	f_bfree: u64,
	pub f_bavail: i64,
	f_files: u64,
	f_ffree: i64,
	f_syncwrites: u64,
	f_asyncwrites: u64,
	f_syncreads: u64,
	f_asyncreads: u64,
	f_spare: [u64; 10],
	f_namemax: u32,
	f_owner: u32,
	f_fsid: fsid_t,
	f_charspare: [u8; 80],
	f_fstypename: [u8; MFSNAMELEN],
	pub f_mntfromname: [u8; MNAMELEN],
	pub f_mnttonname: [u8; MNAMELEN]
}

unsafe extern "C" {
	pub unsafe fn statfs(path: *const i8, buf: *mut statfs) -> i32;
}

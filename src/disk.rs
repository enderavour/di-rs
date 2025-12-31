use std::ffi::{c_int, c_ulong};

pub const DISK_IDENT_SIZE: usize = 256;

macro_rules! _ioc
{
    ($inout:expr, $group:expr, $num:expr, $len:expr) => {
        ($inout | (($len & 0x1FFF) << 16) | (($group) << 8) | ($num))
    }
}

macro_rules! _ior
{
    ($g:expr, $n:expr, $t:expr) => {
        _ioc!(0x40000000, $g, $n, $t)
	}
}

pub const DIOCGIDENT: u64 = _ior!(0x64, 137, 256);

unsafe extern "C"
{
    pub unsafe fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

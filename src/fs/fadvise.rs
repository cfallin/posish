use crate::zero_ok;
use io_experiment::{AsFd, BorrowedFd};
#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
)))]
use libc::posix_fadvise as libc_posix_fadvise;
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
))]
use libc::posix_fadvise64 as libc_posix_fadvise;
use std::{convert::TryInto, io};
use unsafe_io::os::posish::AsRawFd;

/// `POSIX_FADV_*` constants.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum Advice {
    /// `POSIX_FADV_NORMAL`
    Normal = libc::POSIX_FADV_NORMAL,

    /// `POSIX_FADV_SEQUENTIAL`
    Sequential = libc::POSIX_FADV_SEQUENTIAL,

    /// `POSIX_FADV_RANDOM`
    Random = libc::POSIX_FADV_RANDOM,

    /// `POSIX_FADV_NOREUSE`
    NoReuse = libc::POSIX_FADV_NOREUSE,

    /// `POSIX_FADV_WILLNEED`
    WillNeed = libc::POSIX_FADV_WILLNEED,

    /// `POSIX_FADV_DONTNEED`
    DontNeed = libc::POSIX_FADV_DONTNEED,
}

/// `posix_fadvise(fd, offset, len, advice)`
#[inline]
pub fn fadvise<Fd: AsFd>(fd: &Fd, offset: u64, len: u64, advice: Advice) -> io::Result<()> {
    let fd = fd.as_fd();
    unsafe { _fadvise(fd, offset, len, advice) }
}

unsafe fn _fadvise(fd: BorrowedFd<'_>, offset: u64, len: u64, advice: Advice) -> io::Result<()> {
    if let (Ok(offset), Ok(len)) = (offset.try_into(), len.try_into()) {
        zero_ok(libc_posix_fadvise(
            fd.as_raw_fd() as libc::c_int,
            offset,
            len,
            advice as libc::c_int,
        ))?;
    }

    // If the offset or length can't be converted, ignore the advice, as it
    // isn't likely to be useful in that case.
    Ok(())
}

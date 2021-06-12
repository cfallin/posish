//! `Dir`, `Entry`, and `SeekLoc`.

#[cfg(all(libc, target_os = "android"))]
use crate::fs::android::seekdir as libc_seekdir;
use crate::fs::FileType;
use io_experiment::{IntoFd, OwnedFd};
#[cfg(all(libc, not(target_os = "android")))]
use libc::seekdir as libc_seekdir;
#[cfg(not(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "linux"
)))]
use libc::{dirent as libc_dirent, readdir as libc_readdir};
#[cfg(all(
    libc,
    any(
        target_os = "android",
        target_os = "emscripten",
        target_os = "l4re",
        target_os = "linux"
    )
))]
use libc::{dirent64 as libc_dirent, readdir64 as libc_readdir};
use std::{ffi::CStr, io, ptr};
#[cfg(target_os = "wasi")]
use std::{ffi::CString, mem::MaybeUninit};
use unsafe_io::{
    os::posish::{AsRawFd, RawFd},
    OwnsRaw,
};
#[cfg(libc)]
use {
    errno::{set_errno, Errno},
    std::convert::TryInto,
    unsafe_io::os::posish::IntoRawFd,
};
#[cfg(linux_raw)]
use {
    io_experiment::{AsFd, AsFilelike},
    linux_raw_sys::general::linux_dirent64,
    std::ffi::CString,
    std::mem::size_of,
    std::os::raw::{c_char, c_ulong, c_ushort},
};

/// `DIR*`
#[cfg(libc)]
#[repr(transparent)]
pub struct Dir(ptr::NonNull<libc::DIR>);

/// `DIR*`
#[cfg(linux_raw)]
pub struct Dir {
    fd: OwnedFd,
    buf: Vec<u8>,
    pos: usize,
    next: Option<u64>,
}

impl Dir {
    /// Construct a `Dir`, assuming ownership of the file descriptor.
    #[inline]
    pub fn from<F: IntoFd>(fd: F) -> io::Result<Self> {
        let fd = fd.into_fd();
        Self::_from(fd)
    }

    /// Construct a `Dir`, assuming ownership of the file descriptor.
    ///
    /// # Safety
    ///
    /// This accepts any type that implements `IntoRawFd`, however `IntoRawFd`
    /// itself doesn't guarantee that the handle is valid. Callers must ensure
    /// that the handle is valid.
    #[inline]
    pub fn from_into_raw_fd<F: IntoFd>(fd: F) -> io::Result<Self> {
        let fd = fd.into_fd();
        Self::_from(fd)
    }

    #[cfg(libc)]
    fn _from(fd: OwnedFd) -> io::Result<Self> {
        let raw = fd.into_raw_fd() as libc::c_int;
        unsafe {
            let d = libc::fdopendir(raw);
            if let Some(d) = ptr::NonNull::new(d) {
                Ok(Self(d))
            } else {
                let e = io::Error::last_os_error();
                libc::close(raw);
                Err(e)
            }
        }
    }

    #[cfg(linux_raw)]
    fn _from(fd: OwnedFd) -> io::Result<Self> {
        // Buffer size chosen by wild guess.
        let buf = vec![0; 1024];
        let pos = buf.len();
        Ok(Self {
            fd,
            buf,
            pos,
            next: None,
        })
    }

    /// `seekdir(self, loc)`
    #[cfg(libc)]
    #[inline]
    pub fn seek(&mut self, loc: SeekLoc) {
        unsafe { libc_seekdir(self.0.as_ptr(), loc.0) }
    }

    /// `seekdir(self, loc)`
    #[cfg(linux_raw)]
    #[inline]
    pub fn seek(&mut self, loc: SeekLoc) {
        self.pos = self.buf.len();
        self.next = Some(loc.to_raw());
    }

    /// `rewinddir(self)`
    #[cfg(libc)]
    #[inline]
    pub fn rewind(&mut self) {
        unsafe { libc::rewinddir(self.0.as_ptr()) }
    }

    /// `rewinddir(self)`
    #[cfg(linux_raw)]
    #[inline]
    pub fn rewind(&mut self) {
        self.pos = self.buf.len();
        self.next = Some(0);
    }

    /// `readdir(self)`, where `None` means the end of the directory.
    #[cfg(libc)]
    pub fn read(&mut self) -> Option<io::Result<Entry>> {
        set_errno(Errno(0));
        let dirent = unsafe { libc_readdir(self.0.as_ptr()) };
        if dirent.is_null() {
            let curr_errno = io::Error::last_os_error();
            if curr_errno.raw_os_error() == Some(0) {
                // We successfully reached the end of the stream.
                None
            } else {
                // `errno` is unknown or non-zero, so an error occurred.
                Some(Err(curr_errno))
            }
        } else {
            // We successfully read an entry.
            Some(Ok(unsafe {
                Entry {
                    #[cfg(not(target_os = "wasi"))]
                    dirent: *dirent,

                    // TODO: When WASI gains a `d_loc` field, update `Entry::seek_loc`.
                    #[cfg(target_os = "wasi")]
                    dirent: libc_dirent {
                        d_ino: (*dirent).d_ino,
                        d_type: (*dirent).d_type,
                        d_name: MaybeUninit::uninit().assume_init(),
                    },

                    #[cfg(target_os = "wasi")]
                    name: CStr::from_ptr((*dirent).d_name.as_ptr()).to_owned(),
                }
            }))
        }
    }

    /// `readdir(self)`, where `None` means the end of the directory.
    #[cfg(linux_raw)]
    pub fn read(&mut self) -> Option<io::Result<Entry>> {
        if let Some(next) = self.next.take() {
            match std::io::Seek::seek(
                &mut *self.fd.as_filelike_view::<std::fs::File>(),
                std::io::SeekFrom::Start(next),
            ) {
                Ok(_) => (),
                Err(err) => return Some(Err(err)),
            }
        }

        // Compute linux_dirent64 field offsets.
        let z = linux_dirent64 {
            d_ino: 0,
            d_off: 0,
            d_type: 0,
            d_reclen: 0,
            d_name: Default::default(),
        };
        let base = &z as *const _ as usize;
        let offsetof_d_reclen = (&z.d_reclen as *const _ as usize) - base;
        let offsetof_d_name = (&z.d_name as *const _ as usize) - base;
        let offsetof_d_off = (&z.d_off as *const _ as usize) - base;
        let offsetof_d_ino = (&z.d_ino as *const _ as usize) - base;
        let offsetof_d_type = (&z.d_type as *const _ as usize) - base;

        // Test if we need more entries, and if so, read more.
        if self.buf.len() - self.pos < size_of::<linux_dirent64>() {
            match self.read_more()? {
                Ok(()) => (),
                Err(e) => return Some(Err(e)),
            }
        }

        // We successfully read an entry. Extract the fields.
        let pos = self.pos;
        let dirent = self.buf[pos..].as_ptr();
        let name_start = pos + offsetof_d_name;
        unsafe {
            let reclen =
                ptr::read_unaligned(dirent.add(offsetof_d_reclen) as *const c_ushort) as usize;
            assert!(self.buf.len() - pos >= reclen);
            self.pos += reclen;

            let name = CStr::from_ptr(self.buf[name_start..].as_ptr() as *const c_char);
            let name = name.to_owned();
            assert!(name.as_bytes().len() <= self.buf.len() - name_start);

            let d_ino = ptr::read_unaligned(dirent.add(offsetof_d_ino) as *const c_ulong) as u64;
            let d_off = ptr::read_unaligned(dirent.add(offsetof_d_off) as *const c_ulong) as u64;
            let d_type = ptr::read_unaligned(dirent.add(offsetof_d_type));
            Some(Ok(Entry {
                d_ino,
                d_off: d_off - (reclen as u64),
                d_type,
                name: name.to_owned(),
            }))
        }
    }

    #[cfg(linux_raw)]
    fn read_more(&mut self) -> Option<io::Result<()>> {
        self.buf.resize(self.buf.capacity() + 1, 0);
        self.pos = 0;
        let nread = match crate::linux_raw::getdents(self.fd.as_fd(), &mut self.buf) {
            Ok(nread) => nread,
            Err(err) => return Some(Err(err)),
        };
        self.buf.resize(nread, 0);
        if nread == 0 {
            None
        } else {
            Some(Ok(()))
        }
    }
}

#[cfg(libc)]
unsafe impl Send for Dir {}
#[cfg(libc)]
unsafe impl Sync for Dir {}

#[cfg(libc)]
impl AsRawFd for Dir {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        unsafe { libc::dirfd(self.0.as_ptr()) as RawFd }
    }
}

#[cfg(linux_raw)]
impl AsRawFd for Dir {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

// Safety: `Dir` owns its handle.
unsafe impl OwnsRaw for Dir {}

#[cfg(libc)]
impl Drop for Dir {
    #[inline]
    fn drop(&mut self) {
        unsafe { libc::closedir(self.0.as_ptr()) };
    }
}

impl Iterator for Dir {
    type Item = io::Result<Entry>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Self::read(self)
    }
}

/// `struct dirent`
#[cfg(libc)]
#[derive(Debug)]
pub struct Entry {
    dirent: libc_dirent,

    #[cfg(target_os = "wasi")]
    name: CString,
}

/// `struct dirent`
#[cfg(linux_raw)]
#[derive(Debug)]
pub struct Entry {
    d_ino: u64,
    d_off: u64,
    d_type: u8,
    name: CString,
}

#[cfg(libc)]
impl Entry {
    /// Returns the file name of this directory entry.
    #[inline]
    pub fn file_name(&self) -> &CStr {
        #[cfg(not(target_os = "wasi"))]
        unsafe {
            CStr::from_ptr(self.dirent.d_name.as_ptr())
        }

        #[cfg(target_os = "wasi")]
        &self.name
    }

    /// Returns the type of this directory entry.
    #[inline]
    pub fn file_type(&self) -> FileType {
        FileType::from_dirent_d_type(self.dirent.d_type)
    }

    /// Return the inode number of this directory entry.
    #[cfg(not(any(target_os = "freebsd", target_os = "netbsd")))]
    #[inline]
    pub fn ino(&self) -> u64 {
        self.dirent.d_ino
    }

    /// Return the inode number of this directory entry.
    #[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
    #[inline]
    pub fn ino(&self) -> u64 {
        #[allow(clippy::useless_conversion)]
        self.dirent.d_fileno.into()
    }

    /// Return a cookie indicating the location of this entry, for use with [`Dir::seek`].
    ///
    /// [`Dir::seek`]: struct.Dir.html#method.seek
    ///
    /// TODO: Use `d_loc` on WASI once we have libc support.
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    #[inline]
    pub fn seek_loc(&self) -> io::Result<SeekLoc> {
        let off_i64: i64 = self.dirent.d_off;
        unsafe { SeekLoc::from_raw(off_i64 as u64) }
    }
}

#[cfg(linux_raw)]
impl Entry {
    /// Returns the file name of this directory entry.
    #[inline]
    pub fn file_name(&self) -> &CStr {
        &self.name
    }

    /// Returns the type of this directory entry.
    #[inline]
    pub fn file_type(&self) -> FileType {
        FileType::from_dirent_d_type(self.d_type)
    }

    /// Return the inode number of this directory entry.
    #[inline]
    pub fn ino(&self) -> u64 {
        self.d_ino
    }

    /// Return a cookie indicating the location of this entry, for use with [`Dir::seek`].
    ///
    /// [`Dir::seek`]: struct.Dir.html#method.seek
    #[inline]
    pub fn seek_loc(&self) -> io::Result<SeekLoc> {
        unsafe { SeekLoc::from_raw(self.d_off) }
    }
}

/// A location for use with [`Dir::seek`].
///
/// [`Dir::seek`]: struct.Dir.html#method.seek
#[cfg(libc)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct SeekLoc(std::os::raw::c_long);

/// A location for use with [`Dir::seek`].
///
/// [`Dir::seek`]: struct.Dir.html#method.seek
#[cfg(linux_raw)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct SeekLoc(u64);

#[cfg(libc)]
impl SeekLoc {
    /// Return the location encoded as a `u64`. Note that this value is meant to
    /// be opaque, and applications shouldn't do anything with it except call
    /// `SeekLoc::from_raw`.
    #[inline]
    #[allow(clippy::useless_conversion)]
    pub fn to_raw(&self) -> u64 {
        i64::from(self.0) as u64
    }

    /// Construct a new `SeekLoc` from a value returned by `SeekLoc::to_raw`.
    ///
    /// # Safety
    ///
    /// The passed-in `loc` value must be a value returned from
    /// `SeekLoc::to_raw`.
    #[inline]
    pub unsafe fn from_raw(loc: u64) -> io::Result<Self> {
        Ok(Self(loc.try_into().map_err(|_convert_err| {
            io::Error::from_raw_os_error(libc::EINVAL)
        })?))
    }
}

#[cfg(linux_raw)]
impl SeekLoc {
    /// Return the location encoded as a `u64`. Note that this value is meant to
    /// be opaque, and applications shouldn't do anything with it except call
    /// `SeekLoc::from_raw`.
    #[inline]
    pub fn to_raw(&self) -> u64 {
        self.0
    }

    /// Construct a new `SeekLoc` from a value returned by `SeekLoc::to_raw`.
    ///
    /// # Safety
    ///
    /// The passed-in `loc` value must be a value returned from
    /// `SeekLoc::to_raw`.
    #[inline]
    pub unsafe fn from_raw(loc: u64) -> io::Result<Self> {
        Ok(Self(loc))
    }
}

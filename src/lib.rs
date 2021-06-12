//! `posish` provides safe wrappers to `libc` functions. The current focus is
//! on the functionality needed by [`cap-std`] and [`system-interface`] that
//! isn't provided by [`std`] or [`getrandom`].
//!
//! TODO: Update the comments to mention direct syscall mode on Linux!
//!
//! The wrappers perform the following tasks:
//!  - Error values are translated to `Result`s.
//!  - Out-parameters are translated to return values.
//!  - Path arguments can by any kind of string type.
//!  - File descriptors are passed in through arguments implementing
//!    [`AsFd`] instead of as bare integers and returned as [`OwnedFd`]s.
//!  - Constants use `enum`s and [`bitflags`] types.
//!  - Multiplexed functions (eg. `fcntl`, `ioctl`, etc.) are de-multiplexed.
//!  - Variadic functions (eg. `openat`, etc.) are presented as non-variadic.
//!  - Functions and types which need `64` suffixes to enable large-file
//!    support are used automatically.
//!  - Behaviors that depend on the sizes of C types like `long` are hidden.
//!  - File offsets and sizes are presented as `i64` and `u64` rather than
//!    `off_t`.
//!  - In some places, more human-friendly and less historical-accident names
//!    are used.
//!
//! Things they don't do include:
//!  - Emulating functions that aren't natively supported on a platform.
//!  - Detecting whether functions are supported at runtime.
//!  - Hiding significant differences between platforms.
//!  - Hiding ambient authorities.
//!  - Imposing sandboxing features such as filesystem path or network address
//!    sandboxing.
//!
//! See [`cap-std`] and [`system-interface`] for libraries which do hide
//! ambient authorities and perform sandboxing.
//!
//! # Safety
//!
//! This library uses the [io-lifetimes crate] to manage all OS resource
//! handles, ensuring I/O safety.
//!
//! [`cap-std`]: https://crates.io/crates/cap-std
//! [`system-interface`]: https://crates.io/crates/system-interface
//! [`std`]: https://doc.rust-lang.org/std/
//! [`getrandom`]: https://crates.io/crates/getrandom
//! [`std::fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
//! [`bitflags`]: https://crates.io/crates/bitflags
//! [io-lifetimes crate]: https://crates.io/crates/io-lifetimes

#![deny(missing_docs)]
#![cfg_attr(linux_raw, feature(asm))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

#[cfg(all(libc, not(target_os = "wasi")))]
#[macro_use]
mod weak;

pub mod fs;
pub mod io;
#[cfg(not(any(target_os = "wasi", target_os = "redox")))] // WASI doesn't support `net` yet.
pub mod net;
pub mod path;
pub mod process;
pub mod time;

#[cfg(linux_raw)]
mod linux_raw;

/// Given a `libc` return value, translate `0` into `Ok(())` and any other
/// value to an `Err` with the error from `errno`.
#[cfg(libc)]
fn zero_ok<T: LibcResult>(t: T) -> std::io::Result<()> {
    if t.is_zero() {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// Given a `libc` return value, translate `-1` into an `Err` with the error
/// from `errno`, and any other value to an `Ok` containing the value.
#[cfg(libc)]
fn negone_err<T: LibcResult>(t: T) -> std::io::Result<T> {
    if t.is_negone() {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

/// Given a `libc` return value, translate a negative value into an `Err` with
/// the error from `errno`, and any other value to an `Ok` containing the
/// value.
#[cfg(libc)]
#[allow(dead_code)]
fn negative_err<T: LibcResult>(t: T) -> std::io::Result<()> {
    if t.is_negative() {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(libc)]
trait LibcResult {
    fn is_zero(&self) -> bool;
    fn is_negone(&self) -> bool;
    fn is_negative(&self) -> bool;
}

#[cfg(libc)]
macro_rules! is_impls {
    ($($t:ident)*) => ($(impl LibcResult for $t {
        fn is_zero(&self) -> bool {
            *self == 0
        }

        fn is_negone(&self) -> bool {
            *self == -1
        }

        fn is_negative(&self) -> bool {
            *self < 0
        }
    })*)
}

#[cfg(libc)]
is_impls! { i32 i64 isize }

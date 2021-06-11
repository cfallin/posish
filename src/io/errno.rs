#![allow(missing_docs)]

use std::io;
use std::os::raw::c_int;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Errno(c_int);

impl Errno {
    pub const PERM: Errno = Errno(libc::EPERM);
    pub const NOENT: Errno = Errno(libc::ENOENT);
    pub const SRCH: Errno = Errno(libc::ESRCH);
    pub const INTR: Errno = Errno(libc::EINTR);
    pub const IO: Errno = Errno(libc::EIO);
    pub const NXIO: Errno = Errno(libc::ENXIO);
    pub const TOOBIG: Errno = Errno(libc::E2BIG);
    pub const NOEXEC: Errno = Errno(libc::ENOEXEC);
    pub const BADF: Errno = Errno(libc::EBADF);
    pub const CHILD: Errno = Errno(libc::ECHILD);
    pub const AGAIN: Errno = Errno(libc::EAGAIN);
    pub const NOMEM: Errno = Errno(libc::ENOMEM);
    pub const ACCES: Errno = Errno(libc::EACCES);
    pub const FAULT: Errno = Errno(libc::EFAULT);
    #[cfg(not(target_os = "wasi"))]
    pub const NOTBLK: Errno = Errno(libc::ENOTBLK);
    pub const BUSY: Errno = Errno(libc::EBUSY);
    pub const EXIST: Errno = Errno(libc::EEXIST);
    pub const XDEV: Errno = Errno(libc::EXDEV);
    pub const NODEV: Errno = Errno(libc::ENODEV);
    pub const NOTDIR: Errno = Errno(libc::ENOTDIR);
    pub const ISDIR: Errno = Errno(libc::EISDIR);
    pub const INVAL: Errno = Errno(libc::EINVAL);
    pub const NFILE: Errno = Errno(libc::ENFILE);
    pub const MFILE: Errno = Errno(libc::EMFILE);
    pub const NOTTY: Errno = Errno(libc::ENOTTY);
    pub const TXTBSY: Errno = Errno(libc::ETXTBSY);
    pub const FBIG: Errno = Errno(libc::EFBIG);
    pub const NOSPC: Errno = Errno(libc::ENOSPC);
    pub const SPIPE: Errno = Errno(libc::ESPIPE);
    pub const ROFS: Errno = Errno(libc::EROFS);
    pub const MLINK: Errno = Errno(libc::EMLINK);
    pub const PIPE: Errno = Errno(libc::EPIPE);
    pub const DOM: Errno = Errno(libc::EDOM);
    pub const RANGE: Errno = Errno(libc::ERANGE);
    pub const DEADLK: Errno = Errno(libc::EDEADLK);
    pub const NAMETOOLONG: Errno = Errno(libc::ENAMETOOLONG);
    pub const NOLCK: Errno = Errno(libc::ENOLCK);
    pub const NOSYS: Errno = Errno(libc::ENOSYS);
    pub const NOTEMPTY: Errno = Errno(libc::ENOTEMPTY);
    pub const LOOP: Errno = Errno(libc::ELOOP);
    pub const NOMSG: Errno = Errno(libc::ENOMSG);
    pub const IDRM: Errno = Errno(libc::EIDRM);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const CHRNG: Errno = Errno(libc::ECHRNG);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const L2NSYNC: Errno = Errno(libc::EL2NSYNC);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const L3HLT: Errno = Errno(libc::EL3HLT);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const L3RST: Errno = Errno(libc::EL3RST);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LNRNG: Errno = Errno(libc::ELNRNG);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const UNATCH: Errno = Errno(libc::EUNATCH);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOCSI: Errno = Errno(libc::ENOCSI);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const L2HLT: Errno = Errno(libc::EL2HLT);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADE: Errno = Errno(libc::EBADE);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADR: Errno = Errno(libc::EBADR);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const XFULL: Errno = Errno(libc::EXFULL);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOANO: Errno = Errno(libc::ENOANO);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADRQC: Errno = Errno(libc::EBADRQC);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADSLT: Errno = Errno(libc::EBADSLT);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BFONT: Errno = Errno(libc::EBFONT);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOSTR: Errno = Errno(libc::ENOSTR);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const NODATA: Errno = Errno(libc::ENODATA);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const TIME: Errno = Errno(libc::ETIME);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOSR: Errno = Errno(libc::ENOSR);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NONET: Errno = Errno(libc::ENONET);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOPKG: Errno = Errno(libc::ENOPKG);
    #[cfg(not(target_os = "wasi"))]
    pub const REMOTE: Errno = Errno(libc::EREMOTE);
    pub const NOLINK: Errno = Errno(libc::ENOLINK);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const ADV: Errno = Errno(libc::EADV);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const SRMNT: Errno = Errno(libc::ESRMNT);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const COMM: Errno = Errno(libc::ECOMM);
    pub const PROTO: Errno = Errno(libc::EPROTO);
    pub const MULTIHOP: Errno = Errno(libc::EMULTIHOP);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const DOTDOT: Errno = Errno(libc::EDOTDOT);
    pub const BADMSG: Errno = Errno(libc::EBADMSG);
    pub const OVERFLOW: Errno = Errno(libc::EOVERFLOW);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOTUNIQ: Errno = Errno(libc::ENOTUNIQ);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADFD: Errno = Errno(libc::EBADFD);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const REMCHG: Errno = Errno(libc::EREMCHG);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBACC: Errno = Errno(libc::ELIBACC);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBBAD: Errno = Errno(libc::ELIBBAD);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBSCN: Errno = Errno(libc::ELIBSCN);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBMAX: Errno = Errno(libc::ELIBMAX);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBEXEC: Errno = Errno(libc::ELIBEXEC);
    pub const ILSEQ: Errno = Errno(libc::EILSEQ);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const RESTART: Errno = Errno(libc::ERESTART);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const STRPIPE: Errno = Errno(libc::ESTRPIPE);
    #[cfg(not(target_os = "wasi"))]
    pub const USERS: Errno = Errno(libc::EUSERS);
    pub const NOTSOCK: Errno = Errno(libc::ENOTSOCK);
    pub const DESTADDRREQ: Errno = Errno(libc::EDESTADDRREQ);
    pub const MSGSIZE: Errno = Errno(libc::EMSGSIZE);
    pub const PROTOTYPE: Errno = Errno(libc::EPROTOTYPE);
    pub const NOPROTOOPT: Errno = Errno(libc::ENOPROTOOPT);
    pub const PROTONOSUPPORT: Errno = Errno(libc::EPROTONOSUPPORT);
    #[cfg(not(target_os = "wasi"))]
    pub const SOCKTNOSUPPORT: Errno = Errno(libc::ESOCKTNOSUPPORT);
    pub const OPNOTSUPP: Errno = Errno(libc::EOPNOTSUPP);
    #[cfg(not(target_os = "wasi"))]
    pub const PFNOSUPPORT: Errno = Errno(libc::EPFNOSUPPORT);
    pub const AFNOSUPPORT: Errno = Errno(libc::EAFNOSUPPORT);
    pub const ADDRINUSE: Errno = Errno(libc::EADDRINUSE);
    pub const ADDRNOTAVAIL: Errno = Errno(libc::EADDRNOTAVAIL);
    pub const NETDOWN: Errno = Errno(libc::ENETDOWN);
    pub const NETUNREACH: Errno = Errno(libc::ENETUNREACH);
    pub const NETRESET: Errno = Errno(libc::ENETRESET);
    pub const CONNABORTED: Errno = Errno(libc::ECONNABORTED);
    pub const CONNRESET: Errno = Errno(libc::ECONNRESET);
    pub const NOBUFS: Errno = Errno(libc::ENOBUFS);
    pub const ISCONN: Errno = Errno(libc::EISCONN);
    pub const NOTCONN: Errno = Errno(libc::ENOTCONN);
    #[cfg(not(target_os = "wasi"))]
    pub const SHUTDOWN: Errno = Errno(libc::ESHUTDOWN);
    #[cfg(not(target_os = "wasi"))]
    pub const TOOMANYREFS: Errno = Errno(libc::ETOOMANYREFS);
    pub const TIMEDOUT: Errno = Errno(libc::ETIMEDOUT);
    pub const CONNREFUSED: Errno = Errno(libc::ECONNREFUSED);
    #[cfg(not(target_os = "wasi"))]
    pub const HOSTDOWN: Errno = Errno(libc::EHOSTDOWN);
    pub const HOSTUNREACH: Errno = Errno(libc::EHOSTUNREACH);
    pub const ALREADY: Errno = Errno(libc::EALREADY);
    pub const INPROGRESS: Errno = Errno(libc::EINPROGRESS);
    pub const STALE: Errno = Errno(libc::ESTALE);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const UCLEAN: Errno = Errno(libc::EUCLEAN);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOTNAM: Errno = Errno(libc::ENOTNAM);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NAVAIL: Errno = Errno(libc::ENAVAIL);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const ISNAM: Errno = Errno(libc::EISNAM);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const REMOTEIO: Errno = Errno(libc::EREMOTEIO);
    pub const DQUOT: Errno = Errno(libc::EDQUOT);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOMEDIUM: Errno = Errno(libc::ENOMEDIUM);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const MEDIUMTYPE: Errno = Errno(libc::EMEDIUMTYPE);
    pub const CANCELED: Errno = Errno(libc::ECANCELED);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOKEY: Errno = Errno(libc::ENOKEY);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const KEYEXPIRED: Errno = Errno(libc::EKEYEXPIRED);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    #[cfg(not(target_os = "wasi"))]
    pub const KEYREVOKED: Errno = Errno(libc::EKEYREVOKED);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const KEYREJECTED: Errno = Errno(libc::EKEYREJECTED);
    #[cfg(not(target_os = "netbsd"))]
    pub const OWNERDEAD: Errno = Errno(libc::EOWNERDEAD);
    #[cfg(not(target_os = "netbsd"))]
    pub const NOTRECOVERABLE: Errno = Errno(libc::ENOTRECOVERABLE);
    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const RFKILL: Errno = Errno(libc::ERFKILL);
    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const HWPOISON: Errno = Errno(libc::EHWPOISON);
    pub const WOULDBLOCK: Errno = Errno(libc::EWOULDBLOCK);
    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const DEADLOCK: Errno = Errno(libc::EDEADLOCK);
    pub const NOTSUP: Errno = Errno(libc::ENOTSUP);

    #[inline]
    pub fn io_error(&self) -> io::Error {
        io::Error::from_raw_os_error(self.0)
    }

    #[inline]
    pub fn from_io_error(err: &io::Error) -> Option<Self> {
        err.raw_os_error().map(Self)
    }
}

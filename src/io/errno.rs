#![allow(missing_docs)]

use std::{io, os::raw::c_int};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Errno(c_int);

#[cfg(libc)]
impl Errno {
    pub const ACCES: Errno = Errno(libc::EACCES);
    pub const ADDRINUSE: Errno = Errno(libc::EADDRINUSE);
    pub const ADDRNOTAVAIL: Errno = Errno(libc::EADDRNOTAVAIL);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const ADV: Errno = Errno(libc::EADV);
    pub const AFNOSUPPORT: Errno = Errno(libc::EAFNOSUPPORT);
    pub const AGAIN: Errno = Errno(libc::EAGAIN);
    pub const ALREADY: Errno = Errno(libc::EALREADY);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADE: Errno = Errno(libc::EBADE);
    pub const BADF: Errno = Errno(libc::EBADF);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADFD: Errno = Errno(libc::EBADFD);
    pub const BADMSG: Errno = Errno(libc::EBADMSG);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADR: Errno = Errno(libc::EBADR);
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
    pub const BUSY: Errno = Errno(libc::EBUSY);
    pub const CANCELED: Errno = Errno(libc::ECANCELED);
    pub const CHILD: Errno = Errno(libc::ECHILD);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const CHRNG: Errno = Errno(libc::ECHRNG);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const COMM: Errno = Errno(libc::ECOMM);
    pub const CONNABORTED: Errno = Errno(libc::ECONNABORTED);
    pub const CONNREFUSED: Errno = Errno(libc::ECONNREFUSED);
    pub const CONNRESET: Errno = Errno(libc::ECONNRESET);
    pub const DEADLK: Errno = Errno(libc::EDEADLK);
    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const DEADLOCK: Errno = Errno(libc::EDEADLOCK);
    pub const DESTADDRREQ: Errno = Errno(libc::EDESTADDRREQ);
    pub const DOM: Errno = Errno(libc::EDOM);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const DOTDOT: Errno = Errno(libc::EDOTDOT);
    pub const DQUOT: Errno = Errno(libc::EDQUOT);
    pub const EXIST: Errno = Errno(libc::EEXIST);
    pub const FAULT: Errno = Errno(libc::EFAULT);
    pub const FBIG: Errno = Errno(libc::EFBIG);
    #[cfg(not(target_os = "wasi"))]
    pub const HOSTDOWN: Errno = Errno(libc::EHOSTDOWN);
    pub const HOSTUNREACH: Errno = Errno(libc::EHOSTUNREACH);
    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const HWPOISON: Errno = Errno(libc::EHWPOISON);
    pub const IDRM: Errno = Errno(libc::EIDRM);
    pub const ILSEQ: Errno = Errno(libc::EILSEQ);
    pub const INPROGRESS: Errno = Errno(libc::EINPROGRESS);
    pub const INTR: Errno = Errno(libc::EINTR);
    pub const INVAL: Errno = Errno(libc::EINVAL);
    pub const IO: Errno = Errno(libc::EIO);
    pub const ISCONN: Errno = Errno(libc::EISCONN);
    pub const ISDIR: Errno = Errno(libc::EISDIR);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const ISNAM: Errno = Errno(libc::EISNAM);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const KEYEXPIRED: Errno = Errno(libc::EKEYEXPIRED);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const KEYREJECTED: Errno = Errno(libc::EKEYREJECTED);
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
    pub const L2HLT: Errno = Errno(libc::EL2HLT);
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
    pub const LIBEXEC: Errno = Errno(libc::ELIBEXEC);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBMAX: Errno = Errno(libc::ELIBMAX);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBSCN: Errno = Errno(libc::ELIBSCN);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LNRNG: Errno = Errno(libc::ELNRNG);
    pub const LOOP: Errno = Errno(libc::ELOOP);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const MEDIUMTYPE: Errno = Errno(libc::EMEDIUMTYPE);
    pub const MFILE: Errno = Errno(libc::EMFILE);
    pub const MLINK: Errno = Errno(libc::EMLINK);
    pub const MSGSIZE: Errno = Errno(libc::EMSGSIZE);
    pub const MULTIHOP: Errno = Errno(libc::EMULTIHOP);
    pub const NAMETOOLONG: Errno = Errno(libc::ENAMETOOLONG);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NAVAIL: Errno = Errno(libc::ENAVAIL);
    pub const NETDOWN: Errno = Errno(libc::ENETDOWN);
    pub const NETRESET: Errno = Errno(libc::ENETRESET);
    pub const NETUNREACH: Errno = Errno(libc::ENETUNREACH);
    pub const NFILE: Errno = Errno(libc::ENFILE);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOANO: Errno = Errno(libc::ENOANO);
    pub const NOBUFS: Errno = Errno(libc::ENOBUFS);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOCSI: Errno = Errno(libc::ENOCSI);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const NODATA: Errno = Errno(libc::ENODATA);
    pub const NODEV: Errno = Errno(libc::ENODEV);
    pub const NOENT: Errno = Errno(libc::ENOENT);
    pub const NOEXEC: Errno = Errno(libc::ENOEXEC);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOKEY: Errno = Errno(libc::ENOKEY);
    pub const NOLCK: Errno = Errno(libc::ENOLCK);
    pub const NOLINK: Errno = Errno(libc::ENOLINK);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOMEDIUM: Errno = Errno(libc::ENOMEDIUM);
    pub const NOMEM: Errno = Errno(libc::ENOMEM);
    pub const NOMSG: Errno = Errno(libc::ENOMSG);
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
    pub const NOPROTOOPT: Errno = Errno(libc::ENOPROTOOPT);
    pub const NOSPC: Errno = Errno(libc::ENOSPC);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOSR: Errno = Errno(libc::ENOSR);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOSTR: Errno = Errno(libc::ENOSTR);
    pub const NOSYS: Errno = Errno(libc::ENOSYS);
    #[cfg(not(target_os = "wasi"))]
    pub const NOTBLK: Errno = Errno(libc::ENOTBLK);
    pub const NOTCONN: Errno = Errno(libc::ENOTCONN);
    pub const NOTDIR: Errno = Errno(libc::ENOTDIR);
    pub const NOTEMPTY: Errno = Errno(libc::ENOTEMPTY);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOTNAM: Errno = Errno(libc::ENOTNAM);
    #[cfg(not(target_os = "netbsd"))]
    pub const NOTRECOVERABLE: Errno = Errno(libc::ENOTRECOVERABLE);
    pub const NOTSOCK: Errno = Errno(libc::ENOTSOCK);
    pub const NOTSUP: Errno = Errno(libc::ENOTSUP);
    pub const NOTTY: Errno = Errno(libc::ENOTTY);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOTUNIQ: Errno = Errno(libc::ENOTUNIQ);
    pub const NXIO: Errno = Errno(libc::ENXIO);
    pub const OPNOTSUPP: Errno = Errno(libc::EOPNOTSUPP);
    pub const OVERFLOW: Errno = Errno(libc::EOVERFLOW);
    #[cfg(not(target_os = "netbsd"))]
    pub const OWNERDEAD: Errno = Errno(libc::EOWNERDEAD);
    pub const PERM: Errno = Errno(libc::EPERM);
    #[cfg(not(target_os = "wasi"))]
    pub const PFNOSUPPORT: Errno = Errno(libc::EPFNOSUPPORT);
    pub const PIPE: Errno = Errno(libc::EPIPE);
    pub const PROTO: Errno = Errno(libc::EPROTO);
    pub const PROTONOSUPPORT: Errno = Errno(libc::EPROTONOSUPPORT);
    pub const PROTOTYPE: Errno = Errno(libc::EPROTOTYPE);
    pub const RANGE: Errno = Errno(libc::ERANGE);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const REMCHG: Errno = Errno(libc::EREMCHG);
    #[cfg(not(target_os = "wasi"))]
    pub const REMOTE: Errno = Errno(libc::EREMOTE);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const REMOTEIO: Errno = Errno(libc::EREMOTEIO);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const RESTART: Errno = Errno(libc::ERESTART);
    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const RFKILL: Errno = Errno(libc::ERFKILL);
    pub const ROFS: Errno = Errno(libc::EROFS);
    #[cfg(not(target_os = "wasi"))]
    pub const SHUTDOWN: Errno = Errno(libc::ESHUTDOWN);
    #[cfg(not(target_os = "wasi"))]
    pub const SOCKTNOSUPPORT: Errno = Errno(libc::ESOCKTNOSUPPORT);
    pub const SPIPE: Errno = Errno(libc::ESPIPE);
    pub const SRCH: Errno = Errno(libc::ESRCH);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const SRMNT: Errno = Errno(libc::ESRMNT);
    pub const STALE: Errno = Errno(libc::ESTALE);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const STRPIPE: Errno = Errno(libc::ESTRPIPE);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const TIME: Errno = Errno(libc::ETIME);
    pub const TIMEDOUT: Errno = Errno(libc::ETIMEDOUT);
    pub const TOOBIG: Errno = Errno(libc::E2BIG);
    #[cfg(not(target_os = "wasi"))]
    pub const TOOMANYREFS: Errno = Errno(libc::ETOOMANYREFS);
    pub const TXTBSY: Errno = Errno(libc::ETXTBSY);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const UCLEAN: Errno = Errno(libc::EUCLEAN);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const UNATCH: Errno = Errno(libc::EUNATCH);
    #[cfg(not(target_os = "wasi"))]
    pub const USERS: Errno = Errno(libc::EUSERS);
    pub const WOULDBLOCK: Errno = Errno(libc::EWOULDBLOCK);
    pub const XDEV: Errno = Errno(libc::EXDEV);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const XFULL: Errno = Errno(libc::EXFULL);
}

#[cfg(linux_raw)]
impl Errno {
    pub const ACCES: Errno = Errno(linux_raw_sys::errno::EACCES as c_int);
    pub const ADDRINUSE: Errno = Errno(linux_raw_sys::errno::EADDRINUSE as c_int);
    pub const ADDRNOTAVAIL: Errno = Errno(linux_raw_sys::errno::EADDRNOTAVAIL as c_int);
    pub const ADV: Errno = Errno(linux_raw_sys::errno::EADV as c_int);
    pub const AFNOSUPPORT: Errno = Errno(linux_raw_sys::errno::EAFNOSUPPORT as c_int);
    pub const AGAIN: Errno = Errno(linux_raw_sys::errno::EAGAIN as c_int);
    pub const ALREADY: Errno = Errno(linux_raw_sys::errno::EALREADY as c_int);
    pub const BADE: Errno = Errno(linux_raw_sys::errno::EBADE as c_int);
    pub const BADF: Errno = Errno(linux_raw_sys::errno::EBADF as c_int);
    pub const BADFD: Errno = Errno(linux_raw_sys::errno::EBADFD as c_int);
    pub const BADMSG: Errno = Errno(linux_raw_sys::errno::EBADMSG as c_int);
    pub const BADR: Errno = Errno(linux_raw_sys::errno::EBADR as c_int);
    pub const BADRQC: Errno = Errno(linux_raw_sys::errno::EBADRQC as c_int);
    pub const BADSLT: Errno = Errno(linux_raw_sys::errno::EBADSLT as c_int);
    pub const BFONT: Errno = Errno(linux_raw_sys::errno::EBFONT as c_int);
    pub const BUSY: Errno = Errno(linux_raw_sys::errno::EBUSY as c_int);
    pub const CANCELED: Errno = Errno(linux_raw_sys::errno::ECANCELED as c_int);
    pub const CHILD: Errno = Errno(linux_raw_sys::errno::ECHILD as c_int);
    pub const CHRNG: Errno = Errno(linux_raw_sys::errno::ECHRNG as c_int);
    pub const COMM: Errno = Errno(linux_raw_sys::errno::ECOMM as c_int);
    pub const CONNABORTED: Errno = Errno(linux_raw_sys::errno::ECONNABORTED as c_int);
    pub const CONNREFUSED: Errno = Errno(linux_raw_sys::errno::ECONNREFUSED as c_int);
    pub const CONNRESET: Errno = Errno(linux_raw_sys::errno::ECONNRESET as c_int);
    pub const DEADLK: Errno = Errno(linux_raw_sys::errno::EDEADLK as c_int);
    pub const DEADLOCK: Errno = Errno(linux_raw_sys::errno::EDEADLOCK as c_int);
    pub const DESTADDRREQ: Errno = Errno(linux_raw_sys::errno::EDESTADDRREQ as c_int);
    pub const DOM: Errno = Errno(linux_raw_sys::errno::EDOM as c_int);
    pub const DOTDOT: Errno = Errno(linux_raw_sys::errno::EDOTDOT as c_int);
    pub const DQUOT: Errno = Errno(linux_raw_sys::errno::EDQUOT as c_int);
    pub const EXIST: Errno = Errno(linux_raw_sys::errno::EEXIST as c_int);
    pub const FAULT: Errno = Errno(linux_raw_sys::errno::EFAULT as c_int);
    pub const FBIG: Errno = Errno(linux_raw_sys::errno::EFBIG as c_int);
    pub const HOSTDOWN: Errno = Errno(linux_raw_sys::errno::EHOSTDOWN as c_int);
    pub const HOSTUNREACH: Errno = Errno(linux_raw_sys::errno::EHOSTUNREACH as c_int);
    pub const HWPOISON: Errno = Errno(linux_raw_sys::v5_4::errno::EHWPOISON as c_int);
    pub const IDRM: Errno = Errno(linux_raw_sys::errno::EIDRM as c_int);
    pub const ILSEQ: Errno = Errno(linux_raw_sys::errno::EILSEQ as c_int);
    pub const INPROGRESS: Errno = Errno(linux_raw_sys::errno::EINPROGRESS as c_int);
    pub const INTR: Errno = Errno(linux_raw_sys::errno::EINTR as c_int);
    pub const INVAL: Errno = Errno(linux_raw_sys::errno::EINVAL as c_int);
    pub const IO: Errno = Errno(linux_raw_sys::errno::EIO as c_int);
    pub const ISCONN: Errno = Errno(linux_raw_sys::errno::EISCONN as c_int);
    pub const ISDIR: Errno = Errno(linux_raw_sys::errno::EISDIR as c_int);
    pub const ISNAM: Errno = Errno(linux_raw_sys::errno::EISNAM as c_int);
    pub const KEYEXPIRED: Errno = Errno(linux_raw_sys::errno::EKEYEXPIRED as c_int);
    pub const KEYREJECTED: Errno = Errno(linux_raw_sys::errno::EKEYREJECTED as c_int);
    pub const KEYREVOKED: Errno = Errno(linux_raw_sys::errno::EKEYREVOKED as c_int);
    pub const L2HLT: Errno = Errno(linux_raw_sys::errno::EL2HLT as c_int);
    pub const L2NSYNC: Errno = Errno(linux_raw_sys::errno::EL2NSYNC as c_int);
    pub const L3HLT: Errno = Errno(linux_raw_sys::errno::EL3HLT as c_int);
    pub const L3RST: Errno = Errno(linux_raw_sys::errno::EL3RST as c_int);
    pub const LIBACC: Errno = Errno(linux_raw_sys::errno::ELIBACC as c_int);
    pub const LIBBAD: Errno = Errno(linux_raw_sys::errno::ELIBBAD as c_int);
    pub const LIBEXEC: Errno = Errno(linux_raw_sys::errno::ELIBEXEC as c_int);
    pub const LIBMAX: Errno = Errno(linux_raw_sys::errno::ELIBMAX as c_int);
    pub const LIBSCN: Errno = Errno(linux_raw_sys::errno::ELIBSCN as c_int);
    pub const LNRNG: Errno = Errno(linux_raw_sys::errno::ELNRNG as c_int);
    pub const LOOP: Errno = Errno(linux_raw_sys::errno::ELOOP as c_int);
    pub const MEDIUMTYPE: Errno = Errno(linux_raw_sys::errno::EMEDIUMTYPE as c_int);
    pub const MFILE: Errno = Errno(linux_raw_sys::errno::EMFILE as c_int);
    pub const MLINK: Errno = Errno(linux_raw_sys::errno::EMLINK as c_int);
    pub const MSGSIZE: Errno = Errno(linux_raw_sys::errno::EMSGSIZE as c_int);
    pub const MULTIHOP: Errno = Errno(linux_raw_sys::errno::EMULTIHOP as c_int);
    pub const NAMETOOLONG: Errno = Errno(linux_raw_sys::errno::ENAMETOOLONG as c_int);
    pub const NAVAIL: Errno = Errno(linux_raw_sys::errno::ENAVAIL as c_int);
    pub const NETDOWN: Errno = Errno(linux_raw_sys::errno::ENETDOWN as c_int);
    pub const NETRESET: Errno = Errno(linux_raw_sys::errno::ENETRESET as c_int);
    pub const NETUNREACH: Errno = Errno(linux_raw_sys::errno::ENETUNREACH as c_int);
    pub const NFILE: Errno = Errno(linux_raw_sys::errno::ENFILE as c_int);
    pub const NOANO: Errno = Errno(linux_raw_sys::errno::ENOANO as c_int);
    pub const NOBUFS: Errno = Errno(linux_raw_sys::errno::ENOBUFS as c_int);
    pub const NOCSI: Errno = Errno(linux_raw_sys::errno::ENOCSI as c_int);
    pub const NODATA: Errno = Errno(linux_raw_sys::errno::ENODATA as c_int);
    pub const NODEV: Errno = Errno(linux_raw_sys::errno::ENODEV as c_int);
    pub const NOENT: Errno = Errno(linux_raw_sys::errno::ENOENT as c_int);
    pub const NOEXEC: Errno = Errno(linux_raw_sys::errno::ENOEXEC as c_int);
    pub const NOKEY: Errno = Errno(linux_raw_sys::errno::ENOKEY as c_int);
    pub const NOLCK: Errno = Errno(linux_raw_sys::errno::ENOLCK as c_int);
    pub const NOLINK: Errno = Errno(linux_raw_sys::errno::ENOLINK as c_int);
    pub const NOMEDIUM: Errno = Errno(linux_raw_sys::errno::ENOMEDIUM as c_int);
    pub const NOMEM: Errno = Errno(linux_raw_sys::errno::ENOMEM as c_int);
    pub const NOMSG: Errno = Errno(linux_raw_sys::errno::ENOMSG as c_int);
    pub const NONET: Errno = Errno(linux_raw_sys::errno::ENONET as c_int);
    pub const NOPKG: Errno = Errno(linux_raw_sys::errno::ENOPKG as c_int);
    pub const NOPROTOOPT: Errno = Errno(linux_raw_sys::errno::ENOPROTOOPT as c_int);
    pub const NOSPC: Errno = Errno(linux_raw_sys::errno::ENOSPC as c_int);
    pub const NOSR: Errno = Errno(linux_raw_sys::errno::ENOSR as c_int);
    pub const NOSTR: Errno = Errno(linux_raw_sys::errno::ENOSTR as c_int);
    pub const NOSYS: Errno = Errno(linux_raw_sys::errno::ENOSYS as c_int);
    pub const NOTBLK: Errno = Errno(linux_raw_sys::errno::ENOTBLK as c_int);
    pub const NOTCONN: Errno = Errno(linux_raw_sys::errno::ENOTCONN as c_int);
    pub const NOTDIR: Errno = Errno(linux_raw_sys::errno::ENOTDIR as c_int);
    pub const NOTEMPTY: Errno = Errno(linux_raw_sys::errno::ENOTEMPTY as c_int);
    pub const NOTNAM: Errno = Errno(linux_raw_sys::errno::ENOTNAM as c_int);
    pub const NOTRECOVERABLE: Errno = Errno(linux_raw_sys::errno::ENOTRECOVERABLE as c_int);
    pub const NOTSOCK: Errno = Errno(linux_raw_sys::errno::ENOTSOCK as c_int);
    // On Linux, `ENOTSUP` has the same value as `EOPNOTSUPP`.
    pub const NOTSUP: Errno = Errno(linux_raw_sys::errno::EOPNOTSUPP as c_int);
    pub const NOTTY: Errno = Errno(linux_raw_sys::errno::ENOTTY as c_int);
    pub const NOTUNIQ: Errno = Errno(linux_raw_sys::errno::ENOTUNIQ as c_int);
    pub const NXIO: Errno = Errno(linux_raw_sys::errno::ENXIO as c_int);
    pub const OPNOTSUPP: Errno = Errno(linux_raw_sys::errno::EOPNOTSUPP as c_int);
    pub const OVERFLOW: Errno = Errno(linux_raw_sys::errno::EOVERFLOW as c_int);
    pub const OWNERDEAD: Errno = Errno(linux_raw_sys::errno::EOWNERDEAD as c_int);
    // These have type `u32` in the bindgen bindings; cast them to `c_int` as
    // knowledge that the platform errno type is signed is widespread.
    pub const PERM: Errno = Errno(linux_raw_sys::errno::EPERM as c_int);
    pub const PFNOSUPPORT: Errno = Errno(linux_raw_sys::errno::EPFNOSUPPORT as c_int);
    pub const PIPE: Errno = Errno(linux_raw_sys::errno::EPIPE as c_int);
    pub const PROTO: Errno = Errno(linux_raw_sys::errno::EPROTO as c_int);
    pub const PROTONOSUPPORT: Errno = Errno(linux_raw_sys::errno::EPROTONOSUPPORT as c_int);
    pub const PROTOTYPE: Errno = Errno(linux_raw_sys::errno::EPROTOTYPE as c_int);
    pub const RANGE: Errno = Errno(linux_raw_sys::errno::ERANGE as c_int);
    pub const REMCHG: Errno = Errno(linux_raw_sys::errno::EREMCHG as c_int);
    pub const REMOTE: Errno = Errno(linux_raw_sys::errno::EREMOTE as c_int);
    pub const REMOTEIO: Errno = Errno(linux_raw_sys::errno::EREMOTEIO as c_int);
    pub const RESTART: Errno = Errno(linux_raw_sys::errno::ERESTART as c_int);
    pub const RFKILL: Errno = Errno(linux_raw_sys::errno::ERFKILL as c_int);
    pub const ROFS: Errno = Errno(linux_raw_sys::errno::EROFS as c_int);
    pub const SHUTDOWN: Errno = Errno(linux_raw_sys::errno::ESHUTDOWN as c_int);
    pub const SOCKTNOSUPPORT: Errno = Errno(linux_raw_sys::errno::ESOCKTNOSUPPORT as c_int);
    pub const SPIPE: Errno = Errno(linux_raw_sys::errno::ESPIPE as c_int);
    pub const SRCH: Errno = Errno(linux_raw_sys::errno::ESRCH as c_int);
    pub const SRMNT: Errno = Errno(linux_raw_sys::errno::ESRMNT as c_int);
    pub const STALE: Errno = Errno(linux_raw_sys::errno::ESTALE as c_int);
    pub const STRPIPE: Errno = Errno(linux_raw_sys::errno::ESTRPIPE as c_int);
    pub const TIME: Errno = Errno(linux_raw_sys::errno::ETIME as c_int);
    pub const TIMEDOUT: Errno = Errno(linux_raw_sys::errno::ETIMEDOUT as c_int);
    pub const TOOBIG: Errno = Errno(linux_raw_sys::errno::E2BIG as c_int);
    pub const TOOMANYREFS: Errno = Errno(linux_raw_sys::errno::ETOOMANYREFS as c_int);
    pub const TXTBSY: Errno = Errno(linux_raw_sys::errno::ETXTBSY as c_int);
    pub const UCLEAN: Errno = Errno(linux_raw_sys::errno::EUCLEAN as c_int);
    pub const UNATCH: Errno = Errno(linux_raw_sys::errno::EUNATCH as c_int);
    pub const USERS: Errno = Errno(linux_raw_sys::errno::EUSERS as c_int);
    pub const WOULDBLOCK: Errno = Errno(linux_raw_sys::errno::EWOULDBLOCK as c_int);
    pub const XDEV: Errno = Errno(linux_raw_sys::errno::EXDEV as c_int);
    pub const XFULL: Errno = Errno(linux_raw_sys::errno::EXFULL as c_int);
}

impl Errno {
    #[inline]
    pub fn io_error(&self) -> io::Error {
        io::Error::from_raw_os_error(self.0)
    }

    #[inline]
    pub fn from_io_error(err: &io::Error) -> Option<Self> {
        err.raw_os_error().map(Self)
    }
}

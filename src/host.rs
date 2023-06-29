use crate::NullStringError;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::io;
use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};
use videostream_sys as ffi;

/// The Host structure provides the frame sharing functionality.  Only a single
/// host can own frames while a host can have many Client subscribers to the
/// frames.
///
/// A host is created with a socket path which it will own exclusively and
/// allowing clients to connect in order to receive frames.
pub struct Host {
    ptr: *mut ffi::VSLHost,
}

impl Host {
    /// Creates a new Host and creates a socket at the specified path on which
    /// it will listen for client connections.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let path_str_c = CString::new(path.as_ref().as_os_str().as_bytes())?;
        let ptr = unsafe { ffi::vsl_host_init(path_str_c.as_ptr()) };
        if ptr.is_null() {
            let err = io::Error::last_os_error();
            return Err(Box::new(err));
        }

        return Ok(Host { ptr });
    }

    pub fn path(&self) -> Result<PathBuf, Box<dyn Error>> {
        let path_str_c = unsafe { ffi::vsl_host_path(self.ptr) };
        if path_str_c.is_null() {
            return Err(Box::new(NullStringError {}));
        }

        let path_str = unsafe { CStr::from_ptr(path_str_c).to_str()? };
        return Ok(PathBuf::from(path_str));
    }

    pub fn poll(&self) {}

    pub fn process(&self) {}

    pub fn sockets(&self) {}
}

impl Drop for Host {
    fn drop(&mut self) {
        unsafe { ffi::vsl_host_release(self.ptr) }
    }
}

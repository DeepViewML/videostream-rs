//! DeepView VideoStream Library for Rust
//!
//! The VideoStream Library provides a mechanism for zero-copy sharing of video
//! frames across processes and containers.  The sharing is done through dmabuf
//! or shared-memory buffers with signalling over UNIX Domain Sockets.
//!
//! Au-Zone Technologies provides professional support through the
//! [`DeepView Support Portal`].
//!
//! [`DeepView Support Portal`]: https://support.deepviewml.com

use std::error::Error;
use std::fmt;

/// The frame module provides the common frame handling functionality.
pub mod frame;

/// The client module provides the frame subscription functionality.
pub mod client;

/// The host module provides the frame sharing functionality.
pub mod host;

#[derive(Debug)]
struct NullStringError;

impl Error for NullStringError {}

impl fmt::Display for NullStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid null string provided")
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use videostream_sys::vsl_version;

    #[test]
    fn test_version() {
        let c_ver = unsafe { CStr::from_ptr(vsl_version()) };
        println!("VideoStream Library {}", c_ver.to_str().unwrap());
    }
}

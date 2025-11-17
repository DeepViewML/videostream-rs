// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Au-Zone Technologies

//! VideoStream Library for Rust
//!
//! Safe Rust bindings for the VideoStream Library, providing zero-copy video
//! frame management and distribution across processes and containers.
//!
//! The VideoStream Library enables efficient frame sharing through DMA buffers
//! or shared-memory with signaling over UNIX Domain Sockets, optimized for
//! edge AI and computer vision applications on resource-constrained embedded devices.
//!
//! # Quick Start
//!
//! ## Publishing Frames (Host)
//!
//! ```no_run
//! use videostream::Host;
//!
//! let host = Host::new("/tmp/video.sock")?;
//! let frame = host.allocate_frame(1920, 1080, fourcc)?;
//! host.publish(frame)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Subscribing to Frames (Client)
//!
//! ```no_run
//! use videostream::Client;
//!
//! let client = Client::new("/tmp/video.sock", true)?;
//! let frame = client.wait()?;
//! process_frame(&frame);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Features
//!
//! - Zero-copy frame sharing across process boundaries
//! - DMA buffer support for hardware-accelerated access
//! - Hardware video encoding/decoding (H.264, H.265)
//! - V4L2 camera capture integration
//! - Multi-subscriber support (one publisher, many subscribers)
//!
//! # Support
//!
//! For questions and support:
//! - Documentation: <https://docs.rs/videostream>
//! - Repository: <https://github.com/DeepViewML/videostream-rs>
//! - Professional support: support@au-zone.com

use std::{error::Error, ffi::CStr, fmt};
use videostream_sys as ffi;

/// The frame module provides the common frame handling functionality.
pub mod frame;

/// The client module provides the frame subscription functionality.
pub mod client;

/// The host module provides the frame sharing functionality.
pub mod host;

/// The encoder module provides accelerated video encoding to h.264 and h.265
pub mod encoder;

/// The encoder module provides accelerated video decoding from h.264 and h.265
pub mod decoder;

/// The camera module provides camera capture capabilities.
pub mod camera;

/// The fourcc module provides portable handling of fourcc codes.
pub mod fourcc;

#[derive(Debug)]
struct NullStringError;

impl Error for NullStringError {}

impl fmt::Display for NullStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid null string provided")
    }
}

pub fn version() -> &'static str {
    let cstr = unsafe { CStr::from_ptr(ffi::vsl_version()) };
    cstr.to_str().unwrap()
}

pub fn timestamp() -> i64 {
    unsafe { ffi::vsl_timestamp() }
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

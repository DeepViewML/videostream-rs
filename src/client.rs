// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Au-Zone Technologies

use crate::frame::Frame;
use std::{
    error::Error,
    ffi::{CStr, CString},
    io,
};
use videostream_sys as ffi;

pub struct Client {
    ptr: *mut ffi::VSLClient,
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}

impl Client {
    pub fn new(path: &str, reconnect: bool) -> Result<Self, Box<dyn Error>> {
        let path_str_c = CString::new(path)?;
        let ptr = unsafe {
            ffi::vsl_client_init(
                path_str_c.as_ptr(),
                std::ptr::null_mut(),
                reconnect,
            )
        };
        if ptr.is_null() {
            let err = io::Error::last_os_error();
            return Err(Box::new(err));
        }

        Ok(Client { ptr })
    }

    pub fn release(&self) {
        unsafe { ffi::vsl_client_release(self.ptr) }
    }

    pub fn disconnect(&self) {
        unsafe { ffi::vsl_client_disconnect(self.ptr) }
    }

    pub fn userptr() {
        panic!("CURRENTLY NOT USED");
    }

    #[allow(clippy::result_unit_err)]
    pub fn path(&self) -> Result<&str, ()> {
        let path_ptr = unsafe { ffi::vsl_client_path(self.ptr) };
        if !path_ptr.is_null() {
            let p_cstr = unsafe { CStr::from_ptr(path_ptr) };
            let path_ref = p_cstr.to_str().unwrap();
            return Ok(path_ref);
        }
        Err(())
    }

    pub fn set_timeout(&self, timeout: f32) {
        unsafe { ffi::vsl_client_set_timeout(self.ptr, timeout) };
    }

    pub fn get_frame(&self, until: i64) -> Result<Frame, Box<dyn Error>> {
        let frame = unsafe { ffi::vsl_frame_wait(self.ptr, until) };
        if frame.is_null() {
            let err = io::Error::last_os_error();
            return Err(Box::new(err));
        }
        Ok(Frame::wrap(frame).unwrap())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.release();
        self.disconnect();
    }
}

use crate::frame::Frame;
use std::{
    error::Error,
    ffi::{c_void, CStr, CString},
    io,
};
use videostream_sys as ffi;

use crate::NullStringError;

pub struct Client {
    ptr: *mut ffi::VSLClient,
}

unsafe impl Sync for Client {}

impl Client {
    pub fn new(path: &str, reconnect: bool) -> Result<Self, Box<dyn Error>> {
        let path_str_c = CString::new(path)?;
        let ptr = unsafe {
            ffi::vsl_client_init(
                path_str_c.as_ptr(),
                std::ptr::null_mut() as *mut c_void,
                reconnect,
            )
        };
        if ptr.is_null() {
            let err = io::Error::last_os_error();
            return Err(Box::new(err));
        }

        return Ok(Client { ptr });
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

    pub fn path(&self) -> Result<&str, ()> {
        let path_ptr = unsafe { ffi::vsl_client_path(self.ptr) };
        if !path_ptr.is_null() {
            let p_cstr = unsafe { CStr::from_ptr(path_ptr) };
            let path_ref = p_cstr.to_str().unwrap();
            return Ok(path_ref);
        }
        return Err(());
    }

    pub fn set_timeout(&self, timeout: f32) {
        unsafe { ffi::vsl_client_set_timeout(self.ptr, timeout) };
    }

    pub fn get_frame(&self, until: i64) -> Result<Frame, Box<dyn Error>> {
        let frame = unsafe { ffi::vsl_frame_wait(self.ptr, until) };
        if frame.is_null() {
            return Err(Box::new(NullStringError {}));
        }
        return Ok(Frame::wrap(frame).unwrap());
    }
}

impl Drop for Client {
	fn drop(&mut self) {
		self.release();
		self.disconnect();
	}
}

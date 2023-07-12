use crate::client;
use std::{
    error::Error,
    io,
    slice::{from_raw_parts, from_raw_parts_mut},
};
use videostream_sys as ffi;

/// The Frame structure handles the frame and underlying framebuffer.  A frame
/// can be an image or a single video frame, the distinction is not considered.
///
/// A frame can be created and used as a free-standing frame, which means it is
/// not published through a Host nor was it created from a receiving Client. A
/// free-standing frame can be mapped and copied to other frames which provides
/// an optimized method for resizing or converting between formats.
pub struct Frame {
    ptr: *mut ffi::VSLFrame,
}

impl Frame {
    pub fn new(ptr: *mut ffi::VSLFrame) -> Result<Self, ()> {
        if ptr.is_null() {
            return Err(());
        }

        return Ok(Frame { ptr });
    }

    pub fn register() {}

    pub fn unregister() {}

    pub fn wait(client: &client::Client, until: i64) -> Result<Self, Box<dyn Error>> {
        let wrapper = client.get_frame(until)?;
        return Ok(Frame { ptr: wrapper.ptr });
    }

    pub fn release(&self) {
        unsafe { ffi::vsl_frame_release(self.ptr) };
    }

    pub fn trylock(&self) -> Result<(), Box<dyn Error>> {
        let ret = unsafe { ffi::vsl_frame_trylock(self.ptr) };
        if ret == 0 {
            return Ok(());
        }
        let err = io::Error::last_os_error();
        return Err(Box::new(err));
    }

    pub fn unlock(&self) -> Result<(), Box<dyn Error>> {
        if unsafe { ffi::vsl_frame_unlock(self.ptr) as i32 } == -1 {
            let err = io::Error::last_os_error();
            return Err(Box::new(err));
        }
        return Ok(());
    }

    pub fn serial(&self) -> i64 {
        return unsafe { ffi::vsl_frame_serial(self.ptr) };
    }

    pub fn timestamp(&self) -> i64 {
        let timestamp: i64 = unsafe { ffi::vsl_frame_timestamp(self.ptr) };
        return timestamp;
    }

    pub fn duration(&self) -> i64 {
        return unsafe { ffi::vsl_frame_duration(self.ptr) };
    }

    pub fn pts(&self) -> i64 {
        return unsafe { ffi::vsl_frame_pts(self.ptr) };
    }

    pub fn dts(&self) -> i64 {
        return unsafe { ffi::vsl_frame_dts(self.ptr) };
    }

    pub fn expires(&self) -> i64 {
        return unsafe { ffi::vsl_frame_expires(self.ptr) };
    }

    pub fn fourcc(&self) -> u32 {
        return unsafe { ffi::vsl_frame_fourcc(self.ptr) };
    }

    pub fn width(&self) -> i32 {
        let width: std::os::raw::c_int = unsafe { ffi::vsl_frame_width(self.ptr) };
        return width as i32;
    }

    pub fn height(&self) -> i32 {
        let height: std::os::raw::c_int = unsafe { ffi::vsl_frame_height(self.ptr) };
        return height as i32;
    }

    pub fn size(&self) -> i32 {
        return unsafe { ffi::vsl_frame_size(self.ptr) as i32 }; //Needs work
    }

    pub fn handle(&self) -> i32 {
        let handle: std::os::raw::c_int = unsafe { ffi::vsl_frame_handle(self.ptr) };
        return handle as i32;
    }

    pub fn paddr(&self) -> isize {
        return unsafe { ffi::vsl_frame_paddr(self.ptr) };
    }

    pub fn mmap(&self) -> &[u8] {
        let mut size: usize = 0;
        return unsafe {
            from_raw_parts_mut(
                ffi::vsl_frame_mmap(self.ptr, &mut size) as *mut u8,
                self.size() as usize,
            )
        }; // Add error checking to make sure the mmap is not zero
    }

    pub fn munmap(&self) {
        return unsafe { ffi::vsl_frame_munmap(self.ptr) };
    }

    pub fn get_ptr(&self) -> *mut ffi::VSLFrame {
        return self.ptr.clone();
    }
}

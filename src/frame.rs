use crate::client;
use std::{
    error::Error,
    ffi::{CStr, CString},
    io,
    os::fd::RawFd,
    path::Path,
    ptr, slice,
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

unsafe impl Send for Frame {}

impl Frame {
    pub fn new(
        width: u32,
        height: u32,
        stride: u32,
        fourcc_str: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let buf = fourcc_str.as_bytes();
        if buf.len() != 4 {
            return Err("fourcc must be 4 character ascii code".into());
        }
        let mut fourcc: u32 = 0;
        for i in 0..buf.len() {
            fourcc += (buf[i] as u32) << i * 8;
        }

        let ptr = unsafe {
            ffi::vsl_frame_init(width, height, stride, fourcc, std::ptr::null_mut(), None)
        };

        if ptr.is_null() {
            let err = io::Error::last_os_error();
            return Err(Box::new(err));
        }
        return Ok(Frame { ptr });
    }

    pub fn alloc(&self, path: Option<&Path>) -> Result<(), Box<dyn Error>> {
        let path_ptr;
        if let Some(path) = path {
            let path = path.to_str().unwrap();
            let path = CString::new(path).unwrap();
            path_ptr = path.into_raw();
        } else {
            path_ptr = ptr::null_mut();
        }
        let ret = unsafe { ffi::vsl_frame_alloc(self.ptr, path_ptr) } as i32;
        if ret != 0 {
            let err = io::Error::last_os_error();
            return Err(Box::new(err));
        }
        return Ok(());
    }

    pub fn wrap(ptr: *mut ffi::VSLFrame) -> Result<Self, ()> {
        if ptr.is_null() {
            return Err(());
        }

        return Ok(Frame { ptr });
    }

    pub fn wait(client: &client::Client, until: i64) -> Result<Self, Box<dyn Error>> {
        let wrapper = client.get_frame(until)?;
        return Ok(Frame { ptr: wrapper.ptr });
    }

    pub fn trylock(&self) -> Result<(), Box<dyn Error>> {
        let ret = unsafe { ffi::vsl_frame_trylock(self.ptr) };
        if ret != 0 {
            let err = io::Error::last_os_error();
            return Err(Box::new(err));
        }
        return Ok(());
    }

    pub fn unlock(&self) -> Result<(), Box<dyn Error>> {
        if unsafe { ffi::vsl_frame_unlock(self.ptr) as i32 } == -1 {
            let err = io::Error::last_os_error();
            return Err(Box::new(err));
        }
        return Ok(());
    }

    pub fn sync(&self, enable: bool, mode: i32) -> Result<(), Box<dyn Error>> {
        let ret = unsafe { ffi::vsl_frame_sync(self.ptr, enable as i32, mode) };
        if ret >= 0 {
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

    /*
    pub fn stride(&self) -> i32 {
        return unsafe { ffi::vsl_frame_stride(self.ptr) as i32};
    }
    */

    pub fn handle(&self) -> i32 {
        let handle: std::os::raw::c_int = unsafe { ffi::vsl_frame_handle(self.ptr) };
        return handle as i32;
    }

    pub fn paddr(&self) -> Option<isize> {
        let ret = unsafe { ffi::vsl_frame_paddr(self.ptr) };
        if ret == -1 {
            return None;
        }
        return Some(ret);
    }

    pub fn path(&self) -> Option<&str> {
        let ret = unsafe { ffi::vsl_frame_path(self.ptr) };
        if ret.is_null() {
            return None;
        }
        let path = unsafe {
            match CStr::from_ptr(ret).to_str() {
                Ok(path) => path,
                Err(_) => {
                    return None;
                }
            }
        };
        return Some(path);
    }

    pub fn mmap(&self) -> Result<&[u8], ()> {
        let ptr = unsafe { ffi::vsl_frame_mmap(self.ptr, std::ptr::null_mut::<usize>()) };
        if ptr.is_null() || self.size() == 0 {
            return Err(());
        }
        return Ok(unsafe { slice::from_raw_parts(ptr as *const u8, self.size() as usize) });
    }

    pub fn mmap_mut(&self) -> Result<&mut [u8], ()> {
        let mut size: usize = 0;
        let ptr = unsafe { ffi::vsl_frame_mmap(self.ptr, &mut size as *mut usize) };
        if ptr.is_null() || size == 0 {
            return Err(());
        }
        return Ok(unsafe { slice::from_raw_parts_mut(ptr as *mut u8, size) });
    }

    pub fn munmap(&self) {
        return unsafe { ffi::vsl_frame_munmap(self.ptr) };
    }

    pub fn attach(&self, fd: RawFd, size: usize, offset: usize) -> Result<(), Box<dyn Error>> {
        let ret = unsafe { ffi::vsl_frame_attach(self.ptr, fd, size, offset) };
        if ret < 0 {
            let err = io::Error::last_os_error();
            return Err(Box::new(err));
        }
        return Ok(());
    }

    pub fn get_ptr(&self) -> *mut ffi::VSLFrame {
        return self.ptr.clone();
    }
}

impl TryFrom<*mut ffi::VSLFrame> for Frame {
    type Error = ();

    fn try_from(ptr: *mut ffi::VSLFrame) -> Result<Self, Self::Error> {
        if ptr.is_null() {
            return Err(());
        }
        return Ok(Frame { ptr });
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            ffi::vsl_frame_release(self.ptr);
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, Rng};
    use std::{
        fs::{self, File},
        io::Write,
        os::fd::AsRawFd,
    };

    #[test]
    fn frame() {
        //let fourcc = 0x33424752 as u32; //Hex for RGB3
        let frame = Frame::new(640, 480, 0, "RGB3").unwrap();

        assert_eq!(frame.width(), 640);
        assert_eq!(frame.height(), 480);
        assert_eq!(frame.fourcc(), 0x33424752);
        assert_eq!(frame.path(), None);

        frame.alloc(None).unwrap();

        assert_eq!(frame.size(), 640 * 480 * 3);

        let mem: &mut [u8] = frame.mmap_mut().unwrap();
        let mut rng = rand::thread_rng();
        for elem in &mut *mem {
            let num: u8 = rng.gen();
            *elem = num;
        }
        let mem2 = frame.mmap().unwrap();
        for i in 0..mem.len() {
            assert_eq!(mem[i], mem2[i]);
        }

        let frame2 = Frame::new(640, 480, 0, "RGB3").unwrap();
        frame2
            .attach(frame.handle(), frame.size() as usize, 0)
            .unwrap();
        let v2: &mut [u8] = frame2.mmap_mut().unwrap();
        for i in 0..mem.len() {
            assert_eq!(mem[i], v2[i]);
        }

        for elem in &mut *v2 {
            let num: u8 = rng.gen();
            *elem = num;
        }
        assert_eq!(mem[0], v2[0]);
        assert_eq!(mem2[0], v2[0]);
    }

    #[test]
    fn attach_file() {
        let frame = Frame::new(640, 480, 0, "RGB3").unwrap();

        let mut expect = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..(frame.height() * frame.width() * 3) {
            expect.push(rng.gen::<u8>() as u8);
        }
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open("./temp.txt")
            .unwrap();
        file.write_all(&expect).unwrap();

        frame
            .attach(
                file.as_raw_fd(),
                (frame.height() * frame.width() * 3) as usize,
                0,
            )
            .unwrap();

        let mem = frame.mmap().unwrap();

        for i in 0..mem.len() {
            assert_eq!(mem[i], expect[i])
        }
        if let Err(_) = fs::remove_file("./temp.txt") {
            panic!("Test succeeded but file \"./temp.txt\" was not deleted");
        }
    }

    #[test]
    fn bad_attach() {
        let frame = Frame::new(640, 480, 0, "RGB3").unwrap();

        match frame.attach(-1, 1 as usize, 0) {
            Ok(_) => {
                panic!("Failed")
            }
            Err(_) => {}
        };

        match frame.attach(9000, 1 as usize, 0) {
            Ok(_) => {
                panic!("Failed")
            }
            Err(_) => {}
        };
    }

    #[test]
    fn fourcc() {}

    #[test]
    fn bad_fourcc() {}

    #[test]
    fn nodma() {}

    #[test]
    fn invalid_shm_name() {}
}

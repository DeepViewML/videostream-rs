use crate::{frame::Frame, Error};
use std::{
    ffi::{c_int, c_void},
    ptr::null_mut,
};
use videostream_sys::{self as ffi, vsl_frame};
pub struct Decoder {
    ptr: *mut ffi::VSLDecoder,
}

#[derive(Debug, Clone, Copy)]
pub enum DecoderInputCodec {
    H264 = 0,
    HEVC = 1,
}

impl Decoder {
    pub fn create(input_codec: DecoderInputCodec, fps: c_int) -> Self {
        return Decoder {
            ptr: unsafe { ffi::vsl_decoder_create(input_codec as u32, fps) },
        };
    }

    pub fn decode_frame(&self, data: &[u8]) -> Result<(usize, Option<Frame>), Box<dyn Error>> {
        let mut output_frame: *mut vsl_frame = null_mut();
        let output_frame_ptr: *mut *mut vsl_frame = &mut output_frame;
        let len = data.len() as u32;
        let mut bytes_used: usize = 0;
        let ret_code = unsafe {
            ffi::vsl_decode_frame(
                self.ptr,
                data.as_ptr() as *const c_void,
                len,
                &mut bytes_used,
                output_frame_ptr,
            )
        };
        let output_frame = match Frame::wrap(output_frame) {
            Ok(v) => Some(v),
            Err(_) => None,
        };
        return Ok((bytes_used, output_frame));
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe {
            ffi::vsl_decoder_release(self.ptr);
        }
    }
}

use crate::{encoder::VSLRect, frame::Frame, Error};
use std::{
    ffi::{c_int, c_void},
    io,
    ptr::null_mut,
};
use videostream_sys::{
    self as ffi, vsl_frame, VSLDecoderRetCode_VSL_DEC_ERR, VSLDecoderRetCode_VSL_DEC_FRAME_DEC,
    VSLDecoderRetCode_VSL_DEC_INIT_INFO,
};
pub struct Decoder {
    ptr: *mut ffi::VSLDecoder,
}

#[derive(Debug, Clone, Copy)]
pub enum DecoderInputCodec {
    H264 = 0,
    HEVC = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum DecodeReturnCode {
    Success,
    Initialized,
    FrameDecoded,
}

impl Decoder {
    pub fn create(input_codec: DecoderInputCodec, fps: c_int) -> Self {
        return Decoder {
            ptr: unsafe { ffi::vsl_decoder_create(input_codec as u32, fps) },
        };
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::vsl_decoder_width(self.ptr) }
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::vsl_decoder_height(self.ptr) }
    }

    pub fn crop(&self) -> VSLRect {
        unsafe { ffi::vsl_decoder_crop(self.ptr) }
    }

    pub fn decode_frame(
        &self,
        data: &[u8],
    ) -> Result<(DecodeReturnCode, usize, Option<Frame>), Box<dyn Error>> {
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
        if ret_code & VSLDecoderRetCode_VSL_DEC_ERR > 0 {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Decoder Error",
            )));
        }
        let mut return_msg = DecodeReturnCode::Success;
        if ret_code & VSLDecoderRetCode_VSL_DEC_FRAME_DEC > 0 {
            return_msg = DecodeReturnCode::FrameDecoded;
        }

        if ret_code & VSLDecoderRetCode_VSL_DEC_INIT_INFO > 0 {
            return_msg = DecodeReturnCode::Initialized;
        }

        Ok((return_msg, bytes_used, output_frame))
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe {
            ffi::vsl_decoder_release(self.ptr);
        }
    }
}

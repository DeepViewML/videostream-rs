#!/bin/sh

bindgen --allowlist-function 'vsl_.*' videostream.h > src/ffi.rs
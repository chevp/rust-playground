//! Hand-written 1:1 mapping of `c-shim/nuna_core.h`.

#![allow(non_camel_case_types)]

use std::os::raw::c_char;

#[repr(C)]
pub struct nuna_runtime {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn nuna_init(config_path: *const c_char) -> *mut nuna_runtime;
    pub fn nuna_run(rt: *mut nuna_runtime) -> i32;
    pub fn nuna_close(rt: *mut nuna_runtime);
}

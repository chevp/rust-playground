//! Safe Rust wrapper over the 3-function nuna-core launcher ABI.
//!
//! RAII handle, owned strings, no leaked unsafe. Whole shell-side surface
//! fits on one screen — that's the point of Model A.

use std::ffi::{CString, NulError};
use std::ptr;

use crate::ffi;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("nuna_init returned null (bad config path?)")]
    InitFailed,
    #[error("config path contained interior NUL: {0}")]
    Nul(#[from] NulError),
}

pub struct Runtime {
    raw: *mut ffi::nuna_runtime,
}

impl Runtime {
    pub fn init(config_path: &str) -> Result<Self, Error> {
        let c = CString::new(config_path)?;
        let raw = unsafe { ffi::nuna_init(c.as_ptr()) };
        if raw.is_null() {
            return Err(Error::InitFailed);
        }
        Ok(Self { raw })
    }

    /// Blocks until the engine quits. Returns the engine exit code (0 = normal).
    pub fn run(&mut self) -> i32 {
        unsafe { ffi::nuna_run(self.raw) }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::nuna_close(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

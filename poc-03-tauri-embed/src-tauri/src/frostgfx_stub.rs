//! Stand-in for the real frostgfx Engine wrapper.
//!
//! Mirrors the shape of `poc_01_c_abi_bindgen::Engine` so this file can be
//! deleted and replaced with `use poc_01_c_abi_bindgen::{Engine, EngineConfig};`
//! once the C ABI exists and is wired into this workspace.

use std::ffi::c_void;

#[derive(Debug)]
pub enum Error {
    AlreadyInitialized,
    NotInitialized,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EngineState {
    Created,
    Ready,
    Running,
}

#[derive(Default, Clone, Debug)]
#[allow(dead_code)] // populated for parity with the real wrapper; stub doesn't read them
pub struct EngineConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub parent_window_handle: *mut c_void,
}

pub struct Engine {
    state: EngineState,
    _hwnd: *mut c_void,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            state: EngineState::Created,
            _hwnd: std::ptr::null_mut(),
        }
    }

    pub fn initialize(&mut self, cfg: EngineConfig) -> Result<(), Error> {
        if self.state != EngineState::Created {
            return Err(Error::AlreadyInitialized);
        }
        self._hwnd = cfg.parent_window_handle;
        self.state = EngineState::Ready;
        Ok(())
    }

    pub fn activate(&mut self) -> Result<(), Error> {
        if self.state == EngineState::Created {
            return Err(Error::NotInitialized);
        }
        self.state = EngineState::Running;
        Ok(())
    }

    pub fn state(&self) -> EngineState {
        self.state
    }
}

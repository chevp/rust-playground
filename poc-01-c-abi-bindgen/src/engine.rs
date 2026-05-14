//! Safe Rust wrapper over the frostgfx C ABI.
//!
//! RAII handle, owned strings, `Result` returns. The Rust code in
//! `examples/hello.rs` should look idiomatic — all unsafe is confined here.

use std::ffi::{CString, NulError};
use std::os::raw::c_void;
use std::ptr;

use crate::ffi;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("frostgfx call failed: {0:?}")]
    Call(ffi::frostgfx_status),
    #[error("string contained interior NUL: {0}")]
    Nul(#[from] NulError),
    #[error("frostgfx_engine_new returned null")]
    NullEngine,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EngineState {
    Created,
    Ready,
    SceneLoaded,
    Running,
    ShuttingDown,
    Destroyed,
}

impl From<ffi::frostgfx_engine_state> for EngineState {
    fn from(s: ffi::frostgfx_engine_state) -> Self {
        use ffi::frostgfx_engine_state::*;
        match s {
            CREATED => EngineState::Created,
            READY => EngineState::Ready,
            SCENE_LOADED => EngineState::SceneLoaded,
            RUNNING => EngineState::Running,
            SHUTTING_DOWN => EngineState::ShuttingDown,
            DESTROYED => EngineState::Destroyed,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct EngineConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub validation_enabled: bool,
    pub headless: bool,
    pub asset_root: String,
    pub shader_dir: String,
    pub scene_file: String,
    /// HWND on Win32, NSView* on macOS, etc. `null` means the engine creates its own window.
    pub parent_window_handle: *mut c_void,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Camera {
    pub pos: [f32; 3],
    pub rot: [f32; 3],
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct Engine {
    raw: *mut ffi::frostgfx_engine,
    // Hold CStrings for as long as the engine holds borrowed pointers from initialize().
    // The real shim copies inputs, so this is defensive but cheap.
    _strings: Vec<CString>,
}

impl Engine {
    pub fn new() -> Result<Self, Error> {
        let raw = unsafe { ffi::frostgfx_engine_new() };
        if raw.is_null() {
            return Err(Error::NullEngine);
        }
        Ok(Self {
            raw,
            _strings: Vec::new(),
        })
    }

    pub fn initialize(&mut self, cfg: &EngineConfig) -> Result<(), Error> {
        let mut keepalive: Vec<CString> = Vec::with_capacity(5);
        let title = push_cstring(&mut keepalive, &cfg.window_title)?;
        let asset_root = push_cstring(&mut keepalive, &cfg.asset_root)?;
        let shader_dir = push_cstring(&mut keepalive, &cfg.shader_dir)?;
        let scene_file = push_cstring(&mut keepalive, &cfg.scene_file)?;

        let c_cfg = ffi::frostgfx_config {
            window_title: title,
            window_width: cfg.window_width,
            window_height: cfg.window_height,
            fullscreen: cfg.fullscreen as u8,
            vsync: cfg.vsync as u8,
            validation_enabled: cfg.validation_enabled as u8,
            headless: cfg.headless as u8,
            asset_root,
            shader_dir,
            scene_file,
            parent_window_handle: cfg.parent_window_handle,
        };

        let status = unsafe { ffi::frostgfx_initialize(self.raw, &c_cfg) };
        ok(status)?;
        self._strings = keepalive;
        Ok(())
    }

    pub fn activate(&mut self) -> Result<(), Error> {
        ok(unsafe { ffi::frostgfx_activate(self.raw) })
    }

    pub fn tick(&mut self) -> Result<(), Error> {
        ok(unsafe { ffi::frostgfx_tick(self.raw) })
    }

    pub fn shutdown(&mut self) -> Result<(), Error> {
        ok(unsafe { ffi::frostgfx_shutdown(self.raw) })
    }

    pub fn load_scene(
        &mut self,
        scene_uri: &str,
        game_root: Option<&str>,
        preview_only: bool,
    ) -> Result<(), Error> {
        let uri = CString::new(scene_uri)?;
        let root = game_root.map(CString::new).transpose()?;
        let root_ptr = root.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        ok(unsafe {
            ffi::frostgfx_load_scene(self.raw, uri.as_ptr(), root_ptr, preview_only as u8)
        })
    }

    pub fn update_camera(&mut self, cam: &Camera) -> Result<(), Error> {
        let c = ffi::frostgfx_camera {
            pos: cam.pos,
            rot: cam.rot,
            fov: cam.fov,
            znear: cam.znear,
            zfar: cam.zfar,
        };
        ok(unsafe { ffi::frostgfx_update_camera(self.raw, &c) })
    }

    pub fn state(&self) -> EngineState {
        unsafe { ffi::frostgfx_get_state(self.raw) }.into()
    }

    pub fn window_handle(&self) -> *mut c_void {
        unsafe { ffi::frostgfx_get_window_handle(self.raw) }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::frostgfx_engine_free(self.raw) };
        }
    }
}

fn push_cstring(bag: &mut Vec<CString>, s: &str) -> Result<*const std::os::raw::c_char, NulError> {
    if s.is_empty() {
        return Ok(ptr::null());
    }
    let c = CString::new(s)?;
    let p = c.as_ptr();
    bag.push(c);
    Ok(p)
}

fn ok(s: ffi::frostgfx_status) -> Result<(), Error> {
    if s == ffi::frostgfx_status::OK {
        Ok(())
    } else {
        Err(Error::Call(s))
    }
}

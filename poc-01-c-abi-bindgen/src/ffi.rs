//! Hand-written 1:1 mapping of `c-shim/frostgfx_c.h`.
//!
//! Swap for bindgen output by following the recipe in the crate README.

#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_void};

#[repr(C)]
pub struct frostgfx_engine {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum frostgfx_status {
    OK = 0,
    ERROR = 1,
    NOT_READY = 2,
    NOT_FOUND = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum frostgfx_engine_state {
    CREATED = 0,
    READY = 1,
    SCENE_LOADED = 2,
    RUNNING = 3,
    SHUTTING_DOWN = 4,
    DESTROYED = 5,
}

#[repr(C)]
pub struct frostgfx_config {
    pub window_title: *const c_char,
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: u8,
    pub vsync: u8,
    pub validation_enabled: u8,
    pub headless: u8,
    pub asset_root: *const c_char,
    pub shader_dir: *const c_char,
    pub scene_file: *const c_char,
    pub parent_window_handle: *mut c_void,
}

#[repr(C)]
pub struct frostgfx_camera {
    pub pos: [f32; 3],
    pub rot: [f32; 3],
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
}

extern "C" {
    pub fn frostgfx_engine_new() -> *mut frostgfx_engine;
    pub fn frostgfx_engine_free(e: *mut frostgfx_engine);

    pub fn frostgfx_initialize(
        e: *mut frostgfx_engine,
        cfg: *const frostgfx_config,
    ) -> frostgfx_status;
    pub fn frostgfx_activate(e: *mut frostgfx_engine) -> frostgfx_status;
    pub fn frostgfx_shutdown(e: *mut frostgfx_engine) -> frostgfx_status;
    pub fn frostgfx_tick(e: *mut frostgfx_engine) -> frostgfx_status;

    pub fn frostgfx_load_scene(
        e: *mut frostgfx_engine,
        scene_uri: *const c_char,
        game_root: *const c_char,
        preview_only: u8,
    ) -> frostgfx_status;
    pub fn frostgfx_reload_scene(
        e: *mut frostgfx_engine,
        scene_uri: *const c_char,
    ) -> frostgfx_status;

    pub fn frostgfx_update_camera(
        e: *mut frostgfx_engine,
        cam: *const frostgfx_camera,
    ) -> frostgfx_status;

    pub fn frostgfx_get_state(e: *const frostgfx_engine) -> frostgfx_engine_state;
    pub fn frostgfx_get_window_handle(e: *const frostgfx_engine) -> *mut c_void;
    pub fn frostgfx_last_error(
        e: *const frostgfx_engine,
        buf: *mut c_char,
        cap: usize,
    ) -> usize;
}

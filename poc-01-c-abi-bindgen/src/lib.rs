//! Rust bindings for the proposed frostgfx C ABI.
//!
//! See `c-shim/frostgfx_c.h` for the contract this crate wraps.

pub mod ffi;
pub mod engine;

pub use engine::{Engine, EngineConfig, EngineState, Camera, Error};

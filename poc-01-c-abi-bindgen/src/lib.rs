//! Rust bindings for the proposed frostgfx C ABI.
//!
//! See `c-shim/frostgfx_c.h` for the contract this crate wraps.

pub mod engine;
pub mod ffi;

pub use engine::{Camera, Engine, EngineConfig, EngineState, Error};

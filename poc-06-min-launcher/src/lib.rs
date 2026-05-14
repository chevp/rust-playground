//! Minimal launcher-shape FFI to a hypothetical `nuna-core` C library.
//!
//! See `c-shim/nuna_core.h` for the contract and README.md for context on
//! when this shape is sufficient vs. when you need Model B (IPC) or Model C
//! (embedded render surface).

pub mod ffi;
pub mod engine;

pub use engine::{Error, Runtime};

//! POC 5 — calls the real `frostgfx.dll` from Rust via cxx-rs.
//!
//! frostgfx is included as a git submodule at `../frostgfx/`. The submodule
//! provides headers; the .lib / .dll come from the user's local frostgfx
//! build (see `build.rs` for path discovery).

#[cxx::bridge(namespace = "fgx_bridge")]
#[allow(dead_code)] // initialize() + EngineConfigDto are intentionally unused in the minimal demo
mod ffi {
    struct EngineConfigDto {
        window_title: String,
        window_width: u32,
        window_height: u32,
        vsync: bool,
        validation_enabled: bool,
        headless: bool,
    }

    unsafe extern "C++" {
        include!("frostgfx_bridge.hpp");

        type FrostEngineWrapper;

        fn new_engine() -> UniquePtr<FrostEngineWrapper>;
        fn frostgfx_version() -> String;

        fn state(self: &FrostEngineWrapper) -> i32;
        fn initialize(self: Pin<&mut FrostEngineWrapper>, cfg: &EngineConfigDto) -> bool;
    }
}

fn state_name(s: i32) -> &'static str {
    // Matches coregfx::api::EngineState (FrostResponse.hpp).
    match s {
        0 => "Created",
        1 => "Ready",
        2 => "SceneLoaded",
        3 => "Running",
        4 => "ShuttingDown",
        5 => "Destroyed",
        _ => "?",
    }
}

fn main() {
    println!("frostgfx version (C ABI):  {}", ffi::frostgfx_version());

    let engine = ffi::new_engine();
    let s0 = engine.state();
    println!("FrostEngine state (uninit): {} ({})", s0, state_name(s0));

    // NOTE: calling engine.initialize() here would invoke CmdInitialize,
    // which tries to open Vulkan + create a window. In this minimal POC
    // we stop at proving:
    //   1. frostgfx.dll loaded successfully (version call worked)
    //   2. FrostEngine constructor crossed the cxx-rs boundary cleanly
    //   3. getState() round-trips through the bridge
    // Wiring the full initialize/activate/load_scene flow needs a host
    // window (see poc-03-tauri-embed for HWND embedding) and is left as
    // the next step.
    println!("\nlinkage verified — Rust ↔ cxx-rs ↔ C++ shim ↔ frostgfx.dll");

    // Suppress unused-variable warning while keeping the reference alive.
    let _keep_alive = engine;
}

//! End-to-end demo of consuming the frostgfx C ABI from safe Rust.
//!
//! Runs against the no-op stub in `c-shim/frostgfx_stub.c`. Pointing this at
//! the real frostgfx C shim is purely a link-target change.

use poc_01_c_abi_bindgen::{Camera, Engine, EngineConfig, EngineState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::new()?;
    assert_eq!(engine.state(), EngineState::Created);

    engine.initialize(&EngineConfig {
        window_title: "frostgfx-rust-poc".to_string(),
        window_width: 1280,
        window_height: 720,
        vsync: true,
        validation_enabled: true,
        asset_root: "C:/chevp/frameworks/frostgfx/assets".to_string(),
        ..Default::default()
    })?;
    assert_eq!(engine.state(), EngineState::Ready);
    println!("initialized — window handle: {:?}", engine.window_handle());

    engine.load_scene("scenes/hello.scene.xml", None, false)?;
    assert_eq!(engine.state(), EngineState::SceneLoaded);
    println!("scene loaded");

    engine.update_camera(&Camera {
        pos: [0.0, 1.7, -5.0],
        rot: [0.0, 0.0, 0.0],
        fov: 60.0,
        znear: 0.1,
        zfar: 1000.0,
    })?;
    println!("camera updated");

    engine.activate()?;
    assert_eq!(engine.state(), EngineState::Running);
    println!("engine running");

    engine.tick()?;
    engine.shutdown()?;
    println!("shutdown ok");

    Ok(())
}

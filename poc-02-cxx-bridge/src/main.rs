//! cxx bridge to a simplified slice of frostgfx's FrostEngine.
//!
//! Only the methods that DON'T need std::variant / std::future are reachable
//! this way; for anything richer, use the C ABI approach in poc-01.

#[cxx::bridge(namespace = "frostgfx_simple")]
mod ffi {
    // POD types that cross the bridge by value.
    struct Camera {
        pos: [f32; 3],
        rot: [f32; 3],
        fov: f32,
        znear: f32,
        zfar: f32,
    }

    struct EngineConfigDto {
        window_title: String,
        window_width: u32,
        window_height: u32,
        vsync: bool,
        validation_enabled: bool,
        asset_root: String,
        shader_dir: String,
    }

    unsafe extern "C++" {
        include!("frostgfx_simple.hpp");

        type FrostEngineSimple;

        fn new_engine() -> UniquePtr<FrostEngineSimple>;

        fn initialize(self: Pin<&mut FrostEngineSimple>, cfg: &EngineConfigDto) -> bool;
        fn load_scene(self: Pin<&mut FrostEngineSimple>, scene_uri: &str, preview_only: bool) -> bool;
        fn update_camera(self: Pin<&mut FrostEngineSimple>, cam: &Camera) -> bool;
        fn activate(self: Pin<&mut FrostEngineSimple>) -> bool;
        fn shutdown(self: Pin<&mut FrostEngineSimple>) -> bool;

        fn state(self: &FrostEngineSimple) -> i32;
        fn last_error(self: &FrostEngineSimple) -> String;
        fn list_entities(self: &FrostEngineSimple) -> Vec<String>;
    }
}

fn main() {
    let mut engine = ffi::new_engine();

    let cfg = ffi::EngineConfigDto {
        window_title: "frostgfx-cxx-poc".into(),
        window_width: 1280,
        window_height: 720,
        vsync: true,
        validation_enabled: true,
        asset_root: "C:/chevp/frameworks/frostgfx/assets".into(),
        shader_dir: "C:/chevp/frameworks/frostgfx/shaders".into(),
    };

    assert!(engine.pin_mut().initialize(&cfg));
    println!("initialized, state = {}", engine.state());

    // Demonstrate error reporting across the cxx bridge.
    let ok = engine.pin_mut().load_scene("", false);
    assert!(!ok);
    println!("empty scene rejected; last_error = {:?}", engine.last_error());

    assert!(engine.pin_mut().load_scene("scenes/hello.scene.xml", false));
    println!("scene loaded, state = {}", engine.state());

    let cam = ffi::Camera {
        pos: [0.0, 1.7, -5.0],
        rot: [0.0, 0.0, 0.0],
        fov: 60.0,
        znear: 0.1,
        zfar: 1000.0,
    };
    assert!(engine.pin_mut().update_camera(&cam));

    assert!(engine.pin_mut().activate());
    println!("running, state = {}", engine.state());

    let entities = engine.list_entities();
    println!("entities: {:?}", entities);

    engine.pin_mut().shutdown();
    println!("shutdown, state = {}", engine.state());
}

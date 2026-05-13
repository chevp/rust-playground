//! Tauri 2 app that demonstrates handing its native window handle to frostgfx
//! as `EngineConfig::parentWindowHandle`. The Engine wrapper is a stub here;
//! swap it for the real one from poc-01-c-abi-bindgen once the C ABI ships.

mod frostgfx_stub;

use std::ffi::c_void;

#[tauri::command]
fn start_engine(window: tauri::WebviewWindow) -> Result<String, String> {
    let parent_hwnd: *mut c_void = native_handle(&window)?;

    let mut engine = frostgfx_stub::Engine::new();
    engine
        .initialize(frostgfx_stub::EngineConfig {
            window_title: "frostgfx-via-tauri".into(),
            window_width: 1280,
            window_height: 720,
            parent_window_handle: parent_hwnd,
            ..Default::default()
        })
        .map_err(|e| format!("initialize: {e:?}"))?;

    engine.activate().map_err(|e| format!("activate: {e:?}"))?;

    Ok(format!(
        "engine initialized & running\nparent_window_handle = {:?}\nstate = {:?}",
        parent_hwnd,
        engine.state(),
    ))
}

#[cfg(target_os = "windows")]
fn native_handle(window: &tauri::WebviewWindow) -> Result<*mut c_void, String> {
    let hwnd = window.hwnd().map_err(|e| e.to_string())?;
    Ok(hwnd.0 as *mut c_void)
}

#[cfg(not(target_os = "windows"))]
fn native_handle(_window: &tauri::WebviewWindow) -> Result<*mut c_void, String> {
    // Real impl: extract NSView* on macOS, xcb_window_t on Linux, etc.
    // frostgfx's parentWindowHandle is `void*` — same call site, different
    // platform-specific cast.
    Ok(std::ptr::null_mut())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_engine])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

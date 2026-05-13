# poc-03-tauri-embed

Demonstrates how a [Tauri 2](https://tauri.app) desktop app would host
frostgfx by passing its native window handle as
`EngineConfig::parentWindowHandle`.

## The mechanism

frostgfx already supports embedding:

```cpp
// frameworks/frostgfx/include/frostgfx/api/FrostResponse.hpp
struct EngineConfig {
    // ...
    // Embedding: if set, engine renders into this existing window
    // instead of creating its own. Platform-specific (HWND on Win32).
    void* parentWindowHandle = nullptr;
};
```

Tauri owns the WebView window; we extract its native handle and hand it to
frostgfx as `parentWindowHandle`. The Vulkan swapchain then renders into
Tauri's window (or a child window owned by it), and the WebView paints UI
overlays via transparency.

## Layout

```
poc-03-tauri-embed/
  dist/index.html               ← the WebView frontend (vanilla JS, no npm)
  src-tauri/
    Cargo.toml                  ← cargo workspace member
    tauri.conf.json             ← bundle disabled, no icons required
    build.rs                    ← runs tauri-build
    src/
      main.rs                   ← entry — delegates to lib.rs
      lib.rs                    ← Tauri app + IPC command
      frostgfx_stub.rs          ← stand-in for the real Engine wrapper
```

## What the IPC command does

```rust
#[tauri::command]
fn start_engine(window: tauri::WebviewWindow) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    let parent_hwnd = window.hwnd().map_err(|e| e.to_string())?.0 as *mut c_void;
    #[cfg(not(target_os = "windows"))]
    let parent_hwnd = std::ptr::null_mut();   // would extract NSView*, xcb_window_t, etc.

    let mut engine = frostgfx_stub::Engine::new();
    engine.initialize(frostgfx_stub::EngineConfig {
        window_title: "frostgfx-via-tauri".into(),
        parent_window_handle: parent_hwnd,
        ..Default::default()
    })?;
    Ok(format!("engine initialized, hwnd = {:?}", parent_hwnd))
}
```

`frostgfx_stub` is a stand-in — replace it with the real wrapper from
[poc-01-c-abi-bindgen](../poc-01-c-abi-bindgen/) once the C ABI ships.

## Run

Requires [Tauri prerequisites for Windows](https://tauri.app/start/prerequisites/)
(MSVC build tools, WebView2 runtime — likely already installed on this machine
since frostgfx targets the same toolchain):

```pwsh
cargo run -p frostgfx-tauri-poc
```

A 1280×720 window opens with a "Start engine" button. Clicking it invokes the
Rust command and writes the HWND back into the page.

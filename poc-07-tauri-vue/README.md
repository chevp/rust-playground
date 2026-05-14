# poc-07-tauri-vue

Minimal [Tauri 2](https://tauri.app) + [Vue 3](https://vuejs.org) + [Vite](https://vite.dev)
desktop app. Contrasts with [poc-03-tauri-embed](../poc-03-tauri-embed/), which
uses a vanilla-JS frontend on purpose (no npm) and focuses on native-window
embedding for frostgfx.

## What it shows

- Vue 3 SFC frontend (`src/App.vue`) — counter + text input.
- Rust command `greet(name)` invoked via `@tauri-apps/api/core::invoke`.

```ts
// src/App.vue
const greeting = await invoke<string>("greet", { name: name.value });
```

```rust
// src-tauri/src/lib.rs
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! — from Rust via Tauri IPC")
}
```

## Layout

```
poc-07-tauri-vue/
  package.json                  ← Vue 3 + Vite + @tauri-apps/cli
  vite.config.ts                ← fixed port 1420 (Tauri convention)
  tsconfig.json
  index.html
  src/
    main.ts
    App.vue
    shims-vue.d.ts
  src-tauri/
    Cargo.toml                  ← cargo workspace member
    tauri.conf.json             ← beforeDevCommand: npm run dev
    build.rs
    capabilities/default.json
    src/
      main.rs
      lib.rs
```

## Run

One-time:

```sh
npm install
```

Dev (hot reload via Vite, Rust rebuild on change):

```sh
npm run tauri dev
```

This runs `npm run dev` (Vite on `:1420`) and `cargo run -p poc-07-tauri-vue`
together; the Rust window loads the Vite URL.

Release bundle:

```sh
npm run tauri build
```

## Requirements

Same as poc-03: [Tauri prerequisites](https://tauri.app/start/prerequisites/)
for your platform (WebView2 on Windows, WebKitGTK on Linux, none extra on macOS).

# rust-playground

Rust proof-of-concepts for integrating with the [frostgfx](../../frameworks/frostgfx/)
C++ graphics runtime.

## Status

**Scaffold only.** None of the POCs link against a real frostgfx build —
frostgfx has no C ABI yet (see [README §Reuse model](../../frameworks/frostgfx/README.md#reuse-model)
where C ABI is listed as *planned*). Each POC instead uses a local stub that
mimics the surface area frostgfx would expose, so the wiring pattern is
demonstrable today and can be re-pointed at the real library once its ABI lands.

## Why Rust integration is non-trivial

frostgfx's public C++ API ([`include/frostgfx/api/`](../../frameworks/frostgfx/include/frostgfx/api/))
exposes `coregfx::api::FrostEngine` with:

- `Response send(const Command& cmd)` where `Command = std::variant<40+ types>`
- `std::future<Response> sendAsync(const Command& cmd)`
- `void setGameLogic(std::function<void(SynthScene&, float)>)`

None of `std::variant`, `std::future`, `std::function`, `std::string`, `std::map`,
`std::vector` cross the C/Rust FFI boundary directly. The four POCs below
illustrate four different ways around that.

## POCs

| Folder | Pattern | Trade-offs |
|---|---|---|
| [poc-01-c-abi-bindgen](poc-01-c-abi-bindgen/) | Hand-written C ABI shim in C++, Rust calls it via raw FFI (with bindgen alternative) | Most flexible. Requires writing+maintaining a C shim layer in `frostgfx/include/frostgfx/c_api/` |
| [poc-02-cxx-bridge](poc-02-cxx-bridge/) | [`cxx`](https://cxx.rs) bridges Rust ↔ a small C++ subset of FrostEngine | Cleanest Rust code, but `std::variant`/`std::future` are unsupported — only a subset is reachable |
| [poc-03-tauri-embed](poc-03-tauri-embed/) | Tauri 2 app passes its window HWND to `EngineConfig::parentWindowHandle` | Solves windowing, not FFI. Must be combined with poc-01 or poc-02 |
| [poc-04-ipc-stdio](poc-04-ipc-stdio/) | frostgfx runs as a separate process, Rust client talks JSON-over-stdio | No FFI at all. Process-isolation cost; latency for high-frequency commands |
| [poc-07-tauri-vue](poc-07-tauri-vue/) | Tauri 2 + Vue 3 + Vite — minimal IPC roundtrip | Unrelated to frostgfx; reference setup for a real Vue frontend with Rust backend |

## Build

All POCs are members of a single Cargo workspace.

```pwsh
cargo check --workspace        # type-check everything
cargo build -p poc-01-c-abi-bindgen
cargo build -p poc-02-cxx-bridge
cargo build -p frostgfx-tauri-poc
cargo build -p frostgfx-ipc-client
```

Each POC has its own README with details on what's stubbed and what the
real-frostgfx integration would look like.

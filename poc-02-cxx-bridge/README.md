# poc-02-cxx-bridge

Calls a **simplified subset** of `coregfx::api::FrostEngine` from Rust via the
[`cxx`](https://cxx.rs) crate.

## What's reachable with cxx (and what isn't)

| frostgfx public type | cxx support | This POC's workaround |
|---|---|---|
| `std::string` | ✅ supported | passed as-is |
| `std::vector<T>` (`T` primitive or shared) | ✅ supported | used for entity IDs |
| `std::unique_ptr<T>` | ✅ for opaque C++ types | used to hold the engine |
| `std::variant<Cmd...>` | ❌ **not supported** | one method per command, no variant |
| `std::future<Response>` | ❌ **not supported** | sync only; async would need a callback bridge |
| `std::function<...>` | ❌ not directly | wrap behind a Rust `Box<dyn Fn>` via a trampoline |

The C++ class in `cpp/frostgfx_simple.{hpp,cpp}` is what a "cxx-friendly slice"
of FrostEngine would look like. Real wiring would forward each method to the
underlying `coregfx::api::FrostEngine`, performing the variant-construction
internally:

```cpp
// inside FrostEngineSimple::load_scene (real impl)
coregfx::api::CmdLoadScene cmd;
cmd.sceneUri = std::string(uri);
cmd.previewOnly = preview_only;
auto resp = real_engine_.send(cmd);   // variant constructed here, never crosses FFI
return resp.status == coregfx::api::ResponseStatus::Ok;
```

## Run

```pwsh
cargo run -p poc-02-cxx-bridge
```

The stub C++ class echoes inputs back; the Rust side prints what came across
the bridge.

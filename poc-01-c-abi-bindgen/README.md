# poc-01-c-abi-bindgen

Demonstrates how Rust would consume frostgfx through a **C ABI shim** —
the pattern listed as *planned* in [frostgfx/README.md](../../../frameworks/frostgfx/README.md#reuse-model)
(`include/frostgfx/c_api/`).

## What's here

```
c-shim/
  frostgfx_c.h        ← proposed C ABI surface (extern "C", POD only)
  frostgfx_stub.c     ← no-op implementation, just so this POC links
src/
  ffi.rs              ← hand-written `extern "C"` Rust bindings (1:1 with the header)
  engine.rs           ← safe Rust wrapper (RAII, Result-typed)
  lib.rs              ← re-exports
examples/
  hello.rs            ← end-to-end usage from Rust
build.rs              ← uses `cc` to compile the stub into a static lib
```

`cargo run --example hello` walks through: create engine → initialize → load
scene → update camera → query state → shutdown. Every call goes through the C
ABI and returns success because the C stub is a no-op.

## What the real C shim would look like

The stub functions in [c-shim/frostgfx_stub.c](c-shim/frostgfx_stub.c) would be
re-implemented in C++ inside frostgfx itself, e.g.
`frameworks/frostgfx/src/c_api/frostgfx_c.cpp`:

```cpp
// frostgfx_c.cpp
#include <frostgfx/api/FrostEngine.hpp>
#include "frostgfx_c.h"

using namespace coregfx::api;

struct frostgfx_engine { FrostEngine impl; };

extern "C" frostgfx_engine* frostgfx_engine_new() {
    return new frostgfx_engine{};
}

extern "C" frostgfx_status frostgfx_initialize(
    frostgfx_engine* e, const frostgfx_config* cfg)
{
    EngineConfig c;
    c.windowTitle  = cfg->window_title ? cfg->window_title : "";
    c.windowWidth  = cfg->window_width;
    c.windowHeight = cfg->window_height;
    c.parentWindowHandle = cfg->parent_window_handle;
    Response r = e->impl.send(CmdInitialize{ std::move(c) });
    return r.status == ResponseStatus::Ok
        ? FROSTGFX_OK : FROSTGFX_ERROR;
}
// ... one wrapper per command, std::variant fan-out lives here
```

The shim is the place where `std::variant`, `std::future`, and `std::string`
get unfolded into POD types Rust can speak. **The Rust side stays
identical** — only the implementation of `frostgfx_stub.c` swaps for the real
shim and the link target changes.

## Replacing the hand-written FFI with bindgen

Uncomment the `bindgen` build-dependency in `Cargo.toml`, then replace `build.rs`
with:

```rust
fn main() {
    cc::Build::new()
        .file("c-shim/frostgfx_stub.c")
        .include("c-shim")
        .compile("frostgfx_stub");

    let bindings = bindgen::Builder::default()
        .header("c-shim/frostgfx_c.h")
        .allowlist_function("frostgfx_.*")
        .allowlist_type("frostgfx_.*")
        .generate()
        .expect("bindgen failed");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
}
```

Then in `src/ffi.rs`:

```rust
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

This requires `libclang` on the build machine. The hand-written variant
shipped in this POC avoids that dependency.

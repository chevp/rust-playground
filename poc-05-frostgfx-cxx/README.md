# poc-05-frostgfx-cxx

**First POC that actually links the real `frostgfx.dll`** and calls into
`coregfx::api::FrostEngine` from Rust via a [cxx-rs](https://cxx.rs) bridge.

frostgfx is included here as a **git submodule** at
[frostgfx/](frostgfx/) so this POC is self-contained from a fresh
clone:

```pwsh
git clone --recurse-submodules https://github.com/chevp/rust-playground.git
```

or, in an existing clone:

```pwsh
git submodule update --init poc-05-frostgfx-cxx/frostgfx
```

## Layout

```
poc-05-frostgfx-cxx/
  Cargo.toml
  build.rs                 ← cxx-build + frostgfx include paths + link flags
  cpp/
    frostgfx_bridge.hpp    ← C++-side type forward decls
    frostgfx_bridge.cpp    ← wraps coregfx::api::FrostEngine, exposed to Rust
  src/
    main.rs                ← #[cxx::bridge] + demo
  frostgfx/                ← git submodule, contains include/ + .lib + .dll once built
```

## How the linkage works

cxx-rs compiles `cpp/frostgfx_bridge.cpp` as part of this crate's build.
The C++ TU `#include`s real frostgfx headers from the submodule
(`frostgfx/include/`) and calls actual `FrostEngine::send(CmdInitialize{…})`
internally — the variant construction stays on the C++ side and never
crosses the FFI boundary, which is what made the bridge tractable in
[POC 2](../poc-02-cxx-bridge/).

The Rust side links against `frostgfx.lib` (the import library produced by
the frostgfx build) and at runtime loads `frostgfx.dll` from the same
directory as the test binary.

## Prerequisites

frostgfx must be built first (the submodule provides headers; you need the
compiled DLL too). One-time setup:

```pwsh
cd poc-05-frostgfx-cxx/frostgfx
cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE=$env:VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake
cmake --build build --config Debug
```

`build.rs` will fail with a clear message if it can't find the artifacts.

The script looks for the build in this order:
1. `$FROSTGFX_BUILD_DIR` environment variable
2. `poc-05-frostgfx-cxx/frostgfx/build/` (submodule built in place)
3. `../../../frameworks/frostgfx/build/` (sibling repo in the chevp workspace)

## Run

```pwsh
cargo run -p poc-05-frostgfx-cxx
```

Expected output:

```
frostgfx version (C ABI):  0.1.0
FrostEngine state (uninit): 0 (Created)
FrostEngine state after init: 1 (Ready)
```

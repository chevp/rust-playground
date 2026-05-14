# poc-06-min-launcher

Minimal-C-ABI demo for the **launcher shape** (Model A from the architecture
discussion): a Rust shell that does nothing while the engine runs.

## The whole API

```c
nuna_runtime* nuna_init(const char* config_path);
int32_t       nuna_run(nuna_runtime* rt);   /* blocks until quit */
void          nuna_close(nuna_runtime* rt);
```

Three functions. That is the entire C surface.

## When this shape is enough

- The shell picks a config / scene, then hands control to the engine.
- The engine owns its own window and event loop.
- `nuna_run()` blocking the shell thread is acceptable.
- No live control needed during the session (no pause / load-scene /
  screenshot / inspector panel running in parallel).

Examples: "Steam-style picker that launches a game", "scene previewer that
opens a window per click", "test harness around an engine binary".

## When you outgrow it

| Need | Move to |
| --- | --- |
| Shell wants to send commands while engine runs | **Model B (IPC)** — see poc-04 |
| Engine must render into a window the shell owns (Tauri embed) | **Model C** — surface-handle ABI, ~15 functions, `tick()` instead of `run()` |
| Shell needs many engine queries / events | Larger C-ABI surface, see poc-01 for the pattern at scale |

## What's faked here

`c-shim/nuna_core_stub.c` is a no-op implementation. `nuna_init` prints the
config path, `nuna_run` prints 5 "tick" lines, `nuna_close` frees the handle.
The point is the **shape**, not the behaviour.

A real implementation lives in C++ inside `nuna-core` (a hypothetical lib
that bundles `frostgfx + nuna-engine + nuna-assets + nuna-synthxml +
extension-system` behind this same `nuna_core.h`).

## Run it

```
cargo run -p poc-06-min-launcher --example launcher
cargo run -p poc-06-min-launcher --example launcher -- my-config.nuna
```

Expected output:

```
[shell] launcher starting with config 'configs/default.nuna'
[nuna-core] init: loaded config 'configs/default.nuna'
[shell] handing control to engine...
[nuna-core] run: opening window, starting render loop
[nuna-core] tick 1
[nuna-core] tick 2
[nuna-core] tick 3
[nuna-core] tick 4
[nuna-core] tick 5
[nuna-core] run: user quit, returning
[shell] engine returned exit code 0
[nuna-core] close: releasing runtime
[shell] launcher done
```

## Comparison with siblings

- **poc-01-c-abi-bindgen** — same FFI technique, but ~15-function surface
  (init, scene, camera, queries, errors). Use as a template when this 3-fn
  shape isn't enough.
- **poc-04-ipc-stdio** — Model B: shell and engine are separate processes
  talking over stdio. No FFI at all.
- **poc-05-frostgfx-cxx** — links a real C++ artifact; this POC links a stub.

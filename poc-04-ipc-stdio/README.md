# poc-04-ipc-stdio

Out-of-process integration: **frostgfx runs as a separate process**, Rust
talks to it via newline-delimited JSON over stdin/stdout. No FFI, no C++/Rust
type marshalling — the process boundary handles isolation.

This maps naturally to frostgfx's existing I/O Layer model
([FrostEngine.hpp](../../../frameworks/frostgfx/include/frostgfx/api/FrostEngine.hpp)):

> Engine runs as a background process. Consumer interacts via Commands
> (with Response) and receives asynchronous Events. Analogy: Keithley I/O
> Layer — instrument measures continuously in background, consumer sends
> commands and reads results.

The transport is just stdio JSON here; in production it could be a named pipe,
gRPC, a Unix domain socket, or shared memory + a small control channel.

## Layout

```
poc-04-ipc-stdio/
  shared/         ← Cmd / Response / Event types, serde-tagged
  engine-stub/    ← Rust binary pretending to BE frostgfx (reads JSON from stdin)
  client/         ← Rust client: spawns engine-stub, sends commands, prints responses
```

## What the wire looks like

Client → engine (one JSON object per line):
```json
{"type":"Initialize","window_title":"frostgfx","width":1280,"height":720}
{"type":"LoadScene","scene_uri":"scenes/hello.scene.xml","preview_only":false}
{"type":"UpdateCamera","pos":[0,1.7,-5],"rot":[0,0,0],"fov":60,"znear":0.1,"zfar":1000}
{"type":"GetState"}
{"type":"Shutdown"}
```

Engine → client (responses + asynchronous events on the same stream, discriminated by `kind`):
```json
{"kind":"response","status":"Ok","id":1}
{"kind":"response","status":"Ok","id":2,"state":"SceneLoaded"}
{"kind":"event","name":"StateChanged","payload":{"new_state":"SceneLoaded"}}
```

## Run

```pwsh
cargo build -p frostgfx-engine-stub          # builds the engine binary
cargo run   -p frostgfx-ipc-client           # spawns the engine, drives a session
```

`client` discovers `engine-stub` by querying cargo's target directory
(`CARGO_TARGET_DIR` or `target/debug`). The whole cycle runs in milliseconds.

## When to choose this over FFI

- The engine is already a separate process (frostgfx headless / CI)
- You want crash isolation (engine OOM doesn't take down the host)
- You want language independence (any language with JSON + subprocess works)
- You can tolerate per-message latency on the order of microseconds-to-low-milliseconds

When to avoid: tight per-frame game logic callbacks (`setGameLogic` style) —
that path *needs* in-process FFI for cache locality.

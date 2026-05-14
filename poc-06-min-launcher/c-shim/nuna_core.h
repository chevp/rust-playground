/*
 * nuna_core.h - Minimal C ABI for a "launcher-shape" embedding.
 *
 * The smallest viable surface for Model A: shell hands control to the engine,
 * engine runs its own loop in this thread, returns when the user quits.
 *
 * Design rules (same as poc-01):
 *   - POD only. No C++ types.
 *   - Strings: NUL-terminated UTF-8 const char*.
 *   - Caller owns inputs; library copies what it needs.
 *
 * Use this shape when:
 *   - The shell only configures + launches (no live control).
 *   - The engine owns its own window and event loop.
 *   - run() blocking is acceptable to the shell thread.
 *
 * Outgrow this shape (Models B/C from the architecture discussion) when:
 *   - The shell needs to load_scene / pause / screenshot while engine runs.
 *   - The engine must render into a window the shell owns (Tauri embed).
 */
#ifndef NUNA_CORE_H
#define NUNA_CORE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Opaque handle to a configured runtime instance. */
typedef struct nuna_runtime nuna_runtime;

/* Create + configure a runtime from a config file path.
 * Returns NULL on failure (e.g. missing/invalid config). */
nuna_runtime* nuna_init(const char* config_path);

/* Run the engine's main loop until it exits. Returns the engine exit code
 * (0 = normal user quit). Blocks the calling thread for the entire session. */
int32_t nuna_run(nuna_runtime* rt);

/* Tear down the runtime. Idempotent; safe on NULL. */
void nuna_close(nuna_runtime* rt);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* NUNA_CORE_H */

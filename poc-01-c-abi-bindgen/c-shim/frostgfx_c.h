/*
 * frostgfx_c.h - Proposed C ABI for frostgfx.
 *
 * This header is the contract between Rust (and other non-C++ consumers) and
 * the frostgfx C++ implementation. It mirrors the subset of FrostEngine that
 * is reachable without std::variant / std::future / std::string in the API.
 *
 * Design rules:
 *   - POD types only. No C++ types.
 *   - All strings are NUL-terminated UTF-8 const char*.
 *   - Caller owns input buffers; library copies what it needs.
 *   - Output buffers use a caller-provided (buf, cap) pair + a returned len.
 *   - Errors return a frostgfx_status; the engine holds the last error message.
 *
 * Stable ABI: once shipped, append-only. No reordering, no widening of enums.
 */
#ifndef FROSTGFX_C_H
#define FROSTGFX_C_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct frostgfx_engine frostgfx_engine;

typedef enum frostgfx_status {
    FROSTGFX_OK         = 0,
    FROSTGFX_ERROR      = 1,
    FROSTGFX_NOT_READY  = 2,
    FROSTGFX_NOT_FOUND  = 3
} frostgfx_status;

typedef enum frostgfx_engine_state {
    FROSTGFX_STATE_CREATED       = 0,
    FROSTGFX_STATE_READY         = 1,
    FROSTGFX_STATE_SCENE_LOADED  = 2,
    FROSTGFX_STATE_RUNNING       = 3,
    FROSTGFX_STATE_SHUTTING_DOWN = 4,
    FROSTGFX_STATE_DESTROYED     = 5
} frostgfx_engine_state;

/* Mirrors coregfx::api::EngineConfig — subset that crosses FFI cleanly. */
typedef struct frostgfx_config {
    const char* window_title;   /* may be NULL */
    uint32_t    window_width;
    uint32_t    window_height;
    uint8_t     fullscreen;     /* 0 / 1 */
    uint8_t     vsync;          /* 0 / 1 */
    uint8_t     validation_enabled;
    uint8_t     headless;
    const char* asset_root;     /* may be NULL */
    const char* shader_dir;     /* may be NULL */
    const char* scene_file;     /* may be NULL */
    void*       parent_window_handle; /* HWND on Win32, NSView* on macOS, etc. */
} frostgfx_config;

typedef struct frostgfx_camera {
    float pos[3];
    float rot[3];
    float fov;
    float znear;
    float zfar;
} frostgfx_camera;

/* ── Lifecycle ─────────────────────────────────────────────────── */

frostgfx_engine* frostgfx_engine_new(void);
void             frostgfx_engine_free(frostgfx_engine* e);

frostgfx_status  frostgfx_initialize(frostgfx_engine* e, const frostgfx_config* cfg);
frostgfx_status  frostgfx_activate(frostgfx_engine* e);
frostgfx_status  frostgfx_shutdown(frostgfx_engine* e);
frostgfx_status  frostgfx_tick(frostgfx_engine* e);

/* ── Scene ─────────────────────────────────────────────────────── */

frostgfx_status  frostgfx_load_scene(frostgfx_engine* e,
                                     const char* scene_uri,
                                     const char* game_root,
                                     uint8_t preview_only);

frostgfx_status  frostgfx_reload_scene(frostgfx_engine* e, const char* scene_uri);

/* ── Camera ────────────────────────────────────────────────────── */

frostgfx_status  frostgfx_update_camera(frostgfx_engine* e, const frostgfx_camera* cam);

/* ── Queries ───────────────────────────────────────────────────── */

frostgfx_engine_state frostgfx_get_state(const frostgfx_engine* e);

/* Returns the native window handle (HWND on Win32) or NULL before init. */
void*            frostgfx_get_window_handle(const frostgfx_engine* e);

/* Copies the last error into `buf` (NUL-terminated, truncated to cap-1).
 * Returns the number of bytes written (excluding NUL). */
size_t           frostgfx_last_error(const frostgfx_engine* e, char* buf, size_t cap);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* FROSTGFX_C_H */

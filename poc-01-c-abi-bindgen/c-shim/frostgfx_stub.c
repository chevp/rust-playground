/*
 * frostgfx_stub.c - No-op implementation of the proposed frostgfx C ABI.
 *
 * Exists only so the Rust side can link and demonstrate the call pattern
 * end-to-end. The real implementation will live in frostgfx itself, written
 * in C++ on top of coregfx::api::FrostEngine (see README.md).
 */
#include "frostgfx_c.h"
#include <stdlib.h>
#include <string.h>

struct frostgfx_engine {
    frostgfx_engine_state state;
    char last_error[256];
    void* window_handle;
};

frostgfx_engine* frostgfx_engine_new(void) {
    frostgfx_engine* e = (frostgfx_engine*)calloc(1, sizeof(frostgfx_engine));
    if (!e) return NULL;
    e->state = FROSTGFX_STATE_CREATED;
    return e;
}

void frostgfx_engine_free(frostgfx_engine* e) {
    free(e);
}

frostgfx_status frostgfx_initialize(frostgfx_engine* e, const frostgfx_config* cfg) {
    if (!e || !cfg) return FROSTGFX_ERROR;
    /* Real impl would: build EngineConfig, send CmdInitialize, open Vulkan,
     * create or attach window using cfg->parent_window_handle. */
    e->window_handle = cfg->parent_window_handle; /* echo back for the POC */
    e->state = FROSTGFX_STATE_READY;
    return FROSTGFX_OK;
}

frostgfx_status frostgfx_activate(frostgfx_engine* e) {
    if (!e) return FROSTGFX_ERROR;
    e->state = FROSTGFX_STATE_RUNNING;
    return FROSTGFX_OK;
}

frostgfx_status frostgfx_shutdown(frostgfx_engine* e) {
    if (!e) return FROSTGFX_ERROR;
    e->state = FROSTGFX_STATE_SHUTTING_DOWN;
    return FROSTGFX_OK;
}

frostgfx_status frostgfx_tick(frostgfx_engine* e) {
    if (!e) return FROSTGFX_ERROR;
    return FROSTGFX_OK;
}

frostgfx_status frostgfx_load_scene(frostgfx_engine* e,
                                    const char* scene_uri,
                                    const char* game_root,
                                    uint8_t preview_only)
{
    (void)game_root; (void)preview_only;
    if (!e || !scene_uri) return FROSTGFX_ERROR;
    e->state = FROSTGFX_STATE_SCENE_LOADED;
    return FROSTGFX_OK;
}

frostgfx_status frostgfx_reload_scene(frostgfx_engine* e, const char* scene_uri) {
    if (!e || !scene_uri) return FROSTGFX_ERROR;
    return FROSTGFX_OK;
}

frostgfx_status frostgfx_update_camera(frostgfx_engine* e, const frostgfx_camera* cam) {
    if (!e || !cam) return FROSTGFX_ERROR;
    return FROSTGFX_OK;
}

frostgfx_engine_state frostgfx_get_state(const frostgfx_engine* e) {
    return e ? e->state : FROSTGFX_STATE_DESTROYED;
}

void* frostgfx_get_window_handle(const frostgfx_engine* e) {
    return e ? e->window_handle : NULL;
}

size_t frostgfx_last_error(const frostgfx_engine* e, char* buf, size_t cap) {
    if (!e || !buf || cap == 0) return 0;
    size_t n = strlen(e->last_error);
    if (n >= cap) n = cap - 1;
    memcpy(buf, e->last_error, n);
    buf[n] = '\0';
    return n;
}

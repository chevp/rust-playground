/*
 * nuna_core_stub.c - No-op implementation of the launcher C ABI.
 *
 * Lets the Rust side link and exercise the lifecycle end-to-end. A real
 * implementation would live inside nuna-core (C++), backed by frostgfx +
 * nuna-engine + nuna-assets + nuna-synthxml + extension-system.
 */
#include "nuna_core.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Force line buffering so prints interleave naturally with Rust output
 * even when stdout is piped (e.g. cargo's output capture). */
static void log_line(const char* s) {
    fputs(s, stdout);
    fputc('\n', stdout);
    fflush(stdout);
}

struct nuna_runtime {
    char config_path[512];
};

nuna_runtime* nuna_init(const char* config_path) {
    if (!config_path) return NULL;
    nuna_runtime* rt = (nuna_runtime*)calloc(1, sizeof(struct nuna_runtime));
    if (!rt) return NULL;
    strncpy(rt->config_path, config_path, sizeof(rt->config_path) - 1);
    char buf[600];
    snprintf(buf, sizeof(buf), "[nuna-core] init: loaded config '%s'", rt->config_path);
    log_line(buf);
    return rt;
}

int32_t nuna_run(nuna_runtime* rt) {
    if (!rt) return -1;
    log_line("[nuna-core] run: opening window, starting render loop");
    char buf[64];
    for (int i = 1; i <= 5; ++i) {
        snprintf(buf, sizeof(buf), "[nuna-core] tick %d", i);
        log_line(buf);
    }
    log_line("[nuna-core] run: user quit, returning");
    return 0;
}

void nuna_close(nuna_runtime* rt) {
    if (!rt) return;
    log_line("[nuna-core] close: releasing runtime");
    free(rt);
}

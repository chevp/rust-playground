// frostgfx_bridge.hpp - cxx-rs bridge to the real coregfx::api::FrostEngine.
//
// The class definition lives here (not in the .cpp) because cxx-build's
// generated C++ TU needs the complete type to emit member-function trampolines.
#pragma once

#include <memory>
#include <frostgfx/api/FrostEngine.hpp>
#include "rust/cxx.h"

namespace fgx_bridge {

struct EngineConfigDto;   // declared by the Rust cxx::bridge in main.rs

class FrostEngineWrapper {
public:
    FrostEngineWrapper()  = default;
    ~FrostEngineWrapper() = default;

    int32_t state() const;
    bool    initialize(const EngineConfigDto& cfg);

private:
    coregfx::api::FrostEngine engine_;
};

std::unique_ptr<FrostEngineWrapper> new_engine();

rust::String frostgfx_version();

} // namespace fgx_bridge

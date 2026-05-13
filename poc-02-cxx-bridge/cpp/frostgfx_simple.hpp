// frostgfx_simple.hpp - cxx-friendly slice of coregfx::api::FrostEngine.
//
// This is the wrapper that would live inside frostgfx (e.g.
// `src/cxx_bridge/frostgfx_simple.cpp`) and translate cxx-compatible types
// into the variant-based FrostEngine surface. Today it's a stub.

#pragma once

#include <memory>
#include <string>
#include <vector>
#include "rust/cxx.h"

namespace frostgfx_simple {

// Camera POD — passed by value across the bridge.
struct Camera;            // forward-declared (declared in Rust bridge)
struct EngineConfigDto;   // forward-declared (declared in Rust bridge)

// Opaque C++ type Rust holds via UniquePtr.
class FrostEngineSimple {
public:
    FrostEngineSimple();
    ~FrostEngineSimple();

    bool initialize(const EngineConfigDto& cfg);
    bool load_scene(rust::Str scene_uri, bool preview_only);
    bool update_camera(const Camera& cam);
    bool activate();
    bool shutdown();

    int32_t state() const;                    // mirrors EngineState as int
    rust::String last_error() const;
    rust::Vec<rust::String> list_entities() const;

private:
    int32_t state_ = 0;                       // 0 = Created
    std::string last_error_;
    std::vector<std::string> entities_;
};

std::unique_ptr<FrostEngineSimple> new_engine();

} // namespace frostgfx_simple

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Headers live in the submodule.
    let frostgfx_src = manifest_dir.join("frostgfx");
    let include_dir = frostgfx_src.join("include");
    if !include_dir.exists() {
        panic!(
            "frostgfx submodule not initialized. Run:\n  \
             git submodule update --init {}",
            frostgfx_src.display()
        );
    }

    // Locate the built frostgfx (provides frostgfx.lib + frostgfx.dll).
    let build_dir = locate_frostgfx_build(&frostgfx_src, &manifest_dir);
    let lib_dir = build_dir.join("lib").join("Debug");
    let bin_dir = build_dir.join("bin").join("Debug");
    let lib_file = lib_dir.join("frostgfx.lib");
    if !lib_file.exists() {
        panic!(
            "frostgfx.lib not found at {}.\n\
             Build frostgfx first:\n  \
               cd {}\n  \
               cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE=$env:VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake\n  \
               cmake --build build --config Debug",
            lib_file.display(),
            frostgfx_src.display()
        );
    }

    // Compile the C++ shim + emit the cxx-rs bridge.
    cxx_build::bridge("src/main.rs")
        .file("cpp/frostgfx_bridge.cpp")
        .include("cpp")
        .include(&include_dir)
        .std("c++20")
        .define("NOMINMAX", None)
        .define("WIN32_LEAN_AND_MEAN", None)
        // frostgfx headers reference VK_USE_PLATFORM_WIN32_KHR through transitive
        // includes; the public API doesn't actually pull Vulkan in, but the macro
        // matters for any private-impl detail that might appear in inline code.
        .define("VK_USE_PLATFORM_WIN32_KHR", None)
        .compile("frostgfx_bridge");

    // Linkage.
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=frostgfx");

    // Make the runtime DLL discoverable next to the cargo-produced exe.
    // Cargo copies build artifacts to OUT_DIR; the binary ends up two levels
    // up at target/<profile>/. Copy the DLLs there.
    if let Ok(out_dir) = env::var("OUT_DIR") {
        let target_dir = PathBuf::from(&out_dir)
            .ancestors()
            .nth(3)
            .map(Path::to_path_buf)
            .expect("derive cargo target dir");
        for dll in [
            "frostgfx.dll",
            "lua.dll",
            "lz4d.dll",
            "sqlite3.dll",
            "tinyxml2.dll",
        ] {
            let src = bin_dir.join(dll);
            if src.exists() {
                let dst = target_dir.join(dll);
                let _ = std::fs::copy(&src, &dst);
            }
        }
    }

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cpp/frostgfx_bridge.cpp");
    println!("cargo:rerun-if-changed=cpp/frostgfx_bridge.hpp");
    println!("cargo:rerun-if-env-changed=FROSTGFX_BUILD_DIR");
}

fn locate_frostgfx_build(submodule: &Path, manifest_dir: &Path) -> PathBuf {
    if let Ok(p) = env::var("FROSTGFX_BUILD_DIR") {
        return PathBuf::from(p);
    }
    let in_submodule = submodule.join("build");
    if in_submodule.exists() {
        return in_submodule;
    }
    // Fall back to the chevp workspace sibling (../../../frameworks/frostgfx/build).
    let workspace_sibling = manifest_dir
        .ancestors()
        .nth(3)
        .map(|root| root.join("frameworks").join("frostgfx").join("build"))
        .filter(|p| p.exists());
    workspace_sibling.unwrap_or(in_submodule)
}

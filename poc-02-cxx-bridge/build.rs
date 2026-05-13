fn main() {
    cxx_build::bridge("src/main.rs")
        .file("cpp/frostgfx_simple.cpp")
        .include("cpp")
        .std("c++17")
        .compile("frostgfx_simple");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cpp/frostgfx_simple.cpp");
    println!("cargo:rerun-if-changed=cpp/frostgfx_simple.hpp");
}

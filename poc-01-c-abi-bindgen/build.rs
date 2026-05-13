fn main() {
    cc::Build::new()
        .file("c-shim/frostgfx_stub.c")
        .include("c-shim")
        .warnings(true)
        .compile("frostgfx_stub");

    println!("cargo:rerun-if-changed=c-shim/frostgfx_c.h");
    println!("cargo:rerun-if-changed=c-shim/frostgfx_stub.c");
}

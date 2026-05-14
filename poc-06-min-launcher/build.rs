fn main() {
    cc::Build::new()
        .file("c-shim/nuna_core_stub.c")
        .include("c-shim")
        .warnings(true)
        .compile("nuna_core_stub");

    println!("cargo:rerun-if-changed=c-shim/nuna_core.h");
    println!("cargo:rerun-if-changed=c-shim/nuna_core_stub.c");
}

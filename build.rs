use std::env;
use std::path::PathBuf;

fn main() {
    let mut cc = cc::Build::new();

    cc.include("src/c/");
    cc.file("src/c/xxhash.c");
    cc.warnings(false);

    cc.compile("xxhash");

    println!("cargo:rerun-if-changed=src/c/xxhash.h");

    let bindings = bindgen::Builder::default()
        .header("src/c/xxhash.h")
        .use_core()
        .ctypes_prefix("libc")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings.rs");
}

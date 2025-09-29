use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to rerun build.rs if header or source files change
    println!("cargo:rerun-if-changed=include/lib.h");
    println!("cargo:rerun-if-changed=src/lib.c");

    // Compile the C code
    cc::Build::new()
        .file("src/lib.c")
        .include("include")
        .compile("liblib.a");

    // Generate bindings with bindgen
    let bindings = bindgen::Builder::default()
        .header("include/lib.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

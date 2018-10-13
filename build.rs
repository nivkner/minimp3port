extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
fn main() {
    compile_lib();
    create_cbindings();
}

fn compile_lib() {
    cc::Build::new()
        .file("cver/minimp3.h")
        .define("MINIMP3_IMPLEMENTATION", None)
        .define("MINIMP3_NO_SIMD", None)
        .compile("minimp3");
}

fn create_cbindings() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("cver/minimp3.h")
        // Gain access to funtions used to implement the library for incremental porting
        .clang_arg("-DMINIMP3_IMPLEMENTATION")
        .clang_arg("-DMINIMP3_NO_SIMD")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

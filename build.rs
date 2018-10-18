extern crate cc;

fn main() {
    cc::Build::new()
        .file("cver/minimp3.c")
        .define("MINIMP3_IMPLEMENTATION", None)
        .define("MINIMP3_NO_SIMD", None)
        .compile("minimp3");
}

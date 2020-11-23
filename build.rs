use cxx_build::CFG;
use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let mediapipe_include_dir = Path::new(&manifest_dir).join("mediapipe");
    CFG.exported_header_dirs.push(&mediapipe_include_dir);

    cxx_build::bridge("src/lib.rs")
        .file("mediapipe-bind.cc")
        .flag_if_supported("-std=c++14")
        .compile("media-rust-pipe");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=mediapipe-bind.h");
    println!("cargo:rerun-if-changed=mediapipe-bind.cc");
}

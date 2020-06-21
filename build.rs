fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("mediapipe-bind.h")
        .include("mediapipe")
        .flag_if_supported("-std=c++14")
        .compile("media-rust-pipe");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=mediapipe-bind.h");
    println!("cargo:rerun-if-changed=mediapipe-bind.cc");
}

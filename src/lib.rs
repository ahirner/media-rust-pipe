#[cxx::bridge(namespace = mediapipe)]
pub(crate) mod ffi {
    extern "C" {
        include!("../mediapipe-bind.h");

        pub(crate) type ImageFrame;
        pub(crate) fn width(image: &ImageFrame) -> i32;
    }

    extern "Rust" {
        pub fn process(image: &ImageFrame) -> ();
    }
}

fn process(image: &ffi::ImageFrame) -> () {
    let w = ffi::width(image);
    println!("GOT {}", w);
}

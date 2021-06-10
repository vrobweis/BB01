#![feature(with_options)]
#![feature(duration_consts_2)]
#![feature(destructuring_assignment)]

pub mod functions;
pub mod library;

pub use functions::{fullscreen, theme};

#[tokio::test]
async fn base() {
    use crate::library::*;
    let _lib = Library::default();
}

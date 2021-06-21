#![feature(with_options)]
#![feature(destructuring_assignment)]

pub mod functions;
pub mod library;
pub mod retriever;

pub use self::{functions::*, library::*, retriever::*};

#[tokio::test]
async fn base() {
    use crate::library::*;
    let _lib = Library::default();
}

#[tokio::test]
async fn tes() {
    use self::*;
    let _l = Library::default();
    let r = Retriever::default();
    r.get_cnt().await;
}

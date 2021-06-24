#![feature(slice_pattern)]

pub mod functions;
pub mod library;
pub mod retriever;

pub use self::{functions::*, library::*, retriever::*};

#[tokio::test]
async fn base() {
    use self::*;
    let mut l = Library::default();
    l.from_url("https://readmanganato.com/manga-lt989154/chapter-21".to_owned())
        .await;
    // println!("{:?}", p);
}

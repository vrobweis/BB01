#![feature(with_options)]

pub mod funcs;
pub mod library;
pub mod retriever;

pub use self::{funcs::*, library::*, retriever::*};

#[tokio::test]
async fn base() {
    use self::*;
    const TEST: &str = "https://readmanganato.com/manga-lt989154/chapter-21";
    let r = Retriever::default();
    let c = r.book(TEST.to_owned().into()).await;
    c.save();
    println!("{:?}", c.content.len());
}

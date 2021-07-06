#![feature(with_options)]

pub mod funcs;
pub mod library;
pub mod reader;
pub mod retriever;
pub mod ui;

pub use self::{funcs::*, library::*, retriever::*};

pub static APPNAME: &str = "pagepal";
#[tokio::test]
async fn base() {
    use self::*;
    const TEST: &str = "https://readmanganato.com/manga-lt989154/chapter-21";
    let r = Retriever::default();
    let c: Book<Manga> = r.book(TEST.into()).await;
    c.save();
    println!("{:?}", c.content.len());
}

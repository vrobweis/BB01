pub mod functions;
pub mod library;
pub mod retriever;

pub use self::{functions::*, library::*, retriever::*};

#[tokio::test]
async fn base() {
    use self::*;
    const TEST: &str = "https://readmanganato.com/manga-lt989154/chapter-21";
    let mut l = Library::default();
    l.from_url(TEST.to_owned()).await;
    println!("{:?}", l.books);
}

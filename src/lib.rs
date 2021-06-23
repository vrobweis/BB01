pub mod functions;
pub mod library;
pub mod retriever;

pub use self::{functions::*, library::*, retriever::*};

#[tokio::test]
async fn base() {
    use self::*;
    let _l = Library::default();
    let r = Retriever::default();
    let mut p: Page = "http://codenova.ddns.net".into();
    p = r.dl(p).await.unwrap();
    println!("{:?}", p.title());
}

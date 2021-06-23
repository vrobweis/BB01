pub mod functions;
pub mod library;
pub mod retriever;

pub use self::{functions::*, library::*, retriever::*};

#[tokio::test]
async fn base() {
    use self::*;
    let _l = Library::default();
    let _r = Retriever::default();
    // let p: Domain = "http://codenova.ddns.net/index.php".parse().unwrap();
    // println!("{:?}", p);
}

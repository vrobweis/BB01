use crate::{Content, Media, Num, Page};
use serde::{Deserialize as des, Serialize as ser};
use serde_with::serde_as;
use std::collections::BTreeMap;
use tokio::macros::support::thread_rng_n;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug, des, ser)]
pub struct Label(pub String);
#[serde_as]
#[derive(Clone, Default, Debug, ser, des)]
pub struct Book<T: Media> {
    pub title:   Label,
    pub index:   Page,
    // #[serde_as(as = "Vec<(_, _)>")]
    // pub chs:     BTreeMap<u16, Chapter<T>>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub content: BTreeMap<Num, Content<T>>,
    pub pos:     u32,
}

impl<T: Media> Book<T> {
    pub fn save(&self) {
        use std::path::PathBuf;
        static LIBRARY: &str = "library";
        let pb = PathBuf::from(LIBRARY).join(self.title.0.trim());
        self.content.iter().for_each(move |(_, a)| a.save(&pb));
    }
}

impl<T: Media + Eq> Eq for Book<T> {}
impl<T: Media + PartialEq> PartialEq for Book<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.content == other.content
    }
}
impl Default for Label {
    fn default() -> Self { Self(thread_rng_n(1234567890).to_string()) }
}
impl From<String> for Label {
    fn from(s: String) -> Self { Label(s) }
}

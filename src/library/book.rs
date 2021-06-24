use crate::{Chapter, Content, Num, Page};
use serde::{Deserialize as des, Serialize as ser};
use serde_with::serde_as;
use std::collections::BTreeMap;
use tokio::macros::support::thread_rng_n;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug, des, ser)]
pub struct Label(pub String);
#[serde_as]
#[derive(Clone, Default, Debug, ser, des)]
pub struct Book {
    pub title:   Label,
    pub index:   Page,
    pub visual:  bool,
    #[serde_as(as = "Vec<(_, _)>")]
    pub chs:     BTreeMap<u16, Chapter>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub content: BTreeMap<Num, Content>,
    pub pos:     u32,
}

impl Eq for Book {}
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index &&
            self.visual == other.visual &&
            self.content == other.content
    }
}
impl Default for Label {
    fn default() -> Self { Self(thread_rng_n(1234567890).to_string()) }
}

impl From<String> for Label {
    fn from(s: String) -> Self { Label(s) }
}

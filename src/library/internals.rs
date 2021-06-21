use crate::Page;
use serde::{Deserialize as des, Serialize as ser};
use tokio::macros::support::thread_rng_n;

impl Default for Label {
    fn default() -> Self { Self(thread_rng_n(1234567890).to_string()) }
}
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug, des, ser)]
pub struct Label(pub String);

#[derive(Default, Ord, Eq, PartialOrd, Clone, Debug, des, ser)]
pub struct BookSource(pub Label, pub Page);

impl BookSource {
    pub fn name(&self) -> &Label { &self.0 }

    pub fn source(&self) -> &Page { &self.1 }

    pub fn set_name(&mut self, bn: Label) { self.0 = bn; }

    pub fn set_src(&mut self, src: Page) { self.1 = src; }
}
impl PartialEq for BookSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1.loc == other.1.loc
    }
}
impl From<String> for Label {
    fn from(s: String) -> Self { Label(s) }
}

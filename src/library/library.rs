use super::{headers::Headers, Book, BookName, Delay, Source};
use reqwest::Client;
use serde::{Deserialize as des, Serialize as ser};
use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BTreeMap,
        HashMap,
    },
    time::Duration,
};

#[derive(Default, Clone, Debug, ser, des)]
pub struct Library {
    books:   BTreeMap<BookName, Book>,
    #[serde(skip)]
    sites:   HashMap<String, Delay>,
    #[serde(skip)]
    client:  Client,
    headers: BTreeMap<String, Headers>,
}

impl Library {
    pub fn add_book(&mut self, bn: BookName, b: Book) -> Option<Book> {
        self.books.insert(bn, b)
    }

    pub async fn access(&mut self, src: &Source) {
        const DELAY: Duration = Duration::from_secs_f32(1.5);
        match self.sites.entry(src.domain()) {
            Occupied(mut e) => {
                e.get_mut().delay(DELAY).await;
            }
            Vacant(e) => {
                e.insert(Default::default());
            }
        }
    }
}

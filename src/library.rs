use crate::Retriever;
use serde::{Deserialize as des, Serialize as ser};
use serde_with::serde_as;
use std::collections::HashMap;

pub mod book;
pub mod chapter;
pub mod content;
pub mod identifiers;

pub(crate) use self::{book::*, chapter::*, content::*};

#[serde_as]
#[derive(Default, Debug, Clone, ser, des)]
pub struct Library {
    #[serde_as(as = "Vec<(_, _)>")]
    pub books: HashMap<Label, Book>,
    r:         Retriever,
}

impl Library {
    pub async fn from_url(&mut self, url: String) -> Option<Book> {
        let page = url.into();
        let book = self.r.book(page).await;
        self.books.insert(book.title.clone(), book)
    }

    pub fn rename_book(&mut self, idx: &Label, name: String) {
        match self.books.remove(idx) {
            Some(mut b) => {
                b.title = name.clone().into();
                self.books.insert(name.into(), b)
            }
            None => todo!(),
        };
    }
}

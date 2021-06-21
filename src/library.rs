use serde::{Deserialize as des, Serialize as ser};
use serde_with::serde_as;
use std::collections::{BTreeMap, HashMap};

pub mod book;
pub mod chapter;
pub mod content;
pub mod internals;

pub(crate) use self::{book::*, chapter::*, content::*, internals::*};

#[serde_as]
#[derive(Default, Debug, Clone, ser, des)]
pub struct Library {
    #[serde_as(as = "Vec<(_, _)>")]
    books:    HashMap<Label, Book>,
    #[serde_as(as = "Vec<(_, _)>")]
    chapters: BTreeMap<(Label, u16), Chapter>,
}

impl Library {
    pub fn add_book(&mut self, bn: Label, b: Book) -> Option<Book> {
        self.books.insert(bn, b)
    }
}

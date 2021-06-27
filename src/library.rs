use crate::{Page, Retriever};
use serde::{de::DeserializeOwned as deso, Deserialize as des, Serialize as ser};
use serde_with::serde_as;
use std::collections::HashMap;

pub mod book;
pub mod chapter;
pub mod content;
pub mod id;

#[allow(unused)] pub(crate) use self::{book::*, chapter::*, content::*};
pub use content::{Manga, Novel};

#[serde_as]
#[derive(Default, Debug, Clone, ser, des)]
pub struct Library<T = Novel, S = Manga>
where
    T: Media,
    S: Media, {
    #[serde_as(as = "Vec<(_, _)>")]
    pub novels: HashMap<Label, Book<T>>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub manga:  HashMap<Label, Book<S>>,
    r:          Retriever,
}

impl<T: Media + deso + Clone, S: Media + deso + Clone> Library<T, S> {
    pub async fn from_url<Z: Media + Clone>(&mut self, url: String) {
        let page = Page::from(url);
        self.r.refresh(&page).await;
        let b = &page.check_visual().unwrap();
        if *b {
            let book = self.r.book(page).await;
            self.add_manga(book);
        } else {
            let book = self.r.book(page).await;
            self.add_novel(book);
        }
    }

    fn add_manga(&mut self, book: Book<S>) -> Option<Book<S>> {
        self.manga.insert(book.title.clone(), book)
    }

    fn add_novel(&mut self, book: Book<T>) -> Option<Book<T>> {
        self.novels.insert(book.title.clone(), book)
    }

    pub fn rename_novel(&mut self, idx: &Label, name: String) {
        match self.novels.remove(idx) {
            Some(mut b) => {
                b.title = name.clone().into();
                self.novels.insert(name.into(), b)
            }
            None => todo!(),
        };
    }
}

use self::delay::Delay;
use crate::{Book, Content, Media};
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize as des, Serialize as ser};
#[cfg(feature = "trait_ojb_ser")] use serde_traitobject as s;
use serde_with::serde_as;
#[cfg(feature = "trait_ojb_ser")] use std::rc::Rc;
use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BTreeMap,
        HashMap,
    },
    fmt::Debug,
    sync::Arc,
};
use tokio::sync::Mutex;
use url::Host;

pub mod delay;
pub mod finder;
pub mod headers;
pub mod page;

pub use self::{finder::*, headers::*, page::*};
#[derive(Clone, Default, ser, des)]
pub struct FindWrap(
    #[cfg(feature = "trait_ojb_ser")]
    #[serde(with = "serde_traitobject")]
    pub Rc<Option<s::Box<dyn Finder>>>,
);

#[serde_as]
#[derive(Clone, Default, ser, des)]
pub struct Retriever {
    headers: BTreeMap<Host, Headers>,
    #[serde(skip)]
    client:  Client,
    #[serde(skip)]
    sites:   Arc<Mutex<HashMap<Host, Delay>>>,
    #[cfg(feature = "trait_ojb_ser")]
    #[serde_as(as = "Vec<(_, _)>")]
    finders: BTreeMap<Host, FindWrap>,
    //add new fields to the Debug impl
}
/// Struct for download logic
impl Retriever {
    pub async fn refresh(&self, page: &Page) -> Page {
        self.access(page).await;
        if !page.full.get() {
            self.dl(page).await.unwrap()
        } else {
            if page.is_old(None) {}
            page.refresh(Some(&self.client)).await.unwrap()
        }
    }

    /// Generates a Book<T> from a page to either
    /// a chapter or the index of the book
    pub async fn book<T: Debug + Media + Clone>(&self, page: Page) -> Book<T> {
        let index = self.index(&page).await;
        let title = index.title();
        let chapters = self
            .chapters(&index)
            .await
            .into_iter()
            .map(|a| (a.get_content::<T>().unwrap().clone(), a));
        let mut contents = vec![];
        match T::visual() {
            true => {
                for (c, _) in chapters {
                    contents.extend(
                        join_all(c.iter().map(|a| async move {
                            let page = self.refresh(&Page::from(a)).await;
                            (
                                page.clone(),
                                Content::from(page.get_image(&self.client).await),
                            )
                                .into()
                        }))
                        .await,
                    )
                }
            }
            false => contents.extend(chapters.map(|(c, p)| {
                let d: Content<T> = c.join("\n\n").into();
                (p, d).into()
            })),
        }
        //Add content type to book
        let mut bk = Book {
            title,
            index,
            ..Default::default()
        };
        contents.into_iter().for_each(|a: Content<T>| {
            let place = a.src.as_ref().unwrap().get_place();
            bk.content
                .insert(crate::Num(place.1, Some(place.0 as u8)), a);
        });
        bk
    }

    /// Generate a vec with contents for every page
    pub async fn contents<T: Media>(&self, chaps: Vec<Page>) -> Vec<Content<T>> {
        join_all(chaps.iter().map(|page| async move {
            self.refresh(&page).await;
            page.get_image(&self.client).await.into()
        }))
        .await
    }

    /// Gets Pages to all chapters found in an index page
    pub async fn chapters(&self, p: &Page) -> Vec<Page> {
        join_all(
            p.chaps()
                .unwrap()
                .iter()
                .map(|a| async move { self.refresh(&Page::from(a)).await }),
        )
        .await
    }

    /// Tries getting the Index page of the work
    pub async fn index(&self, p: &Page) -> Page {
        self.refresh(
            &self
                .refresh(p)
                .await
                .index()
                .map_err(|_| p.to_owned())
                .unwrap(),
        )
        .await
    }

    /// Initial Page download preparations and actual dl.
    async fn dl(&self, page: &Page) -> Result<Page, Page> {
        let headers = self
            .headers
            .get(&page.domain().unwrap())
            .map(Headers::to_owned)
            .unwrap_or_default()
            .headers;
        page.request(
            self.client
                .get(page.loc.as_str())
                .headers(headers)
                .build()
                .unwrap(),
        );
        page.refresh(Some(&self.client)).await
    }

    /// Keeps track of domains being accessed and adds delay between accessed
    async fn access(&self, p: &Page) {
        match self.sites.lock().await.entry(p.domain().unwrap()) {
            Occupied(mut e) => {
                let mut b = e.get_mut().clone();
                b.delay_if(super::duration()).await;
            }
            Vacant(e) => {
                e.insert(Default::default());
            }
        }
        // TODO: Maybe add a trim function for the map that runs occasionally
    }
}

impl Debug for Retriever {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Retriever")
            .field("headers", &self.headers)
            .field("client", &self.client)
            .field("sites", &self.sites)
            .finish()
    }
}

use self::delay::Delay;
use crate::{Book, Content};
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
impl Retriever {
    pub async fn refresh(&self, page: &Page) -> Page {
        if !page.full.get() {
            self.access(&page).await;
            self.dl(page).await.unwrap()
        } else {
            if page.is_old(None) {
                self.access(page).await;
            }
            page.refresh(Some(&self.client)).await.unwrap()
        }
    }

    pub async fn book(&self, page: Page) -> Book {
        let index = self.index(&page).await;
        let title = index.title();
        dbg!(&title);
        let visual = index.check_visual().unwrap_or_default();
        dbg!(&visual);
        let chapters = self.chapters(&index).await;
        let content = self.contents(chapters, visual).await;
        let mut bk = Book {
            title,
            index,
            visual,
            ..Default::default()
        };
        content.iter().for_each(|a| {
            let place = a.src.as_ref().unwrap().get_place();
            bk.content
                .insert(crate::Num(place.1, Some(place.0 as u8)), a.to_owned());
        });
        bk
    }

    /// generate a vec with contents for every page
    pub async fn contents(
        &self, chapters: Vec<Page>, visual: bool,
    ) -> Vec<Content> {
        // Gets pages to the contents from a page of the chapter
        let pages = join_all(
            chapters
                .iter()
                .map(|a| async move { self.refresh(a).await }),
        )
        .await;
        let content_pages = pages
            .iter()
            .map(|a| a.get_content(visual))
            .map(|a| a.unwrap())
            .flatten();
        if visual {
            join_all(
                content_pages
                    .map(|a| async { self.content(&a.into(), true).await }),
            )
            .await
        } else {
            vec![Content::from(
                content_pages.collect::<Vec<String>>().join(""),
            )]
        }
    }

    pub async fn content(&self, page: &Page, visual: bool) -> Content {
        self.refresh(page.into()).await;
        dbg!(page.loc.path());
        match visual {
            true => page.get_image(Some(&self.client)).await.into(),
            false => page.text().unwrap().join("\n\n").into(),
        }
    }

    pub async fn chapters(&self, p: &Page) -> Vec<Page> {
        join_all(
            p.chaps()
                .unwrap()
                .iter()
                .map(|a| async move { self.refresh(&Page::from(a)).await }),
        )
        .await
    }

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

    async fn dl(&self, p: &Page) -> Result<Page, Page> {
        let headers = self
            .headers
            .get(&p.domain().unwrap())
            .map(Headers::to_owned)
            .unwrap_or_default()
            .headers;
        p.request(
            self.client
                .get(p.loc.as_str())
                .headers(headers)
                .build()
                .unwrap(),
        );
        p.refresh(Some(&self.client)).await
    }

    /// Keeps track of domains being accessed and adds delay between accessed
    async fn access(&self, p: &Page) {
        match self.sites.lock().await.entry(p.domain().unwrap()) {
            Occupied(mut e) => {
                e.get_mut().delay_if(super::duration()).await;
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

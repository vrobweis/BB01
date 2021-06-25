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
    string::ParseError,
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
    pub async fn dl(&self, p: &Page) -> Result<Page, ParseError> {
        if p.is_old(None) {
            self.access(&p).await;
        }
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
        )
        .refresh(Some(&self.client))
        .await
        .unwrap();
        Ok(p.to_owned())
    }

    pub async fn refresh<'a>(&self, page: &'a Page) -> &'a Page {
        self.access(&page).await;
        page.refresh(Some(&self.client)).await.unwrap()
    }

    pub async fn book(&self, page: Page) -> Book {
        let index = self.dl(&page).await.unwrap().index().unwrap();
        self.dl(&index).await.unwrap();
        let title = index.title().to_owned();
        let visual = index.check_visual().unwrap_or_default().to_owned();
        let chapters = self.chapters(&index).await;
        let content = match chapters {
            Some(c) => self.contents(c, visual).await,
            None => vec![],
        };
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
    pub async fn contents(&self, pages: Vec<Page>, visual: bool) -> Vec<Content> {
        let re = join_all(pages.iter().map(|p1| async move {
            self.dl(p1).await.unwrap().get_content(visual)
        }))
        .await;
        let r;
        if visual {
            let temp = join_all(
                re.iter()
                    .map(|a| a.as_ref().unwrap())
                    .flatten()
                    .map(|a| Page::from(a))
                    .map(|a| async move { self.dl(&a).await.unwrap() }),
            )
            .await;
            r = join_all(temp.iter().map(|p2| async move {
                self.content(&self.dl(p2).await.unwrap(), true).await
            }))
            .await;
        } else {
            r = vec![Content::from(
                re.iter()
                    .cloned()
                    .map(|a| a.unwrap())
                    .flatten()
                    .collect::<Vec<String>>()
                    .join(""),
            )];
        }
        r
    }

    pub async fn content(&self, page: &Page, visual: bool) -> Content {
        match visual {
            true => page.get_image(Some(&self.client)).await.into(),
            false => page.text().unwrap().join("").into(),
        }
    }

    pub async fn chapters(&self, p: &Page) -> Option<Vec<Page>> {
        Some(
            join_all(p.chaps().unwrap().iter().cloned().map(|a| async {
                self.dl(&Page::from(a)).await.unwrap().to_owned()
            }))
            .await,
        )
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

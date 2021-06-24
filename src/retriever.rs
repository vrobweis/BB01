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
    pub async fn book(&self, page: Page) -> Book {
        let index = self.dl(&page).await.unwrap().index().unwrap();
        self.dl(&index).await.unwrap();
        let title = index.title().to_owned();
        let visual = index.check_visual().unwrap_or_default().to_owned();
        let chapters = self.chapters(&index).await;
        let content = match chapters {
            Some(c) => self.contents(&c, visual).await,
            None => vec![],
        };

        let mut bk = Book {
            title,
            index,
            visual,
            ..Default::default()
        };
        content.iter().for_each(|a| {
            a.iter().for_each(|b| {
                let place = b.src.as_ref().unwrap().get_place();
                bk.content.insert(
                    crate::Num(place.1, Some(place.0 as u8)),
                    b.to_owned(),
                );
            })
        });
        bk
    }

    /// generate a vec with contents for every page
    pub async fn contents(
        &self, pages: &Vec<Page>, visual: bool,
    ) -> Vec<Vec<Content>> {
        join_all(pages.iter().map(|p1| async move {
            let res = self.dl(p1).await.unwrap().get_content(visual).unwrap();
            match visual {
                true => {
                    join_all(res.iter().map(Page::from).map(|p2| async move {
                        self.dl(&p2).await.unwrap().into()
                    }))
                    .await
                }
                false => vec![Content::from(res.join(""))],
            }
        }))
        .await
    }

    pub async fn content(&self, page: &Page, visual: bool) -> Vec<Content> {
        let res = page
            .refresh(Some(&self.client))
            .await
            .unwrap()
            .get_content(visual);
        match visual {
            true => {
                join_all(res.unwrap().iter().map(Page::from).map(
                    |p| async move {
                        p.refresh(Some(&self.client)).await.unwrap().into()
                    },
                ))
                .await
            }
            false => vec![Content::from(res.unwrap().join(""))],
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

    /// Keeps track of domains being accessed and adds delay between accessed
    async fn access(&self, p: &Page) {
        match self.sites.lock().await.entry(p.domain().unwrap()) {
            Occupied(mut e) => {
                e.get_mut().delay(super::duration()).await;
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

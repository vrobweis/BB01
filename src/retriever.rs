use self::delay::Delay;
use crate::Label;
use reqwest::Client;
use serde::{Deserialize as des, Serialize as ser};
use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BTreeMap,
        HashMap,
    },
    fmt::Debug,
    rc::Rc,
    string::ParseError,
    sync::Arc,
};
use tokio::sync::Mutex;

pub mod delay;
pub mod finder;
pub mod headers;
pub mod page;

pub use self::{finder::*, headers::*, page::*};

#[derive(Clone, Default, ser, des)]
pub struct Retriever {
    headers: BTreeMap<String, Headers>,
    #[serde(skip)]
    client:  Client,
    #[serde(skip)]
    sites:   Arc<Mutex<HashMap<String, Delay>>>,
    #[serde(skip)]
    cntmap:  Arc<HashMap<Label, Page>>,
    #[serde(skip)]
    finders: BTreeMap<Find, Rc<Option<Box<dyn Finder>>>>,
    //add new fields to the Debug impl
}
impl Retriever {
    pub async fn dl(&self, p: Page) -> Result<Page, ParseError> {
        if p.is_old(None) {
            self.access(&p).await;
        }
        let headers = self
            .headers
            .get(&p.domain())
            .map(Headers::to_owned)
            .unwrap_or_default()
            .headers;
        p.request(self.client.get(&p.loc).headers(headers).build().unwrap())
            .refresh(&self.client)
            .await
            .unwrap();
        Ok(p)
    }

    pub async fn get_cnt(&self) { self.cntmap.clone().get(&Label::default()); }

    pub async fn get_page(&self, p: Page) -> Result<Page, Page> { Ok(p) }

    /// Keeps track of domains being accessed and adds delay between accessed
    async fn access(&self, p: &Page) {
        match self.sites.lock().await.entry(p.domain()) {
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
            .field("cntmap", &self.cntmap)
            .finish()
    }
}

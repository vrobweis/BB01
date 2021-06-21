use crate::Label;
use reqwest::Client;
use serde::{Deserialize as des, Serialize as ser};
use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BTreeMap,
        HashMap,
    },
    rc::Rc,
    string::ParseError,
    sync::Arc,
};
use tokio::sync::Mutex;

pub mod delay;
pub mod headers;
pub mod page;

use self::delay::Delay;
pub use self::{headers::*, page::*};

#[derive(Clone, Default, Debug, ser, des)]
pub struct Retriever {
    headers: BTreeMap<String, Headers>,
    #[serde(skip)]
    client:  Rc<Client>,
    #[serde(skip)]
    sites:   Arc<Mutex<HashMap<String, Delay>>>,
    #[serde(skip)]
    cntmap:  Arc<HashMap<Label, Page>>,
}

impl Retriever {
    pub async fn dl(&self, p: Page) -> Result<(), ParseError> {
        if p.is_old(None) {
            self.access(&p).await;
        }
        let headers = self
            .headers
            .get(&p.domain())
            .map(Headers::to_owned)
            .unwrap_or_default()
            .headers;
        p.request(self.client.get(&p.loc).headers(headers).build().unwrap());

        Ok(())
    }

    pub async fn get_cnt(&self) { self.cntmap.clone().get(&Label::default()); }

    async fn access(&self, p: &Page) {
        match self.sites.lock().await.entry(p.domain()) {
            Occupied(mut e) => {
                e.get_mut().delay(super::duration()).await;
            }
            Vacant(e) => {
                e.insert(Default::default());
            }
        }
    }
}

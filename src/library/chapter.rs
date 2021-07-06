use crate::{Content, Media, Page};
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize as des, Serialize as ser};

#[derive(Debug, Clone, Default, ser, des)]
pub struct Chapter<T: Media> {
    pub id:  u16,
    pub src: Option<Page>,
    #[serde(skip)]
    pub cnt: Vec<Content<T>>,
}
impl<T: Media> Chapter<T> {
    pub async fn set_cnt(
        &mut self, content: Option<Vec<Content<T>>>,
    ) -> &Vec<Content<T>> {
        match content {
            Some(c) => self.cnt = c,
            None => self.cnt = self.content(&Client::new()).await,
        }
        &self.cnt
    }

    pub async fn content(&self, client: &Client) -> Vec<Content<T>> {
        let r = self.src.as_ref().unwrap().get_content::<T>().unwrap();
        if T::visual() {
            join_all(r.iter().map(|s| Page::from(s)).map(|a| async move {
                let p = Page::get_image(&a, client).await;
                (a, Content::from(p)).into()
            }))
            .await
        } else {
            vec![
                (self.src.clone().unwrap(), Content::from(r.join("\n\n"))).into(),
            ]
        }
    }
}

use reqwest::header::{HeaderMap, REFERER};
use serde::{Deserialize as des, Serialize as ser};

#[derive(Clone, Debug, ser, des)]
pub struct Headers {
    #[serde(with = "http_serde::header_map")]
    pub headers: HeaderMap,
}
impl Default for Headers {
    fn default() -> Self {
        let mut hm = HeaderMap::new();
        hm.insert(REFERER, "https://manganato.com/".parse().unwrap());
        Self { headers: hm }
    }
}

use serde::{Deserialize as des, Serialize as ser};
use std::path::PathBuf;

#[derive(Default, Clone, Debug, des, ser)]
pub struct Content {
    pub location: PathBuf,
    pub data:     Option<String>,
}

impl Content {
    pub fn data(&self) -> &[u8] { self.data.as_ref().unwrap().as_bytes() }
}

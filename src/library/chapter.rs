#![allow(dead_code)]
use super::Content;
use serde::{Deserialize as des, Serialize as ser};

#[derive(Clone, Default, Debug, ser, des)]
pub struct Chapter<T = Content> {
    pub cnt: T,
}
impl<I> Chapter<I> {
    fn new<T: IntoIterator<IntoIter = I>>(t: T) -> Self {
        Chapter { cnt: t.into_iter() }
    }
}
impl<T: IntoIterator<Item = T, IntoIter = T>> From<T> for Chapter<T> {
    fn from(t: T) -> Chapter<T::IntoIter> { Chapter { cnt: t.into_iter() } }
}

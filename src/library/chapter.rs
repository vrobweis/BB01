#![allow(dead_code)]
use serde::{Deserialize as des, Serialize as ser};

pub trait ChapterTrait {}
#[derive(Clone, Default, Debug, ser, des)]
pub struct Chapter<T>
where
    T: ChapterTrait, {
    pub cnt: T,
}
impl<I: ChapterTrait> Chapter<I> {
    fn new<T: IntoIterator<IntoIter = I>>(t: T) -> Self {
        Chapter { cnt: t.into_iter() }
    }
}
impl<T: IntoIterator<Item = T, IntoIter = T> + ChapterTrait> From<T>
    for Chapter<T>
{
    fn from(t: T) -> Chapter<T::IntoIter> { Chapter { cnt: t.into_iter() } }
}

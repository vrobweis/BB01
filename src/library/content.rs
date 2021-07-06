use crate::{Get, Page};
use serde::{Deserialize as des, Serialize as ser};
use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    fs::File,
    io::Write,
    path::PathBuf,
};

pub type Novel = String;
pub type Manga = Box<Vec<u8>>;

pub trait Media
where
    Self: Default + ser + serde::de::DeserializeOwned, {
    fn save(&self) {}
    fn from(src: Vec<u8>) -> Self;
    fn fetch() {}
    fn visual() -> bool { false }
    fn get(&self) -> &[u8];
}

impl Media for Novel {
    fn from(src: Vec<u8>) -> Self { String::from_utf8(src).unwrap() }

    fn get(&self) -> &[u8] { self.as_bytes() }
}
impl Media for Manga {
    fn from(src: Vec<u8>) -> Self { Box::new(src) }

    fn visual() -> bool { true }

    fn get(&self) -> &[u8] { &self }
}

pub enum MediaData {
    Picture(Page, Vec<String>),
    Text(Page, Vec<u8>),
}

#[derive(Debug, Clone, Default, Eq, PartialEq, ser, des)]
pub struct Num(pub u16, pub Option<u8>);
#[derive(Debug, Clone, Default, Eq, PartialEq, ser, des)]
pub struct Content<T: Media> {
    pub id:  u64,
    pub src: Option<Page>,
    #[serde(skip)]
    data:    T,
}
impl<T: Media> Content<T> {
    pub fn lighten(&self) {
        match &self.src {
            Some(p) => p.empty(),
            None => {}
        }
    }

    pub async fn fetch_image(&mut self) {
        if let (Some(page), data) = (&self.src, &mut self.data) {
            use reqwest::Client;
            *data = T::from(page.get_image(&Client::new()).await);
        }
    }

    pub async fn fetch_novel(&mut self) {
        if let (Some(page), data) = (&self.src, &mut self.data) {
            *data = T::from(
                page.refresh(None)
                    .await
                    .unwrap()
                    .text()
                    .unwrap()
                    .join("\n\n")
                    .bytes()
                    .collect(),
            );
        }
    }

    pub async fn data_load(&mut self) {
        match T::visual() {
            true => self.fetch_image().await,
            false => self.fetch_novel().await,
        };
    }

    pub fn save(&self, pb: &PathBuf) {
        std::fs::create_dir_all(&pb).unwrap();
        let p1 = format!("c{:04}", self.id / 256);
        let p2 = format!("p{:04}", self.id % 256);
        let mut pb = pb.join(p1 + &p2);
        if T::visual() {
            pb.set_extension("jpg");
        }
        File::with_options()
            .write(true)
            .create(true)
            .open(pb)
            .unwrap()
            .write(self.data.get())
            .unwrap();
    }
}

impl Ord for Num {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
            Less => Less,
            Equal => self.1.cmp(&other.1),
            Greater => Greater,
        }
    }
}
impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.0.cmp(&other.0) {
            Less => Less,
            Equal => self.1.cmp(&other.1),
            Greater => Greater,
        })
    }
}
impl From<u8> for Num {
    fn from(num: u8) -> Self { Self(num as u16, None) }
}
impl From<u16> for Num {
    fn from(num: u16) -> Self { Self(num, None) }
}
impl From<u32> for Num {
    fn from(num: u32) -> Self { Self(num as u16, None) }
}
impl From<usize> for Num {
    fn from(num: usize) -> Self { Self(num as u16, None) }
}
impl From<f32> for Num {
    fn from(num: f32) -> Self {
        Self(num.trunc() as u16, match (num.fract() * 200.) as u8 {
            x @ 1..=u8::MAX => Some(x),
            0 => None,
        })
    }
}
impl From<f64> for Num {
    fn from(num: f64) -> Self {
        Self(num.trunc() as u16, match (num.fract() * 200.) as u8 {
            x @ 1..=u8::MAX => Some(x),
            0 => None,
        })
    }
}
impl From<String> for Num {
    fn from(s: String) -> Self {
        match s.parse::<f32>() {
            Ok(num) => num.into(),
            Err(_) => Num(0, Some(200)),
        }
    }
}

impl<T: Media> From<Page> for Content<T> {
    fn from(p: Page) -> Self {
        Content::<T> {
            src: Some(p),
            ..Default::default()
        }
    }
}
impl<T: Media> From<&Page> for Content<T> {
    fn from(p: &Page) -> Self {
        Content::<T> {
            src: Some(p.to_owned()),
            ..Default::default()
        }
    }
}
impl<T: Media> From<String> for Content<T> {
    fn from(data: String) -> Self {
        Content::<T> {
            data: T::from(data.bytes().collect()),
            ..Default::default()
        }
    }
}
impl<T: Media> From<&String> for Content<T> {
    fn from(data: &String) -> Self {
        Content::<T> {
            data: T::from(data.bytes().collect()),
            ..Default::default()
        }
    }
}
impl<T: Media> From<Vec<u8>> for Content<T> {
    fn from(data: Vec<u8>) -> Self {
        Content::<T> {
            data: T::from(data),
            ..Default::default()
        }
    }
}
impl<T: Media> From<(Page, Content<T>)> for Content<T> {
    fn from(tup: (Page, Content<T>)) -> Self {
        let mut a = tup.1;
        let id = tup.0.get_place();
        a.id = id.0 as u64 + id.1 as u64 * 256;
        a.src = Some(tup.0);
        a
    }
}

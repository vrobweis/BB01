use crate::{Get, Page};
use serde::{Deserialize as des, Serialize as ser};
use std::cmp::Ordering::{self, Equal, Greater, Less};

#[derive(Debug, Clone, Eq, PartialEq, ser, des)]
pub struct Num(pub u16, pub Option<u8>);
#[derive(Debug, Clone, Default, Eq, PartialEq, ser, des)]
pub struct Content {
    pub id:  u64,
    pub src: Option<Page>,
    data:    Option<Vec<u8>>, // UNSAFE!
}
impl Content {
    pub fn lighten(&self) {
        match &self.src {
            Some(p) => p.empty(),
            None => todo!(),
        }
    }

    pub async fn fetch_image(&mut self) {
        if let (Some(page), Some(data)) = (&self.src, &mut self.data) {
            *data = page.get_image(None).await;
        }
    }

    pub async fn fetch_novel(&mut self) {
        if let (Some(page), Some(data)) = (&self.src, &mut self.data) {
            *data = page
                .refresh(None)
                .await
                .unwrap()
                .text()
                .unwrap()
                .join("\n\n")
                .bytes()
                .collect();
        }
    }

    pub async fn data_load(&mut self, visual: bool) {
        match self.data {
            None => match visual {
                true => self.fetch_image().await,
                false => self.fetch_novel().await,
            },
            _ => {}
        };
    }

    pub fn data(&self) -> &Vec<u8> { self.data.as_ref().unwrap() }
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

impl From<Page> for Content {
    fn from(p: Page) -> Self {
        Content {
            src: Some(p),
            ..Default::default()
        }
    }
}
impl From<&Page> for Content {
    fn from(p: &Page) -> Self {
        Content {
            src: Some(p.to_owned()),
            ..Default::default()
        }
    }
}
impl From<String> for Content {
    fn from(data: String) -> Self {
        Content {
            data: Some(data.bytes().collect()),
            ..Default::default()
        }
    }
}
impl From<&String> for Content {
    fn from(data: &String) -> Self {
        Content {
            data: Some(data.bytes().collect()),
            ..Default::default()
        }
    }
}
impl From<Vec<u8>> for Content {
    fn from(data: Vec<u8>) -> Self {
        Content {
            data: Some(data),
            ..Default::default()
        }
    }
}

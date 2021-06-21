use crate::Page;
use serde::{Deserialize as des, Serialize as ser};
use std::cmp::Ordering::{self, Equal, Greater, Less};

#[derive(Debug, PartialEq, Eq, Clone, ser, des)]
pub struct Num(pub u16, pub Option<u8>);
#[derive(Clone, Default, Debug, ser, des)]
pub struct Content {
    pub id: u64,
    src:    Option<Page>,
    data:   Option<String>,
}
impl Content {
    pub fn data(&self) -> &[u8] {
        self.data.as_ref().map(String::as_bytes).unwrap_or(&[])
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

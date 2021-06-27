use crate::{Finder, Get};
use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, Request, Url};
use select::{
    document::Document,
    predicate::{Child, Name, Text},
};
use serde::{Deserialize as des, Serialize as ser};
use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    hash::{Hash, Hasher},
    rc::Rc,
    str::FromStr,
};
use url::Host;

impl Default for Page {
    fn default() -> Self {
        Self {
            loc:  "http://codenova.ddns.net/".parse().unwrap(),
            last: Utc::now(),
            html: Default::default(),
            doc:  Default::default(),
            req:  Default::default(),
            full: Default::default(),
        }
    }
}

#[derive(Clone, Debug, ser, des)]
pub struct Page {
    pub loc:  Url,
    last:     DateTime<Utc>,
    #[serde(skip)]
    html:     RefCell<Option<String>>,
    #[serde(skip)]
    pub doc:  RefCell<Option<Document>>,
    #[serde(skip)]
    req:      Rc<RefCell<Option<Request>>>,
    pub full: Cell<bool>,
}

impl Page {
    /// Loads the html and parsed html in Page preparation for future actions
    pub async fn refresh(&self, client: Option<&Client>) -> Result<Self, Self> {
        if self.doc.borrow().is_none() {
            self.full.set(false);
        };
        if let Some(rq) = self.req.borrow().as_ref() {
            // TODO: Better error recovery with a failures counter in Retriever
            let resp = client
                .unwrap_or(&Client::new())
                .execute(rq.try_clone().unwrap())
                .await
                .unwrap();
            // dbg!(&self);
            let html = resp.text().await.unwrap();
            self.doc.replace(Some(html.as_str().into()));
            self.html.replace(Some(html));
            self.full.set(true);
            Ok(self.to_owned())
        } else {
            Err(self.to_owned())
        }
    }

    pub fn request(&self, re: Request) -> &Self {
        self.req.replace(Some(re));
        self.full.set(false);
        self
    }

    /// Get the `example.com` from `http://example.com/path/`
    /// would fail for http://localhost/path
    pub fn domain(&self) -> Result<Host, String> {
        match self.loc.host() {
            Some(d) => Ok(d.to_owned()),
            // for treating IPs differently
            // Some(d @ Host::Ipv4(_)) => Ok(d.to_owned()),
            // Some(d @ Host::Ipv6(_)) => Ok(d.to_owned()),
            _ => Err("No host.".to_owned()),
        }
    }

    pub async fn next(&self, client: &Client, pred: &str) -> Option<Page> {
        let s = self.doc.borrow().as_ref().and_then(|a| {
            a.select(Child(Name("a"), Text))
                .filter(|a| a.text().contains(pred))
                .map(|a| {
                    Page::from(
                        a.parent().unwrap().attr("href").unwrap().to_string(),
                    )
                })
                .next()
        });
        match s {
            Some(s) => Some(s.refresh(Some(client)).await.unwrap().to_owned()),
            None => None,
        }
    }

    /// Returns a Page leading the the index page of the chapter
    pub fn index(&self) -> Result<Self, url::ParseError> {
        // TODO: Alternatively, find links up or left from other links leading to
        // the current page
        let base = self.loc.origin().ascii_serialization();
        let mut index = self
            .loc
            .path_segments()
            .unwrap()
            .rev()
            .fold((Vec::new(), 0, 0), |mut acc, s| {
                if s.to_lowercase().contains("chapter") {
                    acc.1 += 1;
                } else {
                    if acc.1 != 0 || acc.2 > 1 {
                        acc.0.push(s);
                    }
                }
                acc.2 += 1;
                acc
            })
            .0;
        index.push(&base);
        index
            .iter()
            .rev()
            .map(|&a| a)
            .collect::<Vec<_>>()
            .join("/")
            .parse()
    }

    pub fn get_place(&self) -> (u16, u16, String) {
        let segments = self
            .loc
            .path_segments()
            .unwrap()
            .rev()
            .filter(|&a| a != "")
            .collect::<Vec<_>>();
        let numbers = segments
            .iter()
            .map(|a| {
                a.matches(char::is_numeric)
                    .collect::<Vec<&str>>()
                    .join("")
                    .parse::<u16>()
                    .unwrap_or_default()
            })
            .collect::<Vec<u16>>();
        // TODO: do a better job at finding the index
        let index_candidate = if segments.len() < 3 {
            segments.iter().last()
        } else {
            segments.iter().rev().skip(1).next()
        };
        match (numbers.as_slice(), index_candidate) {
            ([x @ 0..=9000, y @ 0..=9000, ..], Some(&z)) => {
                (*x, *y, z.to_string())
            }
            ([x @ 0..=9000], Some(z)) => (0, *x, z.to_string()),
            ([], Some(z)) => (0, 0, z.to_string()),
            _ => (0, 0, "".to_string()),
        }
    }

    pub fn get_content<T: crate::Media>(&self) -> Option<Vec<String>> {
        match T::visual() {
            true => self.images(),
            false => self.text(),
        }
    }

    pub async fn get_image(&self, client: &Client) -> Vec<u8> {
        match self.req.borrow().as_ref() {
            Some(req) => client
                .execute(req.try_clone().unwrap())
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap()
                .to_vec(),
            None => Vec::new(),
        }
    }

    pub fn check_visual(&self) -> Option<bool> {
        let t = vec!["novel", "royalroad", "manganov", "comrademao"];
        let p = vec!["manga", "scans", "hentai", "pururin", "luscious"];
        let f = |s: &&str| -> bool {
            self.loc.origin().ascii_serialization().contains(s)
        };
        Some(match (t.iter().any(|s| f(s)), p.iter().any(|s| f(s))) {
            (true, true) => self.text().unwrap().len() < 20,
            (true, false) => false,
            (false, true) => true,
            (false, false) => self.text().unwrap().len() < 20,
        })
    }

    pub fn is_old(&self, d: Option<Duration>) -> bool {
        (self.last + d.unwrap_or(Duration::seconds(10))) < Utc::now() ||
            !self.full.get()
    }

    pub fn empty(&self) {
        self.html.replace(Default::default());
        self.doc.replace(Default::default());
        self.full.set(false);
    }
}

impl Finder for Page {}
impl Get for Page {
    #[inline]
    fn doc(&self) -> crate::Doc { self.doc.borrow() }
}

impl Eq for Page {}
impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.full == other.full &&
            self.loc == other.loc &&
            self.html == other.html &&
            self.last == other.last
    }
}
impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.loc, &self.full, &self.last).cmp(&(
            &other.loc,
            &other.full,
            &other.last,
        ))
    }
}
impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((&self.loc, &self.full, &self.last).cmp(&(
            &other.loc,
            &other.full,
            &other.last,
        )))
    }
}
impl Hash for Page {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.loc.hash(state);
        self.last.hash(state);
        (*self.html.borrow()).hash(state);
        self.full.get().hash(state);
    }
}
impl<T: Into<String>> From<T> for Page {
    fn from(src: T) -> Self { src.into().parse::<Self>().unwrap().to_owned() }
}
impl FromStr for Page {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            loc: s.parse::<Url>()?,
            ..Default::default()
        })
    }
}

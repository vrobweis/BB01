use crate::{Finder, Get};
use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, Request, Url};
use select::{
    document::Document,
    predicate::{Child, Name, Text},
};
use serde::{Deserialize as des, Serialize as ser};
use std::{
    cell::{Cell, Ref, RefCell},
    cmp::Ordering,
    hash::{Hash, Hasher},
    rc::Rc,
    str::FromStr,
};

impl Default for Page {
    fn default() -> Self {
        Self {
            loc:  "".to_owned(),
            last: Utc::now(),
            html: RefCell::new(None),
            doc:  RefCell::new(None),
            req:  Rc::new(RefCell::new(None)),
            full: Cell::new(false),
        }
    }
}
#[derive(Clone, Debug, ser, des)]
pub struct Page {
    pub loc:  String,
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
    pub async fn refresh(&self, client: &Client) -> Result<&Self, &Self> {
        if self.doc.borrow().is_none() {
            self.full.set(false);
        };
        if let Some(rq) = self.req.borrow().as_ref() {
            // TODO: Better error recovery with a failures counter in Retriever
            let resp = client.execute(rq.try_clone().unwrap()).await.unwrap();
            let html = resp.text().await.unwrap().to_owned();
            self.doc.replace(Some(html.as_str().into()));
            self.html.replace(Some(html));
            self.full.set(true);
            Ok(self)
        } else {
            Err(self)
        }
    }

    pub fn request(&self, re: Request) -> &Self {
        self.req.replace(Some(re));
        self.full.set(false);
        self
    }

    pub fn domain(&self) -> String {
        (&self.loc)
            .parse::<Url>()
            .unwrap()
            .domain()
            .unwrap()
            .to_owned()
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
            Some(s) => Some(s.refresh(client).await.unwrap().to_owned()),
            None => None,
        }
    }

    /// Returns a Page leading the the index page of the chapter
    pub fn index(&self) -> Result<Self, url::ParseError> {
        let url = self.loc.parse::<Url>().unwrap();
        let base = url.origin().ascii_serialization();
        let mut index = url
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
            .parse::<Page>()
    }

    pub fn get_place(&self) -> (u16, u16, String) {
        let url = self.loc.parse::<Url>().expect("Not a Url string.");
        let segments = url
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

    pub fn is_old(&self, d: Option<Duration>) -> bool {
        (self.last + d.unwrap_or(Duration::seconds(10))) < Utc::now() ||
            self.full.get()
    }
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
    fn from(src: T) -> Self {
        Self {
            loc: src.into(),
            ..Default::default()
        }
    }
}
impl FromStr for Page {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Url>().map(|a| Page::from(a.to_string()))
    }
}
impl Finder for Page {}
impl Get for Page {
    #[inline]
    fn doc(&self) -> Ref<'_, Option<Document>> { self.doc.borrow() }
}

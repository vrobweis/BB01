use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, Request,  Url};
use select::document::Document;
use serde::{Deserialize as des, Serialize as ser};
use std::{
    cell::RefCell,
    cmp::Ordering,
    hash::{Hash, Hasher},
    rc::Rc,
    str::FromStr,
};

impl Default for Page {
    fn default() -> Self {
        Self {
            last: Utc::now(),
            ..Default::default()
        }
    }
}
#[derive(Clone, Debug, ser, des)]
pub struct Page {
    pub loc: String,
    last:    DateTime<Utc>,
    #[serde(skip)]
    html:    RefCell<Option<String>>,
    #[serde(skip)]
    doc:     RefCell<Option<Document>>,
    #[serde(skip)]
    req:     Option<Rc<Request>>,
    #[serde(skip)]
    client:  Rc<Client>,
    full:    bool,
}

impl Page {
    fn new(s: &String) -> Self {
        const DEFAULT: &str = "https://codenova.ddns.net/";
        let url = match s.parse::<Url>() {
            Ok(u) => u.to_string(),
            Err(_) => DEFAULT.to_string(),
        };
        url.into()
    }

    pub async fn refresh(&mut self) -> &Self {
        match &self.req {
            Some(r) => self.set(
                self.client
                    .execute(r.try_clone().unwrap())
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .to_string(),
            ),
            None => todo!(),
        }
        self.full = true;
        self
    }

    pub async fn request(&mut self, re: Request) -> &Self {
        self.req = Some(Rc::new(re));
        self.full = false;
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

    pub fn set(&self, html: String) {
        self.doc.replace(Some(html.as_str().into()));
        self.html.replace(Some(html));
    }

    pub fn is_old(&self, d: Option<Duration>) -> bool {
        (self.last + d.unwrap_or(Duration::seconds(10))) < Utc::now() ||
            self.full
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
        self.full.hash(state);
    }
}
impl From<String> for Page {
    fn from(src: String) -> Self { Self::new(&src) }
}
impl From<&String> for Page {
    fn from(src: &String) -> Self { Self::new(src) }
}
impl From<Url> for Page {
    fn from(src: Url) -> Self { src.to_string().into() }
}
impl From<&Url> for Page {
    fn from(src: &Url) -> Self { src.to_string().into() }
}
impl FromStr for Page {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Url>().map(Into::into)
    }
}

// /// Returns a Source leading the the index page of the chapter
// pub async fn index(&self) -> Self {
//     let url = self.loc.parse::<Url>().unwrap();
//     let base = url.origin().ascii_serialization();
//     let mut index = url
//         .path_segments()
//         .unwrap()
//         .rev()
//         .fold((Vec::new(), 0, 0), |mut acc, s| {
//             if s.to_lowercase().contains("chapter") {
//                 acc.1 += 1;
//             } else {
//                 if acc.1 != 0 || acc.2 > 1 {
//                     acc.0.push(s);
//                 }
//             }
//             acc.2 += 1;
//             acc
//         })
//         .0;
//     index.push(&base);
//     index
//         .iter()
//         .rev()
//         .map(|&a| a)
//         .collect::<Vec<_>>()
//         .join("/")
//         .into()
// }

// /// Returns the biggest congregation of links in the html
// pub async fn chapters(&self) -> Option<Vec<String>> {
//     self.doc.as_ref().map(|a| {
//         a.select(Descendant(
//             Name("div"),
//             Or(Name("p"), Or(Name("table"), Name("ul"))),
//         ))
//         .map(|a| a.select(Name("a")).into_selection())
//         .max_by(|a, b| a.len().cmp(&b.len()))
//         .unwrap()
//         .iter()
//         .filter_map(|a| a.attr("href"))
//         .map(|a| a.to_string())
//         .collect()
//     })
//     /* TODO: Add a similarity check and only return the biggest cluster of
// similar     links */
// }

// pub async fn next(&self, pred: &str) -> Option<Source> {
//     let s = self.doc.as_ref().and_then(|a| {
//         a.select(Child(Name("a"), Text))
//             .filter(|a| a.text().contains(pred))
//             .map(|a| {
//                 Source::from(
//                     a.parent().unwrap().attr("href").unwrap().to_string(),
//                 )
//             })
//             .next()
//     });
//     match s {
//         Some(mut s) => {
//             s.refresh(None).await;
//             Some(s)
//         }
//         None => None,
//     }
// }

// /// Returns something that looks like a book title
// pub fn title(&self) -> Label {
//     println!("{}", self.doc.is_some());
//     let title = self
//         .doc
//         .as_ref()
//         .expect("HTML not found.")
//         .select(Name("title"))
//         .into_selection()
//         .first()
//         .unwrap()
//         .text();

//     if title.contains(" Chapter") {
//         title
//             .split(" Chapter")
//             .filter(|&a| a != "")
//             .collect::<Vec<_>>()
//             .first()
//             .unwrap()
//             .to_string()
//     } else {
//         title
//     }
//     .into()
//     // .to_ascii_lowercase()
//     // .split(" chapter")
//     // .filter(|&a| a != "")
//     // .collect::<Vec<_>>()
//     // .first()
//     // .unwrap()
//     // .chars()
//     // .fold(String::new(), |mut acc, s| {
//     //     if acc.is_empty() || "- ".contains(acc.chars().last().unwrap()) {
//     //         acc.extend(s.to_uppercase());
//     //     } else {
//     //         acc.push(s);
//     //     }
//     //     acc
//     // })
//     // .into()
// }

// /// Returns the text from the children of the <div> with most <p> tags
// pub fn text(&self) -> Option<Vec<String>> {
//     self.doc.as_ref().map(|a| {
//         // TODO: Improve by par_map()?
//         a.select(Child(Name("div"), Name("p")))
//             .map(|a| a.parent().unwrap().children().into_selection())
//             .max_by(|a, b| a.len().cmp(&b.len()))
//             .unwrap()
//             .select(Text)
//             .iter()
//             .map(|a| a.text())
//             .collect()
//     })
// }

// /// similar to index() return the source addr of the div with most <img>
// pub fn images_batch(&self) -> Option<Vec<String>> {
//     self.doc.as_ref().map(|a| {
//         a.select(Child(Name("div"), Name("img")))
//             .map(|a| a.parent().unwrap().select(Name("img")).into_selection())
//             .max_by(|a, b| a.len().cmp(&b.len()))
//             .unwrap()
//             .iter()
//             .map(|a| a.attr("src").unwrap().to_string())
//             .collect()
//     })
//     /* TODO: Similar to index() add a check for links similarity */
// }

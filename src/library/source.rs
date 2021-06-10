use super::BookName;
use reqwest::{Client, Url};
use select::{
    document::Document,
    predicate::{Child, Descendant, Name, Or, Text},
};
use serde::{Deserialize as des, Serialize as ser};
use std::time::Duration;
use tokio::time::{sleep_until, Instant};

impl Default for Delay {
    fn default() -> Self { Self(Instant::now()) }
}
#[derive(Clone, Debug)]
pub struct Delay(pub Instant);
impl Delay {
    pub async fn delay(&mut self, delay: Duration) {
        let until = self.0 + delay;
        if until > Instant::now() {
            sleep_until(until).await;
        }
        self.0 = Instant::now();
    }
}

#[derive(Default, Clone, Debug, des, ser)]
pub struct Source {
    loc:  String,
    #[serde(skip)]
    html: Option<String>,
    #[serde(skip)]
    doc:  Option<Document>,
    full: bool,
}

impl Source {
    pub async fn new(src: String, client: Option<&Client>) -> Self {
        let loc = src;
        let (html, doc) = dl(&loc, client).await;
        Self {
            loc,
            html,
            doc,
            full: true,
        }
    }

    pub async fn refresh(&mut self, client: Option<&Client>) -> &mut Self {
        (self.html, self.doc) = dl(&self.loc, client).await;
        self.full = true;
        self
    }

    /// Returns a Source leading the the index page of the chapter
    pub async fn index(&self) -> Self {
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
            .into()
    }

    /// Returns the biggest congregation of links in the html
    pub async fn chapters(&self) -> Option<Vec<String>> {
        self.doc.as_ref().map(|a| {
            a.select(Descendant(
                Name("div"),
                Or(Name("p"), Or(Name("table"), Name("ul"))),
            ))
            .map(|a| a.select(Name("a")).into_selection())
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .iter()
            .filter_map(|a| a.attr("href"))
            .map(|a| a.to_string())
            .collect()
        })
        /* TODO: Add a similarity check and only return the biggest cluster of similar
        links */
    }

    pub async fn next(&self, pred: &str) -> Option<Source> {
        let s = self.doc.as_ref().and_then(|a| {
            a.select(Child(Name("a"), Text))
                .filter(|a| a.text().contains(pred))
                .map(|a| {
                    Source::from(
                        a.parent().unwrap().attr("href").unwrap().to_string(),
                    )
                })
                .next()
        });
        match s {
            Some(mut s) => {
                s.refresh(None).await;
                Some(s)
            }
            None => None,
        }
    }

    /// Returns something that looks like a book title
    pub fn title(&self) -> BookName {
        println!("{}", self.doc.is_some());
        let title = self
            .doc
            .as_ref()
            .expect("HTML not found.")
            .select(Name("title"))
            .into_selection()
            .first()
            .unwrap()
            .text();

        if title.contains(" Chapter") {
            title
                .split(" Chapter")
                .filter(|&a| a != "")
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .to_string()
        } else {
            title
        }
        .into()
        // .to_ascii_lowercase()
        // .split(" chapter")
        // .filter(|&a| a != "")
        // .collect::<Vec<_>>()
        // .first()
        // .unwrap()
        // .chars()
        // .fold(String::new(), |mut acc, s| {
        //     if acc.is_empty() || "- ".contains(acc.chars().last().unwrap()) {
        //         acc.extend(s.to_uppercase());
        //     } else {
        //         acc.push(s);
        //     }
        //     acc
        // })
        // .into()
    }

    /// Returns the text from the children of the <div> with most <p> tags
    pub fn text(&self) -> Option<Vec<String>> {
        self.doc.as_ref().map(|a| {
            // TODO: Improve by par_map()?
            a.select(Child(Name("div"), Name("p")))
                .map(|a| a.parent().unwrap().children().into_selection())
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap()
                .select(Text)
                .iter()
                .map(|a| a.text())
                .collect()
        })
    }

    /// similar to index() return the source addr of the div with most <img>
    pub fn images_batch(&self) -> Option<Vec<String>> {
        self.doc.as_ref().map(|a| {
            a.select(Child(Name("div"), Name("img")))
                .map(|a| a.parent().unwrap().select(Name("img")).into_selection())
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap()
                .iter()
                .map(|a| a.attr("src").unwrap().to_string())
                .collect()
        })
        /* TODO: Similar to index() add a check for links similarity */
    }

    pub fn domain(&self) -> String {
        self.loc
            .parse::<Url>()
            .unwrap()
            .domain()
            .unwrap()
            .to_string()
    }

    pub fn pos(&self) -> (u16, u16, String) { get_place(&self.loc) }
}
impl Eq for Source {}
impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && self.html == other.html
    }
}
impl From<String> for Source {
    fn from(src: String) -> Self {
        Self {
            loc:  src,
            html: None,
            doc:  None,
            full: false,
        }
    }
}
impl From<&String> for Source {
    fn from(src: &String) -> Self {
        Self {
            loc:  src.clone(),
            html: None,
            doc:  None,
            full: false,
        }
    }
}

async fn dl(
    src: &String, client: Option<&Client>,
) -> (Option<String>, Option<Document>) {
    let html = client
        .unwrap_or(&Client::new())
        .get(src)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .ok();
    let doc = Some(html.as_ref().unwrap().as_str().into());
    (html, doc)
}

fn get_place(url: &String) -> (u16, u16, String) {
    let url = url.parse::<Url>().expect("Not a Url string.");
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
        ([x @ 0..=9000, y @ 0..=9000, ..], Some(&z)) => (*x, *y, z.to_string()),
        ([x @ 0..=9000], Some(z)) => (0, *x, z.to_string()),
        ([], Some(z)) => (0, 0, z.to_string()),
        _ => (0, 0, "".to_string()),
    }
}

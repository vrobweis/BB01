use crate::Label;
use select::{
    document::Document,
    predicate::{Child, Descendant, Name, Or, Text},
};
use serde::{Deserialize as des, Serialize as ser};
use std::cell::Ref;

#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Debug, ser, des)]
pub struct Find(u8, String);

type Doc<'a> = Ref<'a, Option<Document>>;
pub trait Finder {
    /// Returns the text from the children of the <div> with most <p> tags
    #[inline]
    fn text_def(&self) -> Box<dyn Fn(Doc) -> Option<Vec<String>>> {
        Box::new(|doc: Doc| {
            doc.as_ref().map(|a| {
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
        })
    }
    /// similar to index() return the srcs from the div with most <img>
    #[inline]
    fn images_def(&self) -> Box<dyn Fn(Doc) -> Option<Vec<String>>> {
        Box::new(|doc: Doc| {
            doc.as_ref().map(|a| {
                a.select(Child(Name("div"), Name("img")))
                    .map(|a| {
                        a.parent().unwrap().select(Name("img")).into_selection()
                    })
                    .max_by(|a, b| a.len().cmp(&b.len()))
                    .unwrap()
                    .iter()
                    .map(|a| a.attr("src").unwrap().to_string())
                    .collect()
                /* TODO: Similar to index() add a check for links similarity */
            })
        })
    }
    /// Returns the biggest congregation of links in the html
    #[inline]
    fn index_def(&self) -> Box<dyn Fn(Doc) -> Option<Vec<String>>> {
        Box::new(|doc: Doc| {
            doc.as_ref().map(|a: &Document| {
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
            /* TODO: Add a similarity check and only return the biggest cluster of
            similar links */
        })
    }
    /// Returns something that looks like a book title
    #[inline]
    fn title_def(&self) -> Box<dyn Fn(Doc) -> Label> {
        Box::new(|doc: Doc| {
            let title = doc
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
            //     if acc.is_empty() || "- ".contains(acc.chars().last().unwrap())
            // {         acc.extend(s.to_uppercase());
            //     } else {
            //         acc.push(s);
            //     }
            //     acc
            // })
            // .into()})
        })
    }
}
pub trait Get: Finder {
    fn doc(&self) -> Ref<'_, Option<Document>>;
    #[inline]
    fn text(&self) -> Option<Vec<String>> { self.text_def()(self.doc()) }
    #[inline]
    fn index(&self) -> Option<Vec<String>> { self.index_def()(self.doc()) }
    #[inline]
    fn image(&self) -> Option<Vec<String>> { self.images_def()(self.doc()) }
    #[inline]
    fn title(&self) -> Label { self.title_def()(self.doc()) }
}

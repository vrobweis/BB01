use super::{Content, Source};
use serde::{Deserialize as des, Serialize as ser};
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Default, Ord, Eq, PartialEq, PartialOrd, Clone, Debug, des, ser)]
pub struct BookName(String);
#[derive(Default, Clone, Debug, des, ser)]
pub struct Book {
    name:     BookName,
    index:    Source,
    visual:   bool,
    chapters: Vec<u16>,
    contents: Vec<Content>,
    pos:      u32,
}
impl Book {
    pub fn add_chapter(&mut self, chn: usize, cont: Vec<Content>) {
        let n = self.chapters[..chn].iter().fold(0, |a, &n| a + n as usize);
        self.chapters.push(cont.len() as u16);
        self.contents.splice(n..n, cont);
    }

    pub fn remove_chapter(&mut self, chn: usize) {
        let b = self.chapters[..chn].iter().fold(0, |a, &n| a + n as usize);
        let end = b + self.chapters[chn] as usize;
        self.chapters.remove(chn);
        self.contents.drain(b..end);
    }

    pub fn save(&self) {
        self.chapters
            .iter()
            .enumerate()
            .fold(0usize, |cur, (n, &next)| {
                let next = cur + next as usize;
                self.contents[cur..next]
                    .iter()
                    .enumerate()
                    .for_each(|(z, c)| {
                        let mut pb = PathBuf::from(&self.name.0)
                            .join(n.to_string())
                            .join(&c.location)
                            .join(format!("{:04}", z));
                        if self.visual {
                            &pb.set_extension("jpg");
                        }
                        std::fs::create_dir_all(&pb).unwrap();
                        File::with_options()
                            .write(true)
                            .create(true)
                            .open(pb)
                            .unwrap()
                            .write(c.data())
                            .unwrap();
                    });
                next
            });
    }
}

impl Eq for Book {}
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.visual == other.visual &&
            self.index == other.index
    }
}
impl From<String> for BookName {
    fn from(s: String) -> Self { BookName(s) }
}

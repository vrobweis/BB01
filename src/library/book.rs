use crate::{BookSource, Chapter, Content, Num};
use serde::{Deserialize as des, Serialize as ser};
use serde_with::serde_as;
use std::collections::BTreeMap;

#[serde_as]
#[derive(Clone, Default, Debug, ser, des)]
pub struct Book {
    bs:      BookSource,
    visual:  bool,
    #[serde_as(as = "Vec<(_, _)>")]
    chs:     BTreeMap<u16, Chapter>,
    #[serde_as(as = "Vec<(_, _)>")]
    content: BTreeMap<Num, Content>,
    pos:     u32,
}

impl Eq for Book {}
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.bs == other.bs &&
            self.visual == other.visual &&
            self.content == other.content
    }
}
// impl Book {
//     pub fn add_chapter(&mut self, chn: usize, cont: Vec<Content>) {
//         let n = self.chapters[..chn].iter().fold(0, |a, &n| a + n as usize);
//         self.chapters.push(cont.len() as u16);
//         self.contents.splice(n..n, cont);
//     }
//     pub fn remove_chapter(&mut self, chn: usize) {
//         let b = self.chapters[..chn].iter().fold(0, |a, &n| a + n as usize);
//         let end = b + self.chapters[chn] as usize;
//         self.chapters.remove(chn);
//         self.contents.drain(b..end);
//     }
//     pub fn save(&self) {
//         self.chapters
//             .iter()
//             .enumerate()
//             .fold(0usize, |cur, (n, &next)| {
//                 let next = cur + next as usize;
//                 self.contents[cur..next]
//                     .iter()
//                     .enumerate()
//                     .for_each(|(z, c)| {
//                         let mut pb = PathBuf::from(&self.name().0)
//                             .join(n.to_string())
//                             .join(&c.loc)
//                             .join(format!("{:04}", z));
//                         if self.visual {
//                             pb.set_extension("jpg");
//                         }
//                         std::fs::create_dir_all(&pb).unwrap();
//                         File::with_options()
//                             .write(true)
//                             .create(true)
//                             .open(pb)
//                             .unwrap()
//                             .write(c.data())
//                             .unwrap();
//                     });
//                 next
//             });
//     }
//     pub fn name(&self) -> &Label { &self.bs.name() }
//     pub fn index(&self) -> &Source { &self.bs.source() }
// }

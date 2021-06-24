trait ID: Sized + PartialEq + Eq + PartialOrd + Ord {}

impl<T> ID for T where T: Sized + PartialEq + Eq + PartialOrd + Ord {}

trait CompositeID<T, U>: ID where T: ID, U: ID {
    fn new(id1: T, id2: U) -> Self;

    fn get_first(&self) -> T;
    fn get_second(&self) -> U;
}

type BookIDType = u8;
type ChapterIDType = u16;
type ContentIDType = u32;


#[test]
fn test_id_properties() {
    let book: BookID = BookID::new(3);
    let chapter: ChapterID = ChapterID::new(book.clone(), 7);
    let content: ContentID = ContentID::new(chapter.clone(), 12);

    assert_eq!(content.get_book_id(), book);
    assert_eq!(content.get_chapter_id(), chapter.clone());
    assert_eq!(chapter.get_book_id(), book);
}


#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BookID(BookIDType);

impl BookID {
    pub fn new(id_value: BookIDType) -> Self {
        Self(id_value)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ChapterID(ChapterIDType);

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ContentID(u32);

impl ChapterID {
    fn get_book_id(&self) -> BookID {
        self.get_first()
    }
}

impl ContentID {
    fn get_chapter_id(&self) -> ChapterID {
        self.get_first()
    }

    fn get_book_id(&self) -> BookID {
        self.get_chapter_id().get_book_id()
    }
}


impl From<BookIDType> for BookID {
    fn from(num: BookIDType) -> Self {
        BookID(num)
    }
}

impl From<ChapterIDType> for ChapterID {
    fn from(num: ChapterIDType) -> Self {
        ChapterID(num)
    }
}

impl From<ContentIDType> for ContentID {
    fn from(num: ContentIDType) -> Self {
        ContentID(num)
    }
}


impl CompositeID<BookID, u8> for ChapterID {
    fn new(id1: BookID, id2: u8) -> Self {
        Self( (((*id1) as u16) << 8) as u16 + (id2 as u16) )
    }

    fn get_first(&self) -> BookID {
        BookID(((self.0 & 0xFF00) >> 8) as u8)
    }

    fn get_second(&self) -> u8 {
        ((self.0 & 0xFF)) as u8
    }
}

impl CompositeID<ChapterID, u16> for ContentID {
    fn new(id1: ChapterID, id2: u16) -> Self {
        Self( (((*id1) as u32) << 16) as u32 + (id2 as u32) )
    }

    fn get_first(&self) -> ChapterID {
        ChapterID(((self.0 & 0xFFFF0000) >> 16) as u16)
    }

    fn get_second(&self) -> u16 {
        ((self.0 & 0xFFFF)) as u16
    }
}


use std::ops::Deref;

impl Deref for BookID {
    type Target = BookIDType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ChapterID {
    type Target = ChapterIDType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ContentID {
    type Target = ContentIDType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
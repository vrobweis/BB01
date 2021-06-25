pub(crate) trait Id: Sized + PartialEq + Eq + PartialOrd + Ord {}

impl<T> Id for T where T: Sized + PartialEq + Eq + PartialOrd + Ord {}

pub(crate) trait CompositeId<T, U>: Id where T: Id, U: Id {
    fn new(id1: T, id2: U) -> Self;

    fn get_first(&self) -> T;
    fn get_second(&self) -> U;
}

type BookIdType = u8;
type ChapterIdType = u16;
type ContentIdType = u32;


#[test]
fn test_id_properties() {
    let book: BookId = BookId::new(3);
    let chapter: ChapterId = ChapterId::new(book.clone(), 7);
    let content: ContentId = ContentId::new(chapter.clone(), 12);

    assert_eq!(content.get_book_id(), book);
    assert_eq!(content.get_chapter_id(), chapter.clone());
    assert_eq!(chapter.get_book_id(), book);
}


#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BookId(BookIdType);

impl BookId {
    pub fn new(id_value: BookIdType) -> Self {
        Self(id_value)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChapterId(ChapterIdType);

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContentId(u32);

impl ChapterId {
    fn get_book_id(&self) -> BookId {
        self.get_first()
    }
}

impl ContentId {
    fn get_chapter_id(&self) -> ChapterId {
        self.get_first()
    }

    fn get_book_id(&self) -> BookId {
        self.get_chapter_id().get_book_id()
    }
}


impl From<BookIdType> for BookId {
    fn from(num: BookIdType) -> Self {
        BookId(num)
    }
}

impl From<ChapterIdType> for ChapterId {
    fn from(num: ChapterIdType) -> Self {
        ChapterId(num)
    }
}

impl From<ContentIdType> for ContentId {
    fn from(num: ContentIdType) -> Self {
        ContentId(num)
    }
}


impl CompositeId<BookId, u8> for ChapterId {
    fn new(id1: BookId, id2: u8) -> Self {
        Self( (((*id1) as u16) << 8) as u16 + (id2 as u16) )
    }

    fn get_first(&self) -> BookId {
        BookId(((self.0 & 0xFF00) >> 8) as u8)
    }

    fn get_second(&self) -> u8 {
        ((self.0 & 0xFF)) as u8
    }
}

impl CompositeId<ChapterId, u16> for ContentId {
    fn new(id1: ChapterId, id2: u16) -> Self {
        Self( (((*id1) as u32) << 16) as u32 + (id2 as u32) )
    }

    fn get_first(&self) -> ChapterId {
        ChapterId(((self.0 & 0xFFFF0000) >> 16) as u16)
    }

    fn get_second(&self) -> u16 {
        ((self.0 & 0xFFFF)) as u16
    }
}


use std::ops::Deref;

impl Deref for BookId {
    type Target = BookIdType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ChapterId {
    type Target = ChapterIdType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ContentId {
    type Target = ContentIdType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
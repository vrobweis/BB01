use std::rc::Rc;

use crate::{Library, Book, Chapter, Content};

struct AppData {
    state: State,

}

#[derive(Debug, Clone)]
pub enum State {
    Starting,

    LibraryPreview { },
    BookPreview {  book: Rc<Book>},
    ChapterPreview {  book: Rc<Book>, chapter: Rc<Chapter> },

    Read {  book: Rc<Book>, chapter: Rc<Chapter>, content: Content },

    ManageBook{  book: Rc<Book>},
    AddBook{ },

    Failure(String)
}


#[derive(Debug, Clone)]
pub enum Event {
    FinishedLoading,
    SelectBook (Book),
    SelectChapter (Chapter),
    SelectContent (Content),

    Back(State),
}

impl State {
    fn next(self, event: Event) -> State {
        match (self, event) {
            (State::Starting, Event::FinishedLoading) => State::LibraryPreview {  },
            (State::LibraryPreview {  }, Event::SelectBook(book)) => State::BookPreview { book: Rc::new(book) },
            (State::BookPreview { book }, Event::SelectChapter(chapter)) => State::ChapterPreview { book, chapter: Rc::new(chapter) },
            (State::ChapterPreview { book, chapter }, Event::SelectContent(content)) => State::Read { book, chapter, content },

            (s, Event::Back(previous_state)) => match s {
                State::Starting => todo!(),
                State::LibraryPreview {  } => todo!(),
                State::BookPreview { book  } => State::LibraryPreview {  },
                State::ChapterPreview { book, chapter } => State::BookPreview { book },
                State::Read { book, chapter, content } =>  State::ChapterPreview { book, chapter },
                State::ManageBook{ book } => todo!(),
                State::AddBook{  } => todo!(),
                State::Failure(_) => todo!(),
            },

            (s, e) => State::Failure(format!("Wrong state, event combination: {:#?} {:#?}", s, e))
        }
    }
}

impl AppData {

    fn next(&mut self, event: Event) {
        self.state = self.state.clone().next(event);
    }

    fn run(&self) {
        match &self.state {
            State::Starting => todo!(),
            State::LibraryPreview {  } => todo!(),
            State::BookPreview { book } => todo!(),
            State::ChapterPreview { book, chapter } => todo!(),
            State::ManageBook { book } => todo!(),
            State::AddBook {  } => todo!(),
            State::Read { book, chapter, content } => todo!(),
            State::Failure(_) => todo!(),
        }
    }
}



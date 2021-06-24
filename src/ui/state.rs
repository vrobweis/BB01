use crate::{Chapter, Content};


enum AppState {
    Starting,

    StartMenu,

    LibraryPreview,
    BookPreview,
    ChapterPreview,


    ManageBook,
    AddBook,

    Search,
    SelectChapter,

    Read { chapter: Chapter, current_content: Content }

}
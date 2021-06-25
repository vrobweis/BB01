use crate::{identifiers::*};


#[derive(Debug, Clone)]
pub enum AppState {
    Starting,

    StartMenu,

    LibraryPreview,
    BookPreview,
    ChapterPreview,


    ManageBook,
    AddBook,

    Search,
    SelectChapter,

    Read { current_content: ContentId }

}

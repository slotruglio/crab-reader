use druid::im::Vector;
use druid::{widget::Flex, WidgetPod};
use druid::{Data, Lens, LensExt, Widget, WidgetExt};

use super::book::BookState;

#[derive(Clone, PartialEq, Data, Lens)]
pub struct BookLibraryState {
    pub books: Vector<BookState>,
}

impl BookLibraryState {
    pub fn new() -> Self {
        Self {
            books: Vector::new(),
        }
    }

    pub fn with_books(&mut self, books: Vector<BookState>) -> &mut Self {
        self.books = books;
        self
    }

    pub fn get(&mut self) -> Self {
        self.clone()
    }

    pub fn build(&self) -> Flex<BookLibraryState> {
        let mut row = Flex::row();
        for (idx, book) in self.books.iter().enumerate() {
            row.add_flex_child(
                book.clone()
                    .build()
                    .padding(5.0)
                    .lens(BookLibraryState::books.index(idx)),
                1.0,
            );
        }
        row
    }
}

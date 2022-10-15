use druid::{im::Vector, Data, Lens, Selector};

use super::{book::GUIBook, mockup::MockupBook};

pub const SELECTED_BOOK_SELECTOR: Selector<Option<u16>> = Selector::new("selected-book");

type Book = MockupBook;

/// This trait deinfes all the functionalities that a `Library` struct must expose
/// in order to be rendered correctly in the GUI of the application
pub trait GUILibrary<B: GUIBook + PartialEq + Data> {
    /// Empty/default constructo
    fn new() -> Self;

    /// Add a book to the library
    fn add_book(&mut self, book: &B);

    /// Remove a book from the library
    /// The `idx` argument is the index in the array (relax this constraint??)
    fn remove_book(&mut self, idx: u16);

    /// Get a mutable reference to a book in the library
    /// The `idx` argument is the index in the array (relax this constraint??)
    fn get_book_mut(&mut self, idx: u16) -> Option<&mut B>;

    /// Get a reference to a book in the library
    /// The `idx` argument is the index in the array (relax this constraint??)
    fn get_book(&self, idx: u16) -> Option<&B>;

    /// Get the number of books in the library
    fn number_of_books(&self) -> usize;

    /// Get the index of the selected book
    /// If no book is selected, return `None`
    fn get_selected_book_idx(&self) -> Option<u16>;

    /// Get a reference to the selected book
    /// If no book is selected, return `None`
    fn get_selected_book(&self) -> Option<&B>;

    /// Get a mutable reference to the selected book
    /// If no book is selected, return `None`
    fn get_selected_book_mut(&mut self) -> Option<&mut B>;

    /// Set the selected book by its index
    /// If the index is out of bounds, do nothing
    fn set_selected_book_idx(&mut self, idx: u16);

    /// Unselct the currently selected book
    fn unselect_current_book(&mut self);
}

#[derive(Clone, Lens, PartialEq, Data)]
pub struct MockupLibrary<B: GUIBook + PartialEq + Data> {
    books: Vector<B>,
    selected_book: Option<u16>,
}

impl GUILibrary<Book> for MockupLibrary<Book> {
    fn new() -> Self {
        Self {
            books: Vector::new(),
            selected_book: None,
        }
    }

    fn add_book(&mut self, book: &Book) {
        self.books
            .push_back(book.clone().with_index(self.books.len() as u16));
    }

    fn remove_book(&mut self, idx: u16) {
        let idx = idx as usize;
        if let Some(_) = self.books.get(idx) {
            self.books.remove(idx);
        }
    }

    fn get_book_mut(&mut self, idx: u16) -> Option<&mut Book> {
        let idx = idx as usize;
        self.books.get_mut(idx)
    }

    fn get_book(&self, idx: u16) -> Option<&Book> {
        let idx = idx as usize;
        self.books.get(idx)
    }

    fn get_selected_book_idx(&self) -> Option<u16> {
        self.selected_book.clone()
    }

    fn number_of_books(&self) -> usize {
        self.books.len()
    }

    fn get_selected_book_mut(&mut self) -> Option<&mut MockupBook> {
        if let Some(idx) = self.get_selected_book_idx() {
            self.get_book_mut(idx)
        } else {
            None
        }
    }

    fn set_selected_book_idx(&mut self, idx: u16) {
        if idx < self.number_of_books() as u16 {
            self.selected_book = Some(idx);
        }
    }

    fn get_selected_book(&self) -> Option<&Book> {
        if let Some(idx) = self.get_selected_book_idx() {
            self.get_book(idx)
        } else {
            None
        }
    }

    fn unselect_current_book(&mut self) {
        self.selected_book = None;
    }
}

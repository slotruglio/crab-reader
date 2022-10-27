use super::book::GUIBook;
use druid::{Data, Selector};
pub const SELECTED_BOOK_SELECTOR: Selector<Option<usize>> = Selector::new("selected-book");

/// This trait deinfes all the functionalities that a `Library` struct must expose
/// in order to be rendered correctly in the GUI of the application
pub trait GUILibrary<B: GUIBook + PartialEq + Data> {
    /// Add a book to the library
    fn add_book(&mut self, book: impl Into<String>);

    /// Remove a book from the library
    /// The `idx` argument is the index in the array (relax this constraint??)
    fn remove_book(&mut self, idx: usize);

    /// Get a mutable reference to a book in the library
    /// The `idx` argument is the index in the array (relax this constraint??)
    fn get_book_mut(&mut self, idx: usize) -> Option<&mut B>;

    /// Get a reference to a book in the library
    /// The `idx` argument is the index in the array (relax this constraint??)
    fn get_book(&self, idx: usize) -> Option<&B>;

    /// Get the number of books in the library
    fn number_of_books(&self) -> usize;

    /// Get the index of the selected book
    /// If no book is selected, return `None`
    fn get_selected_book_idx(&self) -> Option<usize>;

    /// Get a reference to the selected book
    /// If no book is selected, return `None`
    fn get_selected_book(&self) -> Option<&B>;

    /// Get a mutable reference to the selected book
    /// If no book is selected, return `None`
    fn get_selected_book_mut(&mut self) -> Option<&mut B>;

    /// Set the selected book by its index
    /// If the index is out of bounds, do nothing
    fn set_selected_book_idx(&mut self, idx: usize);

    /// Unselct the currently selected book
    fn unselect_current_book(&mut self);
}

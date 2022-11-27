use crate::models::library::SortBy;
use druid::Data;
use std::sync::Arc;

pub trait GUIBook: PartialEq + Data {
    /// Returns the title
    fn get_title(&self) -> String;

    /// Builder pattern for title
    fn with_title(self, title: impl Into<String>) -> Self;

    /// Sets the title for the book
    fn set_title(&mut self, title: impl Into<String>);

    /// Returns the author
    fn get_author(&self) -> String;

    /// Builder pattern for author
    fn with_author(self, author: impl Into<String>) -> Self;

    /// Sets the author for the book
    fn set_author(&mut self, author: impl Into<String>);

    /// Returns the number of pages
    fn get_number_of_pages(&self) -> usize;

    /// Builder pattern for number of pages
    fn with_number_of_pages(self, npages: usize) -> Self;

    /// Sets the number of pages for the book
    fn set_number_of_pages(&mut self, npages: usize);

    /// Returns the number of read pages
    fn get_number_of_read_pages(&self) -> usize;

    /// Builder pattern for number of read pages
    fn with_number_of_read_pages(self, read_pages: usize) -> Self;

    /// Sets the number of read pages for the book
    fn set_number_of_read_pages(&mut self, read_pages: usize);

    /// Returns the index of the book.
    ///
    /// The idx is intended to be the position in the array of the `Library` struct (relax this constraint?)
    fn get_index(&self) -> usize;

    /// Builder pattern for index
    ///
    /// The idx is intended to be the position in the array of the `Library` struct (relax this constraint?)
    fn with_index(self, idx: usize) -> Self;

    /// Sets the index of the book.
    ///
    /// The idx is intended to be the position in the array of the `Library` struct (relax this constraint?)
    fn set_index(&mut self, idx: usize);

    /// Builds the cover image from the cover image data
    fn build_cover(&self) -> Result<Box<[u8]>, String>;

    /// Builds the cover image from the cover image data with the specified size
    fn build_cover_with_size(&self, width: u32, height: u32) -> Result<Box<[u8]>, String>;

    /// Builder pattern for the description (i.e, like a synopsis for the book)
    fn with_description(self, description: impl Into<String>) -> Self;

    /// Sets the description (i.e, like a synopsis for the book)
    fn set_description(&mut self, description: impl Into<String>);

    /// Returns the selected state of the book
    fn is_selected(&self) -> bool;

    /// Sets the book as selected
    fn set_selected(&mut self, selected: bool);

    /// Set the book as selected
    fn select(&mut self);

    /// Set the book as unselected
    fn unselect(&mut self);

    /// Returns the cover of this book
    fn get_cover_image(&self) -> Arc<Vec<u8>>;

    /// Sets the cover image
    fn set_cover_image(&mut self, cover_image: Vec<u8>);

    fn is_filtered_out(&self) -> bool;

    fn set_filtered_out(&mut self, filtered_out: bool);

    fn is_favorite(&self) -> bool;
}

/// This trait deinfes all the functionalities that a `Library` struct must expose
/// in order to be rendered correctly in the GUI of the application
pub trait GUILibrary {
    type B;
    /// Add a book to the library
    fn add_book(&mut self, book: impl Into<String>);

    /// Remove a book from the library
    /// The `idx` argument is the index in the array (relax this constraint??)
    fn remove_book(&mut self, idx: usize);

    /// Get a mutable reference to a book in the library
    /// The `idx` argument is the index in the array (relax this constraint??)
    fn get_book_mut(&mut self, idx: usize) -> Option<&mut Self::B>;

    /// Get a reference to a book in the library
    /// The `idx` argument is the index in the array (relax this constraint??)
    fn get_book(&self, idx: usize) -> Option<&Self::B>;

    /// Get the number of books in the library
    fn number_of_books(&self) -> usize;

    /// Get the index of the selected book
    /// If no book is selected, return `None`
    fn get_selected_book_idx(&self) -> Option<usize>;

    /// Get a reference to the selected book
    /// If no book is selected, return `None`
    fn get_selected_book(&self) -> Option<&Self::B>;

    /// Get a mutable reference to the selected book
    /// If no book is selected, return `None`
    fn get_selected_book_mut(&mut self) -> Option<&mut Self::B>;

    /// Set the selected book by its index
    /// If the index is out of bounds, do nothing
    fn set_selected_book_idx(&mut self, idx: usize);

    /// Unselct the currently selected book
    fn unselect_current_book(&mut self);

    /// Schedule the loading of a book.
    fn schedule_cover_loading(&mut self, path: impl Into<String>, idx: usize);

    /// Check if any covers are loaded and set the cover for the corresponding book
    fn check_covers_loaded(&mut self) -> bool;

    /// Get the order in which the books are sorted
    fn get_sort_order(&self) -> SortBy;

    fn toggle_fav_filter(&mut self);

    fn only_fav(&self) -> bool;

    fn next_book_idx(&self) -> Option<usize>;

    fn prev_book_idx(&self) -> Option<usize>;
}

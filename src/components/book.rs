use std::rc::Rc;

/// This trait defines all the methods that a `Book` struct must implement
/// in order to be rendered visually correct in the GUI of the application.
pub trait GUIBook {
    /// Empty/Default constructor
    fn new() -> Self;

    /// Returns the title
    fn get_title(&self) -> Rc<String>;

    /// Builder pattern for title
    fn with_title(self, title: impl Into<String>) -> Self;

    /// Sets the title for the book
    fn set_title(&mut self, title: impl Into<String>);

    /// Returns the author
    fn get_author(&self) -> Rc<String>;

    /// Builder pattern for author
    fn with_author(self, author: impl Into<String>) -> Self;

    /// Sets the author for the book
    fn set_author(&mut self, author: impl Into<String>);

    /// Returns the number of pages
    fn get_number_of_pages(&self) -> u16;

    /// Builder pattern for number of pages
    fn with_number_of_pages(self, npages: u16) -> Self;

    /// Sets the number of pages for the book
    fn set_number_of_pages(&mut self, npages: u16);

    /// Returns the number of read pages
    fn get_number_of_read_pages(&self) -> u16;

    /// Builder pattern for number of read pages
    fn with_number_of_read_pages(self, read_pages: u16) -> Self;

    /// Sets the number of read pages for the book
    fn set_number_of_read_pages(&mut self, read_pages: u16);

    /// Returns the index of the book.
    ///
    /// The idx is intended to be the position in the array of the `Library` struct (relax this constraint?)
    fn get_index(&self) -> u16;

    /// Builder pattern for index
    ///
    /// The idx is intended to be the position in the array of the `Library` struct (relax this constraint?)
    fn with_index(self, idx: u16) -> Self;

    /// Sets the index of the book.
    ///
    /// The idx is intended to be the position in the array of the `Library` struct (relax this constraint?)
    fn set_index(&mut self, idx: u16);

    /// Returns the path to the cover image
    ///
    /// For now, the path is relative to the root of the project (relax this constraint?)
    fn get_cover_path(&self) -> Rc<String>;

    /// Builder pattern for cover path
    ///
    /// For now, the path is relative to the root of the project (relax this constraint?)
    fn with_cover_path(self, cover_path: impl Into<String>) -> Self;

    /// Sets the path to the cover image
    ///
    /// For now, the path is relative to the root of the project (relax this constraint?)
    fn set_cover_path(&mut self, cover_path: impl Into<String>);

    /// Returns the description (i.e, like a synopsis for the book)
    fn get_description(&self) -> Rc<String>;

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
}

use druid::{im::Vector, Data, Lens};

use super::{
    book::{Book, GUIBook},
    book_cover::BookCover,
    library::GUILibrary,
};

#[derive(Clone, Lens, PartialEq, Data)]
pub struct MockupLibrary<B: GUIBook + PartialEq + Data> {
    books: Vector<B>,
    selected_book: Option<usize>,
}

impl GUILibrary<Book> for MockupLibrary<Book> {
    fn new() -> Self {
        if let Ok(cwd) = std::env::current_dir() {
            let dir = cwd.join("src").join("epubs");
            if let Ok(files) = std::fs::read_dir(dir) {
                let books: Vector<Book> = files
                    .filter(|file| file.is_ok())
                    .map(|file| file.unwrap().path())
                    .filter(|filename| filename.extension().unwrap() == "epub")
                    .map(|path| path.to_str().unwrap().to_string())
                    .map(|path| Book::new(path))
                    .collect();
                Self {
                    books,
                    selected_book: None,
                }
            } else {
                panic!("Couldn't list `epubs` dir in {}", cwd.display());
            }
        } else {
            panic!("Cannot get current working directory");
        }
    }

    fn add_book(&mut self, path: impl Into<String>) {
        let book = Book::new(path.into()).with_index(self.books.len());
        self.books.push_back(book);
    }

    fn remove_book(&mut self, idx: usize) {
        if let Some(_) = self.books.get(idx) {
            self.books.remove(idx);
        }
    }

    fn get_book_mut(&mut self, idx: usize) -> Option<&mut Book> {
        self.books.get_mut(idx)
    }

    fn get_book(&self, idx: usize) -> Option<&Book> {
        self.books.get(idx)
    }

    fn get_selected_book_idx(&self) -> Option<usize> {
        self.selected_book.clone()
    }

    fn number_of_books(&self) -> usize {
        self.books.len()
    }

    fn get_selected_book_mut(&mut self) -> Option<&mut Book> {
        if let Some(idx) = self.get_selected_book_idx() {
            self.get_book_mut(idx)
        } else {
            None
        }
    }

    fn set_selected_book_idx(&mut self, idx: usize) {
        if idx < self.number_of_books() {
            self.unselect_current_book();
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
        if let Some(selected) = self.get_selected_book_mut() {
            selected.unselect();
        }
        self.selected_book = None;
    }
}

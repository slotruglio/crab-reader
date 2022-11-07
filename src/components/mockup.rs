use std::path::{Path, PathBuf};

use druid::{im::Vector, Data, Lens};

use crate::utils::epub_utils;

use super::{
    book::{Book, GUIBook},
    library::GUILibrary,
};

const SAVED_BOOKS_PATH: &str = "saved_books/";

#[derive(Clone, Lens, PartialEq, Data)]
pub struct MockupLibrary<B: GUIBook + PartialEq + Data> {
    books: Vector<B>,
    selected_book: Option<usize>,
    sorted_by: SortBy,
}

impl MockupLibrary<Book> {
    pub fn new() -> Self {
        let mut lib = Self {
            books: Vector::new(),
            selected_book: None,
            sorted_by: SortBy::Title,
        };
        if let Ok(paths) = lib.epub_paths() {
            for path in paths {
                let path: String = path.to_str().unwrap().to_string();
                lib.add_book(path);
            }
        }
        lib
    }

    pub fn epub_dir(&self) -> Result<PathBuf, String> {
        let path = std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join("src")
            .join("epubs");
        return if path.is_dir() {
            Ok(path)
        } else {
            Err(format!("Dir {} not found", path.display()))
        };
    }

    pub fn epub_paths(&self) -> Result<Vector<PathBuf>, String> {
        let dir = self.epub_dir()?;
        let files = std::fs::read_dir(dir).map_err(|e| e.to_string())?;
        let vec: Vector<PathBuf> = files
            .filter(|file| file.is_ok())
            .map(|file| file.unwrap().path())
            .filter(|filename| filename.extension().unwrap_or_default() == "epub")
            .collect();
        Ok(vec)
    }
}

impl GUILibrary<Book> for MockupLibrary<Book> {
    fn add_book(&mut self, path: impl Into<String>) {
        let path: String = path.into();
        let file_name = path.split("/").last().unwrap();
        let folder_name = file_name.split(".").next().unwrap();
        // extract metadata and chapters
        if !Path::new(&format!("{}{}", SAVED_BOOKS_PATH, folder_name)).exists() {
            let _res = epub_utils::extract_all(&path)
                .expect(format!("Failed to extract {}", file_name).as_str());
        }

        let book = Book::new(path).with_index(self.books.len());
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

#[derive(Clone, PartialEq, Data)]
pub enum SortBy {
    Title,
    TitleRev,
    Author,
    AuthorRev,
    PercRead,
    PercReadRev,
}

impl MockupLibrary<Book> {
    pub fn sort_by(&mut self, by: SortBy) {
        if self.sorted_by == by {
            return;
        }

        self.books.sort_by(|one, other| match by {
            SortBy::Title => one.get_title().cmp(&other.get_title()),
            SortBy::TitleRev => other.get_title().cmp(&one.get_title()),
            SortBy::Author => one.get_author().cmp(&other.get_author()),
            SortBy::AuthorRev => other.get_author().cmp(&one.get_author()),
            SortBy::PercRead => one
                .get_perc_read()
                .partial_cmp(&other.get_perc_read())
                .unwrap(),
            SortBy::PercReadRev => other
                .get_perc_read()
                .partial_cmp(&one.get_perc_read())
                .unwrap(),
            _ => one.get_title().cmp(&other.get_title()),
        });
        self.sorted_by = by;
    }
}

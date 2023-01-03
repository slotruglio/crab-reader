use derivative::Derivative;
use druid::{im::Vector, Data, Lens};
use epub::doc::EpubDoc;
use image::io::Reader as ImageReader;
use std::{io::Cursor, path::PathBuf, rc::Rc, sync::Arc};

use crate::traits::reader::BookManagement;
use crate::utils::thread_loader::{ThreadLoader, ThreadResult};
use crate::{
    models::book::Book,
    traits::gui::{GUIBook, GUILibrary},
    utils::{
        dir_manager::{get_epub_dir, get_saved_books_dir},
        epub_utils,
    },
};

pub struct LibraryFilterLens;

impl Lens<Library<Book>, String> for LibraryFilterLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &Library<Book>, f: F) -> V {
        f(&data.filter_by)
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut Library<Book>, f: F) -> V {
        let mut filter = data.filter_by.to_string();
        let res = f(&mut filter);
        data.filter_by = filter.into();
        data.filter_books();
        res
    }
}

#[derive(Clone, Derivative, Lens, Data)]
#[derivative(PartialEq)]
pub struct Library<B: GUIBook + Data> {
    books: Vector<B>,
    selected_book: Option<usize>,
    sorted_by: SortBy,
    filter_by: Rc<String>,
    filter_fav: bool,
    visible_books: usize,
    #[data(ignore)]
    #[derivative(PartialEq = "ignore")]
    cover_loader: Arc<ThreadLoader<Vec<u8>>>,
    #[data(ignore)]
    #[derivative(PartialEq = "ignore")]
    book_loader: Arc<ThreadLoader<Book>>,
    pub do_paint_shadows: bool,
}

impl Library<Book> {
    pub fn new() -> Self {
        let mut lib = Self {
            books: Vector::new(),
            selected_book: None,
            sorted_by: SortBy::Title,
            filter_by: String::default().into(),
            visible_books: 0,
            cover_loader: ThreadLoader::default().into(),
            book_loader: ThreadLoader::default().into(),
            filter_fav: false,
            do_paint_shadows: false,
        };

        if let Ok(paths) = lib.epub_paths() {
            for path in paths {
                let path: String = path.to_str().unwrap().to_string();
                lib.schedule_book_loading(&path);
            }
        }
        lib
    }

    #[cfg(test)]
    pub fn new_no_cache() -> Self {
        let lib = Self {
            books: Vector::new(),
            selected_book: None,
            sorted_by: SortBy::Title,
            filter_by: String::default().into(),
            visible_books: 0,
            cover_loader: ThreadLoader::default().into(),
            book_loader: ThreadLoader::default().into(),
            filter_fav: false,
            do_paint_shadows: false,
        };
        lib
    }

    pub fn epub_dir(&self) -> Result<PathBuf, String> {
        let path = get_epub_dir();
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

impl GUILibrary for Library<Book> {
    type B = Book;
    fn check_books_loaded(&mut self) -> bool {
        if let Some(result) = self.book_loader.try_recv() {
            self.add_book(result.value());
            true
        } else {
            false
        }
    }

    #[cfg(test)]
    fn check_books_loaded_blocking(&mut self) {
        let book = self.book_loader.recv();
        self.add_book(book.value());
    }

    fn schedule_book_loading(&mut self, path: impl Into<String>) {
        let path = path.into();
        let tx = self.book_loader.tx();
        self.book_loader.execute(move || {
            let file_name = path.split("/").last().unwrap();
            let folder = file_name.split(".").next().unwrap();
            if !get_saved_books_dir().join(folder).exists() {
                let _res = epub_utils::extract_all(&path)
                    .expect(format!("Failed to extract {}", file_name).as_str());
            }
            let book = Book::new(&path);
            let result = ThreadResult::new(book, 0);
            tx.send(result)
                .expect(format!("Failed to send {}", file_name).as_str());
        });
    }

    fn add_book(&mut self, book: Self::B) {
        let idx = self.books.len();
        let path = book.get_path().clone();
        self.books.push_back(book.with_index(idx));
        self.schedule_cover_loading(path, idx);
        self.visible_books += 1;
    }

    fn remove_book(&mut self, idx: usize) -> bool {
        if let Some(book) = self.books.get(idx) {
            if !book.is_filtered_out() {
                self.visible_books -= 1;
            }
            self.books.remove(idx);
            return true;
        }
        false
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
            self.books[idx].select();
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

    fn schedule_cover_loading(&mut self, path: impl Into<String>, idx: usize) {
        let path = path.into();
        let tx = self.cover_loader.tx();
        self.cover_loader.execute(move || {
            let mut epub = EpubDoc::new(path).map_err(|e| e.to_string()).unwrap();
            let cover = epub.get_cover().map_err(|e| e.to_string()).unwrap();
            let reader = ImageReader::new(Cursor::new(cover))
                .with_guessed_format()
                .map_err(|e| e.to_string())
                .unwrap();
            let image = reader.decode().map_err(|e| e.to_string()).unwrap();
            let thumbnail = image.thumbnail_exact(150, 250);
            let rgb = thumbnail.to_rgb8().to_vec();
            let result = ThreadResult::new(rgb, idx);
            tx.send(result).expect("Error sending result cover");
        });
    }

    fn check_covers_loaded(&mut self) -> bool {
        let mut loaded = false;
        while let Some(result) = self.cover_loader.try_recv() {
            let book = self.get_book_mut(result.idx());
            if let Some(book) = book {
                let cover = result.value();
                book.set_cover_buffer(cover);
                loaded = true;
            }
        }
        loaded
    }

    fn get_sort_order(&self) -> SortBy {
        self.sorted_by.clone()
    }

    fn toggle_fav_filter(&mut self) {
        self.filter_fav = !self.filter_fav;
        self.filter_books();
    }

    fn only_fav(&self) -> bool {
        self.filter_fav
    }

    fn next_book_idx(&self) -> Option<usize> {
        let Some(idx) = self.get_selected_book_idx() else {
            return self.books.iter().enumerate().find(|(_, book)| !book.is_filtered_out()).map(|(idx, _)| idx)
        };
        self.books
            .iter()
            .enumerate()
            .find(|(i, book)| *i > idx && !book.is_filtered_out())
            .map(|(idx, _)| idx)
    }

    fn prev_book_idx(&self) -> Option<usize> {
        let Some(idx) = self.get_selected_book_idx() else {
            return self.books.iter().enumerate().find(|(_, book)| !book.is_filtered_out()).map(|(idx, _)| idx);
        };
        self.books
            .iter()
            .enumerate()
            .rev()
            .find(|(cidx, b)| !b.is_filtered_out() && *cidx < idx)
            .map(|(idx, _)| idx)
    }

    fn get_number_of_visible_books(&self) -> usize {
        self.visible_books
    }

    fn get_filter_by(&self) -> Rc<String> {
        self.filter_by.clone()
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

impl Library<Book> {
    pub fn sort_by(&mut self, by: SortBy) {
        if self.sorted_by == by {
            return;
        }

        let old_title = self
            .get_selected_book()
            .map(|b| b.get_title().to_string())
            .unwrap_or_default();
        let mut new_idx = None;

        self.books.sort_by(|one, other| match by {
            SortBy::Title => one
                .get_title()
                .to_lowercase()
                .cmp(&other.get_title().to_lowercase()),
            SortBy::TitleRev => other
                .get_title()
                .to_lowercase()
                .cmp(&one.get_title().to_lowercase()),
            SortBy::Author => one
                .get_author()
                .to_lowercase()
                .cmp(&other.get_author().to_lowercase()),
            SortBy::AuthorRev => other
                .get_author()
                .to_lowercase()
                .cmp(&one.get_author().to_lowercase()),
            SortBy::PercRead => one
                .get_perc_read()
                .partial_cmp(&other.get_perc_read())
                .unwrap(),
            SortBy::PercReadRev => other
                .get_perc_read()
                .partial_cmp(&one.get_perc_read())
                .unwrap(),
        });
        self.books.iter_mut().enumerate().for_each(|(i, book)| {
            book.set_index(i);
            if book.get_title() == old_title {
                new_idx = Some(i);
            }
        });
        self.selected_book = new_idx;
        self.sorted_by = by;
    }

    pub fn filter_books(&mut self) {
        let filter = self.get_filter_by();
        let only_fav = self.filter_fav;
        let mut cnt = 0;
        self.books.iter_mut().for_each(|book| {
            let auth = book.get_author().to_lowercase();
            let title = book.get_title().to_lowercase();
            let auth_sim = rust_fuzzy_search::fuzzy_compare(filter.as_str(), &auth.as_str());
            let title_sim = rust_fuzzy_search::fuzzy_compare(filter.as_str(), &title.as_str());
            let basic_sim = if auth.contains(&*filter) || title.contains(&*filter) {
                1.0
            } else {
                0.0
            };

            let sim = auth_sim.max(title_sim).max(basic_sim);

            // what is a good number for this threshold??
            if sim < 0.3 {
                book.set_filtered_out(true);
            } else if only_fav && !book.is_favorite() {
                book.set_filtered_out(true);
            } else {
                book.set_filtered_out(false);
                cnt += 1;
            }
        });

        if let Some(book) = self.get_selected_book_mut() {
            if book.is_filtered_out() {
                book.unselect();
                self.unselect_current_book();
            }
        }
        self.visible_books = cnt;
    }
}

pub struct LibrarySelectedBookLens;

impl<L: GUILibrary<B = Book>> Lens<L, Book> for LibrarySelectedBookLens {
    fn with<V, F: FnOnce(&Book) -> V>(&self, data: &L, f: F) -> V {
        let Some(book) = data.get_selected_book() else {
            return f(&Book::empty_book());
        };
        f(book)
    }

    fn with_mut<V, F: FnOnce(&mut Book) -> V>(&self, data: &mut L, f: F) -> V {
        let Some(book) = data.get_selected_book_mut() else {
            return f(&mut Book::empty_book());
        };
        f(book)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPUBS_PATHS: [&str; 3] = [
        "bibbia_la_sacra_bibbia.epub",
        "collodi_pinocchio.epub",
        "svevo_la_coscienza_di_zeno.epub",
    ];

    #[test]
    fn library_starts_empty() {
        let lib = Library::new_no_cache();
        assert_eq!(lib.number_of_books(), 0);
        assert_eq!(lib.get_number_of_visible_books(), 0);
    }

    #[test]
    fn library_test_epubs_present() {
        let cwd = std::env::current_dir().expect("Could not get current directory");
        let dir = cwd
            .join("test_books")
            .read_dir()
            .expect("Could not read test_epubs");
        assert!(dir
            .into_iter()
            .map(|p| p
                .expect("Could not get path")
                .file_name()
                .into_string()
                .expect("Could not get path string"))
            .all(|p| EPUBS_PATHS.contains(&p.as_str())));
    }

    #[test]
    fn library_recv_book() {
        let mut lib = Library::new_no_cache();
        let first = EPUBS_PATHS[0];
        let cwd = std::env::current_dir().expect("Could not get current directory");
        let path = cwd.join("test_books").join(first);
        let file = path.to_str().expect("Could not get path string");
        lib.schedule_book_loading(file);

        let book = lib.book_loader.recv().value();
        assert_eq!(book.get_title(), "La Sacra Bibbia");
        assert_eq!(book.get_author(), "Autori vari");
    }

    fn setup_load_books() -> Library<Book> {
        let mut lib = Library::new_no_cache();
        let cwd = std::env::current_dir().expect("Could not get current directory");
        let path = cwd.join("test_books");

        for book in EPUBS_PATHS {
            let path = path.clone().join(book);
            let path = path.as_path().to_str().expect("Could not get path string");
            lib.schedule_book_loading(path);
        }

        for _ in 0..EPUBS_PATHS.len() {
            lib.check_books_loaded_blocking();
        }

        lib
    }

    #[test]
    fn library_load_books() {
        let lib = setup_load_books();

        assert_eq!(lib.number_of_books(), EPUBS_PATHS.len());
        assert_eq!(lib.get_number_of_visible_books(), EPUBS_PATHS.len());
    }

    #[test]
    fn library_favorites() {
        let mut lib = setup_load_books();

        assert!(lib.books.iter().all(|b| !b.is_favorite()));

        lib.get_book_mut(0)
            .expect("Could not get book")
            .set_favorite(true);
        let title = lib.get_book(0).expect("Could not get book").get_title();
        let fav = lib
            .books
            .iter()
            .filter(|b| b.is_favorite())
            .take(1)
            .map(|b| b.get_title())
            .next()
            .expect("Could not get favorite book");
        lib.toggle_fav_filter();

        assert_eq!(fav, title);
        assert_eq!(lib.get_number_of_visible_books(), 1);
        assert_eq!(lib.number_of_books(), EPUBS_PATHS.len());

        lib.books.iter_mut().for_each(|b| b.set_favorite(false));
    }

    #[test]
    fn library_search() {
        let mut lib = setup_load_books();

        if lib.only_fav() {
            lib.toggle_fav_filter();
        }

        lib.filter_by = Rc::from("Pinocchio".to_string());
        lib.filter_books();
        assert_eq!(lib.get_number_of_visible_books(), 1);

        lib.filter_by = Rc::from("Pnocchio".to_string()); // Fuzzy Search
        lib.filter_books();
        assert_eq!(lib.get_number_of_visible_books(), 1);

        lib.filter_by = Rc::from("Collodi".to_string());
        lib.filter_books();
        assert_eq!(lib.get_number_of_visible_books(), 1);

        lib.filter_by = Rc::from("Colodi".to_string()); // Fuzzy Search
        lib.filter_books();
        assert_eq!(lib.get_number_of_visible_books(), 1);
    }

    #[test]
    fn library_remove_book() {
        let mut lib = setup_load_books();
        let title = lib.get_book(0).expect("Could not get book").get_title();
        assert!(lib.remove_book(0));
        assert_eq!(lib.get_number_of_visible_books(), EPUBS_PATHS.len() - 1);
        assert_eq!(lib.number_of_books(), EPUBS_PATHS.len() - 1);
        assert!(lib.books.iter().all(|b| b.get_title() != title));
    }

    #[test]
    fn library_test_sorting() {
        let mut lib = setup_load_books();

        lib.sort_by(SortBy::PercRead);
        lib.sort_by(SortBy::Title);
        lib.books
            .iter()
            .map(|b| b.get_title())
            .zip(["La coscienza di Zeno", "La Sacra Bibbia", "Pinocchio"])
            .for_each(|(a, b)| assert_eq!(a, b));

        lib.sort_by(SortBy::Author);
        lib.books
            .iter()
            .map(|b| b.get_author())
            .zip([
                "Autori vari",
                "Carlo Collodi",
                "Italo (alias Ettore Schmitz) Svevo",
            ])
            .for_each(|(a, b)| assert_eq!(a, b));
    }
}

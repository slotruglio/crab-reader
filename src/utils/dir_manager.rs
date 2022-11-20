use std::path::{PathBuf, Path};

use dirs;

const APP_NAME: &str = "crab-reader";

fn get_app_dir() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap();
    dir.push(APP_NAME);
    let _ = std::fs::create_dir_all(&dir);
    // testing only
    get_fake_app_dir()

    // todo: return the real app dir
    // dir
}

fn get_fake_app_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

fn get_config_dir() -> PathBuf {
    let mut config_dir = get_app_dir();
    config_dir.push("conf");
    let _ = std::fs::create_dir_all(&config_dir);
    config_dir
}

/// Get the path to the directory where the user's books are stored.
pub fn get_saved_books_dir() -> PathBuf {
    let mut data_dir = get_app_dir();
    data_dir.push("saved_books");
    let _ = std::fs::create_dir_all(&data_dir);
    data_dir
}

/// Get the path to the directory where the user's edited books are stored.
pub fn get_edited_books_dir() -> PathBuf {
    let mut data_dir = get_app_dir();
    data_dir.push("edited_books");
    let _ = std::fs::create_dir_all(&data_dir);
    data_dir
}

/// Get the path to the directory where the user's EPUB are stored.
pub fn get_epub_dir() -> PathBuf {
    let mut data_dir = get_app_dir();
    data_dir.push("epubs");
    let _ = std::fs::create_dir_all(&data_dir);
    data_dir
}

/// Get path of the folder where cover images are stored
pub fn get_saved_covers_dir() -> PathBuf {
    let mut data_dir = get_app_dir();
    data_dir.push("covers");
    let _ = std::fs::create_dir_all(&data_dir);
    data_dir
}

/// Get path of the books with saved progress
pub fn get_savedata_path() -> PathBuf {
    let mut config_file = get_config_dir();
    config_file.push("books_saved.json");
    config_file
}

pub fn get_books_notes_path() -> PathBuf {
    let mut config_file = get_config_dir();
    config_file.push("books_notes.json");
    config_file
}

/// Get path of the metadata file given a book path
pub fn get_metadata_path(book_path: &String) -> PathBuf {
    let book_name = Path::new(book_path).file_stem().unwrap().to_str().unwrap();

    let mut book_dir = get_saved_books_dir()
    .join(book_name);

    let _ = std::fs::create_dir_all(&book_dir);
    book_dir.push("metadata.json");

    book_dir
}
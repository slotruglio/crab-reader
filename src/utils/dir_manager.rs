use std::path::PathBuf;

use dirs;

const APP_NAME: &str = "crab-reader";

fn get_app_dir() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap();
    dir.push(APP_NAME);

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
    config_dir
}

/// Get the path to the directory where the user's books are stored.
pub fn get_saved_books_dir() -> PathBuf {
    let mut data_dir = get_app_dir();
    data_dir.push("saved_books");
    data_dir
}

/// Get the path to the directory where the user's edited books are stored.
pub fn get_edited_books_dir() -> PathBuf {
    let mut data_dir = get_app_dir();
    data_dir.push("edited_books");
    data_dir
}

/// Get the path to the directory where the user's EPUB are stored.
pub fn get_epub_dir() -> PathBuf {
    let mut data_dir = get_app_dir();
    data_dir.push("epubs");
    data_dir
}

/// Get path of the books with saved progress
pub fn get_savedata_path() -> PathBuf {
    let mut config_file = get_config_dir();
    config_file.push("books_saved.json");
    config_file
}

/// Get path of the folder where cover images are stored
pub fn get_saved_covers_dir() -> PathBuf {
    let mut data_dir = get_app_dir();
    data_dir.push("covers");
    data_dir
}
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
    path::Path,
    sync::mpsc::channel,
};

use serde_json::{json, Value};

use crate::{MYENV, utils::dir_manager::{get_savedata_path, get_saved_books_dir}};

use super::envmanager::FontSize;

pub enum FileExtension {
    TXT,
    HTML,
    EPUB,
}

// waiting for implementation of this env var

/// function to save page of chapter of currently opened book
pub fn save_data<T: Into<String> + Clone>(
    book_path: T,
    chapter: usize,
    page: usize,
    content: T,
    font_size: FontSize,
    edited: bool,
) -> Result<(), Box<dyn std::error::Error>> {

    println!(
        "DEBUG saving data: {} {} {} {} {}",
        chapter,
        page,
        book_path.clone().into(),
        font_size.to_string(),
        edited
    );

    // check if exists a savedata file
    let savedata_path = get_savedata_path();
    let mut json = json!({});
    if let Ok(opened_file) = File::open(savedata_path.clone()) {
        println!("DEBUG file exists");
        let reader = BufReader::new(opened_file);
        if let Ok(content) = serde_json::from_reader(reader) {
            json = content
        };
    } else {
        println!("DEBUG file doesn't exist");
        create_dir_all(savedata_path.parent().unwrap()).unwrap();
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_savedata_path())?;

    let mut set = json[book_path.clone().into()]["edited_chapters"].as_array().unwrap_or(&vec![]).iter().map(|x| x.as_u64().unwrap() as usize).collect::<Vec<usize>>();
    println!("DEBUG set before push: {:?}", set);
    if edited {
        if !set.contains(&chapter) {
            set.push(chapter);
            println!("DEBUG set after push: {:?}", set);
        }
    }
    let value = json!(
        {
            "chapter":chapter,
            "page":page,
            "font_size":font_size.to_string(),
            "edited_chapters": set,
            "content":content.into()
        }
    );

    json[book_path.into()] = value;
    serde_json::to_writer_pretty(file, &json)?;

    Ok(())
}

pub fn remove_savedata_of_book<T: Into<String> + Clone>(
    book_path: T,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();

    let thread = std::thread::spawn(move || {
        let mut json = json!({});
        if let Ok(opened_file) = File::open(get_savedata_path()) {
            println!("DEBUG file exists");
            let reader = BufReader::new(opened_file);
            if let Ok(content) = serde_json::from_reader(reader) {
                json = content
            };
        }
        tx.send(json).unwrap();
    });

    if let Ok(()) = thread.join() {
        let mut json = rx.recv().unwrap();
        if json[book_path.clone().into()].is_object() {
            let file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(get_savedata_path())?;

            json.as_object_mut().unwrap().remove(&book_path.into());
            serde_json::to_writer_pretty(file, &json)?;
        }
        drop(rx);
        Ok(())
    } else {
        Err("Error while saving data".into())
    }
}

pub fn remove_all_savedata() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_savedata_path();
    if config_path.exists() {
        std::fs::remove_file(config_path)?
    }
    Ok(())
}

/// function to load the last read page of a chapter given the path of the book
pub fn load_data<T: Into<String> + Clone>(
    book_path: T,
) -> Result<(usize, usize, f64), Box<dyn std::error::Error>> {
    let mut chapter = 1;
    let mut page = 0;
    let mut font_size = FontSize::MEDIUM.to_f64();

    if let Ok(file) = File::open(get_savedata_path()) {
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader)?;

        if let Some(value) = json.get(book_path.clone().into()) {
            let saved_chapter_value = value
                .get("chapter").and_then(|v| v.as_u64());
            let saved_page_value = value
                .get("page").and_then(|v| v.as_u64());
            let saved_font_size = value
                .get("font_size").and_then(|v| v.as_str());

            if saved_chapter_value.is_some() && saved_page_value.is_some() && saved_font_size.is_some() {
                chapter = saved_chapter_value.unwrap() as usize;
                page = saved_page_value.unwrap() as usize;
                font_size = FontSize::from_string(saved_font_size.unwrap().to_string()).to_f64();
            } else {
                chapter = 1;
                page = 0;
                font_size = FontSize::MEDIUM.to_f64();
            }
        };
    }
    println!(
        "DEBUG reading data: {} {} {} {}",
        &chapter,
        &page,
        book_path.into(),
        font_size
    );
    Ok((chapter, page, font_size))
}

pub fn get_chapter(
    folder_name: &str,
    chapter: usize,
    extension: FileExtension,
) -> Result<String, String> {
    let slice = get_chapter_bytes(folder_name, chapter, extension)?;
    String::from_utf8(slice).map_err(|e| e.to_string())
}

pub fn get_chapter_bytes(
    folder_name: impl Into<String>,
    chapter: usize,
    extension: FileExtension,
) -> Result<Vec<u8>, String> {
    let ext = match extension {
        FileExtension::TXT => "txt",
        FileExtension::HTML => "html",
        FileExtension::EPUB => "epub",
    };

    let filename = get_saved_books_dir()
        .join(&folder_name.into())
        .join(format!("page_{}.{}", chapter, ext));
    println!("filename from where get page: {:?}", filename);

    std::fs::read(filename).map_err(|e| e.to_string())
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    //todo() use tempdir to create a temp dir and use it for tests

    // fn save_page_of_chapter
    #[test]
    #[ignore]
    fn save_create_file_and_write_correctly() {
        let book_path = "test_book";
        let chapter = 1;
        let page = 1;

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // assert that function returns Ok
        assert!(save_page_of_chapter(book_path, chapter, page).is_ok());

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        let file = File::open(CONFIG_PATH).unwrap();
        let reader = BufReader::new(file);

        // assert that file contains correct data
        let json: Value = serde_json::from_reader(reader).unwrap();
        assert_eq!(json, json!({book_path:{"chapter":chapter, "page":page}}));

        // delete file
        std::fs::remove_file(CONFIG_PATH).unwrap();

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    #[test]
    #[ignore]
    fn save_overwrite_file_and_write_correctly_the_same_book() {
        let book_path = "test_book";

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(CONFIG_PATH)
            .unwrap();

        let json_to_write = json!({book_path:{"chapter":0, "page":0}});
        serde_json::to_writer_pretty(file, &json_to_write).unwrap();

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        // overwrite file
        let chapter = 2;
        let page = 2;
        assert!(save_page_of_chapter(book_path, chapter, page).is_ok());

        let file = File::open(CONFIG_PATH).unwrap();
        let reader = BufReader::new(file);

        // assert that file contains correct data
        let json: Value = serde_json::from_reader(reader).unwrap();
        assert_eq!(json, json!({book_path:{"chapter":chapter, "page":page}}));

        // delete file
        std::fs::remove_file(CONFIG_PATH).unwrap();

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    #[test]
    #[ignore]
    fn save_a_book_different_from_one_already_in_file() {
        let book_path = "test_book";
        let chapter = 1;
        let page = 1;

        let book_path2 = "test_book2";
        let chapter2 = 2;
        let page2 = 2;

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // create file with one book
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(CONFIG_PATH)
            .unwrap();

        let json_to_write = json!({book_path:{"chapter":chapter, "page":page}});
        serde_json::to_writer_pretty(file, &json_to_write).unwrap();

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        // overwrite file
        assert!(save_page_of_chapter(book_path2, chapter2, page2).is_ok());

        let file = File::open(CONFIG_PATH).unwrap();
        let reader = BufReader::new(file);

        // assert that file contains correct data
        let json: Value = serde_json::from_reader(reader).unwrap();
        assert_eq!(
            json,
            json!({book_path:{"chapter":chapter, "page":page}, book_path2:{"chapter":chapter2, "page":page2}})
        );

        // delete file
        std::fs::remove_file(CONFIG_PATH).unwrap();

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    // fn remove_savedata_of_book
    #[test]
    #[ignore]
    fn remove_from_not_existing_file_book_s_savedata() {
        let book_path = "test_book";

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // assert that function returns Ok
        assert!(remove_savedata_of_book(book_path).is_ok());

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    #[test]
    #[ignore]
    fn remove_from_file_existing_book_s_savedata() {
        let book_path = "test_book";
        let chapter = 1;
        let page = 1;

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // create file with one book
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(CONFIG_PATH)
            .unwrap();

        let json_to_write = json!({book_path:{"chapter":chapter, "page":page}});
        serde_json::to_writer_pretty(file, &json_to_write).unwrap();

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        // remove book from file
        assert!(remove_savedata_of_book(book_path).is_ok());

        let file = File::open(CONFIG_PATH).unwrap();
        let reader = BufReader::new(file);

        // assert that file contains correct data
        let json: Value = serde_json::from_reader(reader).unwrap();
        assert!(json[book_path].is_null());

        // delete file
        std::fs::remove_file(CONFIG_PATH).unwrap();

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    #[test]
    #[ignore]
    fn remove_from_file_without_book_s_savedata() {
        let book_path = "test_book";

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // create file with one book
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(CONFIG_PATH)
            .unwrap();

        let json_to_write = json!({});
        serde_json::to_writer_pretty(file, &json_to_write).unwrap();

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        // remove book from file
        assert!(remove_savedata_of_book(book_path).is_ok());

        let file = File::open(CONFIG_PATH).unwrap();
        let reader = BufReader::new(file);

        // assert that file contains correct data
        let json: Value = serde_json::from_reader(reader).unwrap();
        assert!(json[book_path].is_null());

        // delete file
        std::fs::remove_file(CONFIG_PATH).unwrap();

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    // fn remove_all_savedata
    #[test]
    #[ignore]
    fn remove_all_savedata_from_not_existing_file() {
        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // assert that function returns Ok
        assert!(remove_all_savedata().is_ok());

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    #[test]
    #[ignore]
    fn remove_all_savedata_from_existing_file() {
        let book_path = "test_book";
        let chapter = 1;
        let page = 1;

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // create file with one book
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(CONFIG_PATH)
            .unwrap();

        let json_to_write = json!({book_path:{"chapter":chapter, "page":page}});
        serde_json::to_writer_pretty(file, &json_to_write).unwrap();

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        // remove all savedata
        assert!(remove_all_savedata().is_ok());

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    // fn get_page_of_chapter
    #[test]
    #[ignore]
    fn get_savedata_from_not_existing_file() {
        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // assert that function returns (1,0)
        assert_eq!(get_page_of_chapter("test_book").unwrap(), (1, 0));
    }

    #[test]
    #[ignore]
    fn get_savedata_from_existing_file_with_book_inside() {
        let book_path = "test_book";
        let chapter = 1;
        let page = 1;

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // create file with one book
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(CONFIG_PATH)
            .unwrap();

        let json_to_write = json!({book_path:{"chapter":chapter, "page":page}});
        serde_json::to_writer_pretty(file, &json_to_write).unwrap();

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        // assert that function returns (1,0)
        assert_eq!(get_page_of_chapter(book_path).unwrap(), (chapter, page));

        // delete file
        std::fs::remove_file(CONFIG_PATH).unwrap();

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    #[test]
    #[ignore]
    fn get_savedata_from_existing_file_without_book_inside() {
        let book_path = "test_book";

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // create file with one book
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(CONFIG_PATH)
            .unwrap();

        let json_to_write = json!({});
        serde_json::to_writer_pretty(file, &json_to_write).unwrap();

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        // assert that function returns (1,0)
        assert_eq!(get_page_of_chapter(book_path).unwrap(), (1, 0));

        // delete file
        std::fs::remove_file(CONFIG_PATH).unwrap();

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    #[test]
    #[ignore]
    fn get_savedata_from_existing_file_with_book_inside_with_wrong_data() {
        let book_path = "test_book";

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // create file with one book
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(CONFIG_PATH)
            .unwrap();

        let json_to_write = json!({book_path:{"chapter":"ciao", "page":"ciao"}});
        serde_json::to_writer_pretty(file, &json_to_write).unwrap();

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        // assert that function returns (1,0)
        assert_eq!(get_page_of_chapter(book_path).unwrap(), (1, 0));

        // delete file
        std::fs::remove_file(CONFIG_PATH).unwrap();

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    #[test]
    #[ignore]
    fn get_savedata_from_existing_file_with_book_inside_with_wrong_data2() {
        let book_path = "test_book";

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // create file with one book
        let file = std::fs::File::create(CONFIG_PATH).unwrap();

        let json_to_write = json!({book_path:{"chapter":"ciao", "page":1}});
        serde_json::to_writer_pretty(file, &json_to_write).unwrap();

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        // assert that function returns (1,0)
        assert_eq!(get_page_of_chapter(book_path).unwrap(), (1, 0));

        // delete file
        std::fs::remove_file(CONFIG_PATH).unwrap();

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }

    #[test]
    #[ignore]
    fn get_savedata_from_existing_file_with_book_inside_with_wrong_data3() {
        let book_path = "test_book";

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);

        // create file with one book
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(CONFIG_PATH)
            .unwrap();

        let json_to_write = json!({book_path:{"chapter":1, "page":"ciao"}});
        serde_json::to_writer_pretty(file, &json_to_write).unwrap();

        // assert that file exists
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), true);

        // assert that function returns (1,0)
        assert_eq!(get_page_of_chapter(book_path).unwrap(), (1, 0));

        // delete file
        std::fs::remove_file(CONFIG_PATH).unwrap();

        // assert that file doesn't exist
        assert_eq!(std::path::Path::new(CONFIG_PATH).exists(), false);
    }
}

*/
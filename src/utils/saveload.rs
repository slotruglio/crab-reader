use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
    sync::mpsc::channel, path::Path, str::FromStr,
};

use rust_fuzzy_search::fuzzy_compare;
use serde_json::{json, Value};

use crate::{MYENV, utils::{dir_manager::{get_savedata_path, get_saved_books_dir, get_edited_books_dir, get_epub_dir}, epub_utils::{get_metadata_of_book, split_chapter_in_vec}}};

use super::{envmanager::FontSize, dir_manager::get_metadata_path};

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
    // create a vec of edited chapters
    let mut set = json[book_path.clone().into()]["edited_chapters"].as_array().unwrap_or(&vec![]).iter().map(|x| x.as_u64().unwrap() as usize).collect::<Vec<usize>>();
    println!("DEBUG set before push: {:?}", set);
    if edited {
        if !set.contains(&chapter) {
            set.push(chapter);
            println!("DEBUG set after push: {:?}", set);
        }
    }
    // json value for the book
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

    // open file to write
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_savedata_path())?;

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

pub fn remove_edited_chapter<T: Into<String>+Clone>(book_path: T, chapter_number: usize) {
    let savedata_path = get_savedata_path();
    let mut json = json!({});
    if let Ok(opened_file) = File::open(savedata_path.clone()) {
        println!("DEBUG file exists");
        let reader = BufReader::new(opened_file);
        if let Ok(content) = serde_json::from_reader(reader) {
            json = content
        };
    }

    let mut set = json[book_path.clone().into()]["edited_chapters"]
        .as_array().unwrap_or(&vec![])
        .iter().map(|x| x.as_u64().unwrap() as usize).collect::<Vec<usize>>();
    
    
    if let Ok(index) = set.binary_search(&chapter_number) {
        set.remove(index);
    } else {
        return;
    }

    json[book_path.clone().into()]["edited_chapters"] = json!(set);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_savedata_path()).unwrap();

    serde_json::to_writer_pretty(file, &json).unwrap();
    let book: String = book_path.into();
    let folder_name = Path::new(&book).file_stem().unwrap().to_str().unwrap();
    let path = get_edited_books_dir().join(folder_name).join(format!("page_{}.txt", chapter_number));
    let _ = std::fs::remove_file(path);

}

fn evaluate_numeric_options(chapter: Option<u64>, page: Option<u64>) -> Option<(usize, usize)> {
    let chapter = chapter?;
    let page = page?;
    Some((chapter as usize, page as usize))
}
fn evaluate_str_options<'a>(font_size: Option<&str>, saved_content: Option<&'a str>) -> Option<(FontSize, &'a str)> {
    let font_size = font_size?;
    let saved_content = saved_content?;
    Some((FontSize::from(font_size.to_string()), saved_content))
}

/// function to get the most similar page of chapter to the last read one
fn search_page<T: Into<String> + Clone>(book_path: T, chapter_number: usize, text: &str) -> usize {
    let pages = split_chapter_in_vec(
        book_path.clone().into().as_str(),
        Option::None,
        chapter_number,
        8,
        MYENV.lock().unwrap().font.size,
        800.0,
        300.0,
    );

    let mut best_page = (0, 0.0);
    for (i, page) in pages.iter().enumerate() {
        let result = fuzzy_compare(
            text,
            page.as_str(),
        );
        if result > best_page.1 {
            best_page = (i, result);
        }
    }
    best_page.0
}

pub fn save_favorite<T: Into<String> + Clone>(book_path: T, favorite: bool) -> Result<(), Box<dyn std::error::Error>> {

    let mut metadata = get_metadata_of_book(book_path.clone().into().as_str());
    let old_string = &metadata["favorite"];
    if (old_string == "true" && favorite) || (old_string == "false" && !favorite) {
        return Ok(());
    }

    metadata.insert("favorite".to_string(), if favorite { "true" } else { "false" }.to_string());

    let json = json!(metadata);
    let metadata_path = get_metadata_path(&book_path.into());

    let metadata_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(metadata_path)
        .unwrap();

    serde_json::to_writer_pretty(metadata_file, &json)?;
    return Ok(());
}

/// function to load the last read page of a chapter given the path of the book
pub fn load_data<T: Into<String> + Clone>(
    book_path: T,
    force_fuzzy: bool,
) -> Result<(usize, usize, f64), Box<dyn std::error::Error>> {
    let mut chapter = 1;
    let mut page = 0;
    let mut font_size = FontSize::MEDIUM;
    let mut content = "";

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
            let saved_content = value
                .get("content").and_then(|v| v.as_str());

            // assign values considering the same font size
            if let Some((c, s)) = evaluate_numeric_options(saved_chapter_value, saved_page_value) {
                chapter = c;
                page = s;
            }
            if let Some((f, c)) = evaluate_str_options(saved_font_size, saved_content){
                font_size = f;
                content = c;
            }

            let app_font_size = FontSize::from(MYENV.lock().unwrap().font.size);
            // check if the application font size (env) is different from the saved one
            if (app_font_size != font_size) || force_fuzzy {
                println!("DEBUG font size is different");
                // need to "find" the last read page
                page = search_page(book_path.clone(), chapter, content);
            }
        };
    }
    println!(
        "DEBUG reading data: {} {} {} {}",
        &chapter,
        &page,
        book_path.into(),
        font_size.to_f64()
    );
    Ok((chapter, page, font_size.to_f64()))
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
    let (path, ext) = match extension {
        FileExtension::TXT => (get_edited_books_dir(),"txt"),
        FileExtension::HTML => (get_saved_books_dir(),"html"),
        FileExtension::EPUB => (get_epub_dir(),"epub"),
    };

    let filename = path
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
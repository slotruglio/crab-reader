use std::{
    collections::HashMap,
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
    path::Path,
    str::FromStr,
    sync::mpsc::channel,
};

use druid::im::Vector;
use rust_fuzzy_search::fuzzy_compare;
use serde_json::{json, Value};

use crate::{
    models::{note::Note, book::{PAGE_WIDTH, PAGE_HEIGHT}},
    utils::{
        dir_manager::{
            get_books_notes_path, get_edited_books_dir, get_epub_dir, get_saved_books_dir,
            get_savedata_path,
        },
        epub_utils::{get_metadata_of_book, split_chapter_in_vec},
    },
    MYENV,
};

use super::{dir_manager::get_metadata_path, envmanager::FontSize};

pub enum FileExtension {
    TXT,
    HTML,
    EPUB,
}

/// function to save page of chapter of currently opened book
pub fn save_data<T: Into<String> + Clone>(
    book_path: T,
    chapter: usize,
    page: usize,
    content: T,
    font_size: FontSize,
    edited: bool,
) -> Result<(), Box<dyn std::error::Error>> {

    // check if exists a savedata file
    let savedata_path = get_savedata_path();
    let mut json = json!({});
    if let Ok(opened_file) = File::open(savedata_path.clone()) {
        let reader = BufReader::new(opened_file);
        if let Ok(content) = serde_json::from_reader(reader) {
            json = content
        };
    } else {
        create_dir_all(savedata_path.parent().unwrap()).unwrap();
    }
    // create a vec of edited chapters
    let mut set = json[book_path.clone().into()]["edited_chapters"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|x| x.as_u64().unwrap() as usize)
        .collect::<Vec<usize>>();
    if edited {
        if !set.contains(&chapter) {
            set.push(chapter);
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
            // file exists
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

pub fn remove_edited_chapter<T: Into<String> + Clone>(book_path: T, chapter_number: usize) {
    let savedata_path = get_savedata_path();
    let mut json = json!({});
    if let Ok(opened_file) = File::open(savedata_path.clone()) {
        // file exists
        let reader = BufReader::new(opened_file);
        if let Ok(content) = serde_json::from_reader(reader) {
            json = content
        };
    }

    let mut set = json[book_path.clone().into()]["edited_chapters"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|x| x.as_u64().unwrap() as usize)
        .collect::<Vec<usize>>();

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
        .open(get_savedata_path())
        .unwrap();

    serde_json::to_writer_pretty(file, &json).unwrap();
    let book: String = book_path.into();
    let folder_name = Path::new(&book).file_stem().unwrap().to_str().unwrap();
    let path = get_edited_books_dir()
        .join(folder_name)
        .join(format!("page_{}.txt", chapter_number));
    let _ = std::fs::remove_file(path);
}

fn evaluate_numeric_options(chapter: Option<u64>, page: Option<u64>) -> Option<(usize, usize)> {
    let chapter = chapter?;
    let page = page?;
    Some((chapter as usize, page as usize))
}
fn evaluate_str_options<'a>(
    font_size: Option<&str>,
    saved_content: Option<&'a str>,
) -> Option<(FontSize, &'a str)> {
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
        PAGE_WIDTH,
        PAGE_HEIGHT,
    );

    let mut best_page = (0, 0.0);
    for (i, page) in pages.iter().enumerate() {
        let result = fuzzy_compare(text, page.as_str());
        if result > best_page.1 {
            best_page = (i, result);
        }
    }
    best_page.0
}

pub fn save_favorite<T: Into<String> + Clone>(
    book_path: T,
    favorite: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut metadata = get_metadata_of_book(book_path.clone().into().as_str());
    let old_string = &metadata["favorite"];
    if (old_string == "true" && favorite) || (old_string == "false" && !favorite) {
        return Ok(());
    }

    metadata.insert(
        "favorite".to_string(),
        if favorite { "true" } else { "false" }.to_string(),
    );

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
            let saved_chapter_value = value.get("chapter").and_then(|v| v.as_u64());
            let saved_page_value = value.get("page").and_then(|v| v.as_u64());
            let saved_font_size = value.get("font_size").and_then(|v| v.as_str());
            let saved_content = value.get("content").and_then(|v| v.as_str());

            // assign values considering the same font size
            if let Some((c, s)) = evaluate_numeric_options(saved_chapter_value, saved_page_value) {
                chapter = c;
                page = s;
            }
            if let Some((f, c)) = evaluate_str_options(saved_font_size, saved_content) {
                font_size = f;
                content = c;
            }

            let app_font_size = FontSize::from(MYENV.lock().unwrap().font.size);
            // check if the application font size (env) is different from the saved one
            if (app_font_size != font_size) || force_fuzzy {
                // need to "find" the last read page
                page = search_page(book_path.clone(), chapter, content);
            }
        };
    }
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
        FileExtension::TXT => (get_edited_books_dir(), "txt"),
        FileExtension::HTML => (get_saved_books_dir(), "html"),
        FileExtension::EPUB => (get_epub_dir(), "epub"),
    };

    let filename = path
        .join(&folder_name.into())
        .join(format!("page_{}.{}", chapter, ext));

    std::fs::read(filename).map_err(|e| e.to_string())
}

/// function to save note in a page of chapter of currently opened book
pub fn save_note<T: Into<String> + Clone>(
    book_path: T,
    chapter: usize,
    page_text: T,
    note: T,
) -> Result<String, Box<dyn std::error::Error>> {
    // check if exists a file
    let notes_path = get_books_notes_path();
    let mut json = json!({});
    if let Ok(opened_file) = File::open(notes_path.clone()) {
        // file exists
        let reader = BufReader::new(opened_file);
        if let Ok(content) = serde_json::from_reader(reader) {
            json = content
        };
    } else {
        // file doesn't exist
        create_dir_all(notes_path.parent().unwrap()).unwrap();
    }

    let text = page_text.into();
    let to_take = if text.len() > 200 {
        text.len() / 3
    } else {
        text.len()
    };
    // json value for the note to save
    let value = json!(
        {
            "start": &text[..to_take],
            "note":note.into()
        }
    );

    // array of the book
    if let Some(array) = json[book_path.clone().into()].as_array_mut() {
        // other stuff
        if let Some(obj) = array.iter_mut().find(|obj| obj["chapter"] == chapter) {
            // chapter already exists
            if let Some(array_notes) = obj["notes"].as_array_mut() {
                // add the note to the array
                array_notes.push(value);
            } else {
                // create the array
                obj["notes"] = json!([value]);
            }
        } else {
            // chapter doesn't exist
            array.push(json!({"chapter":chapter, "notes":[value]}));
        }
    } else {
        json[book_path.into()] = json!([
            {"chapter":chapter,
            "notes":[value]
        }]);
    }
    // open file to write
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_books_notes_path())?;

    serde_json::to_writer_pretty(file, &json)?;

    Ok(text[..to_take].to_string())
}

/// function to load notes of a book
pub fn load_notes<T: Into<String> + Clone>(
    book_path: T,
) -> Result<HashMap<(usize, usize), Vector<Note>>, Box<dyn std::error::Error>> {
    let mut map: HashMap<(usize, usize), Vector<Note>> = HashMap::new();

    if let Ok(file) = File::open(get_books_notes_path()) {
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader)?;

        if let Some(book_array) = json[book_path.clone().into()].as_array() {
            // exists a book with that name
            for chapter in book_array {
                let chapter_number = chapter["chapter"].as_u64().unwrap() as usize;
                if let Some(notes_array) = chapter["notes"].as_array() {
                    for note in notes_array {
                        let start_page = note["start"].as_str().unwrap();
                        let note_text = note["note"].as_str().unwrap().to_string();
                        let page =
                            search_page(book_path.clone().into(), chapter_number, start_page);
                        map.entry((chapter_number, page))
                            .and_modify(|v| {
                                v.push_back(Note::new(start_page.into(), note_text.clone()))
                            })
                            .or_insert(Vector::from(vec![Note::new(start_page.into(), note_text)]));
                    }
                }
            }
        }
    }
    Ok(map)
}

/// function to delete a note of a book
pub fn delete_note<T: Into<String> + Clone>(
    book_path: T,
    chapter: usize,
    start_page: T,
) -> Result<(), Box<dyn std::error::Error>> {
    // open file to read
    let Ok(file) = File::open(get_books_notes_path()) else {
        return Ok(());
    };

    let reader = BufReader::new(file);
    let mut json: Value = serde_json::from_reader(reader)?;

    // check if there is a book with that name and an array
    let Some(book_array) = json[book_path.clone().into()].as_array_mut() else {
        return Ok(());
    };

    let mut index_to_delete = None;

    for chapter_value in book_array {
        let chapter_number = chapter_value["chapter"].as_u64().unwrap() as usize;
        if chapter_number == chapter {
            // chapter found
            let Some(notes_array) = chapter_value["notes"].as_array_mut() else {
                return Ok(());
            };

            for (i, note) in notes_array.into_iter().enumerate() {
                let saved_start_page = note["start"].as_str().unwrap();
                if saved_start_page == start_page.clone().into() {
                    // note found
                    index_to_delete = Some(i);
                }
            }

            if let Some(index) = index_to_delete {
                notes_array.remove(index);
            }
        }
    }

    // open file to write
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_books_notes_path())?;

    serde_json::to_writer_pretty(file, &json)?;

    Ok(())
}

/// delete notes of book_path given chapter, and vec of start_page (string)
pub fn delete_notes<T: Into<String> + Clone>(
    book_path: T,
    chapter: usize,
    start_pages: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let Ok(file) = File::open(get_books_notes_path()) else {
        return Ok(());
    };

    let reader = BufReader::new(file);
    let mut json: Value = serde_json::from_reader(reader)?;

    // check if there is a book with that name and an array
    let Some(book_array) = json[book_path.clone().into()].as_array_mut() else {
        return Ok(());
    };

    for item in book_array {
        let chapter_number = item["chapter"].as_u64().unwrap() as usize;

        if chapter_number != chapter {
            continue;
        }

        let Some(notes_array) = item["notes"].as_array_mut() else {
            return Ok(());
        };

        notes_array.retain(|note| {
            !start_pages
                .iter()
                .any(|start| note["start"].as_str().unwrap() == start)
        });
    }

    // open file to write
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_books_notes_path())?;

    serde_json::to_writer_pretty(file, &json)?;

    Ok(())
}

/// function to delete all notes of a book
pub fn delete_all_notes<T: Into<String> + Clone>(
    book_path: T,
) -> Result<(), Box<dyn std::error::Error>> {
    // open file to read
    let Ok(file) = File::open(get_books_notes_path()) else {
        return Ok(());
    };

    let reader = BufReader::new(file);
    let mut json: Value = serde_json::from_reader(reader)?;

    // check if there is a book with that name and an array
    let Some(book_array) = json[book_path.clone().into()].as_array_mut() else {
        return Ok(());
    };

    book_array.clear();

    // open file to write
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_books_notes_path())?;

    serde_json::to_writer_pretty(file, &json)?;

    Ok(())
}


pub fn delete_book(book_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let epub = Path::new(book_path);
    // delete book from file
    if epub.exists() {
        // remove from saved_books
        let saved_book = get_saved_books_dir().join(epub.file_stem().unwrap());
        std::fs::remove_dir_all(saved_book)?;

        // remove from epubs dir
        std::fs::remove_file(book_path)?;
    }

    Ok(())
}

pub fn copy_book_in_folder(from: &String) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(from);
    if !path.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        )));
    }

    let to = get_epub_dir().join(path.file_name().unwrap());
    std::fs::copy(from, to)?;
    Ok(())
}


// Tests are provided only for the functions that are really used
#[cfg(test)]
mod tests {
    use std::{path::PathBuf, fmt::format};

    use super::*;
    use serde_json::{json, Value};

    fn delete_savedata() {
        // delete existing file
        let _ = std::fs::remove_file(get_savedata_path());

        // assert that file doesn't exist
        assert_eq!(get_savedata_path().exists(), false);
    }
    /// if exists, copy file at path
    fn copy_existing_file(path: PathBuf) {
        if !path.exists() {return}

        let _ = std::fs::copy(
            &path, 
            path.with_file_name("tmp_copy.json")
        );
    }
    /// if exists, restore file at path
    fn restore_existing_file(path: PathBuf) {
        if !path.with_file_name("tmp_copy.json").exists() {return}
        let _ = std::fs::copy(
            path.with_file_name("tmp_copy.json"),
            path
        );
    }

    fn create_file_for_bytes(ext: FileExtension) -> (String, usize, Vec<u8>) {
        let folder_name = "test_bytes";
        let chapter = 1;
        let content: Vec<u8> = "this is a test".into();
        let (path, ext_str) = match ext {
            FileExtension::TXT => (get_edited_books_dir(), "txt"),
            _ => (get_saved_books_dir(), "html"),
        };

        // create dir for file
        let _ = std::fs::create_dir_all(path.join(folder_name));
        // create fake file
        let file = std::fs::write(
            path.join(format!("test_bytes/page_1.{}", ext_str)), 
            &content
        );

        assert!(file.is_ok());

        (folder_name.to_string(), chapter, content)
    }

    // fn save_data
    #[test]
    #[ignore]
    fn save_create_file_and_write_correctly() {
        let book_path = "test_book";
        let chapter = 1;
        let page = 1;
        let content = "inventa gli ordigni fuori del suo corpo";

        copy_existing_file(get_savedata_path());

        delete_savedata();

        // assert that function returns Ok
        assert!(save_data(book_path, chapter, page, content, FontSize::MEDIUM, false).is_ok());

        // assert that file exists
        assert_eq!(get_savedata_path().exists(), true);

        let file = File::open(get_savedata_path()).unwrap();
        let reader = BufReader::new(file);

        // assert that file contains correct data
        let json: Value = serde_json::from_reader(reader).unwrap();
        assert_eq!(json, json!({
            book_path:{
                "chapter":chapter, "page":page, "content":content,
                "font_size": "medium", "edited_chapters": []
            }
        }));

        delete_savedata();
        restore_existing_file(get_savedata_path());
    }

    #[test]
    #[ignore]
    fn save_overwrite_existing_file() {
        let book_path = "test_book";
        let chapter = 1;
        let page = 1;
        let content = "inventa gli ordigni fuori del suo corpo";
        
        copy_existing_file(get_savedata_path());
        delete_savedata();

        // assert that function returns Ok
        assert!(save_data(book_path, chapter, page, content, FontSize::MEDIUM, false).is_ok());

        // assert that file exists
        assert_eq!(get_savedata_path().exists(), true);

        // save new data
        let new_content = "li usa per distruggere il mondo";
        assert!(save_data(book_path, chapter, page+1, new_content, FontSize::MEDIUM, false).is_ok());

        // assert that file contains new data
        let file = File::open(get_savedata_path()).unwrap();
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader).unwrap();
        assert_eq!(json, json!({
            book_path:{
                "chapter":chapter, "page":page+1, "content":new_content,
                "font_size": "medium", "edited_chapters": []
            }
        }));

        delete_savedata();
        restore_existing_file(get_savedata_path());
    }

    #[test]
    #[ignore]
    fn save_another_book_existing_file() {
        let book_path = "test_book";
        let chapter = 1;
        let page = 1;
        let content = "inventa gli ordigni fuori del suo corpo";

        copy_existing_file(get_savedata_path());
        delete_savedata();

        // assert that function returns Ok
        assert!(save_data(book_path, chapter, page, content, FontSize::MEDIUM, false).is_ok());

        // assert that file exists
        assert_eq!(get_savedata_path().exists(), true);

        let new_book_path = "test_book_123";
        let new_chapter = 1;
        let new_page = 1;
        let new_content = "inventa gli ordigni fuori del suo corpo";
        assert!(save_data(new_book_path, new_chapter, new_page, new_content, FontSize::MEDIUM, false).is_ok());

        // assert that file contains new data
        let file = File::open(get_savedata_path()).unwrap();
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader).unwrap();
        assert_eq!(json, json!({
            book_path:{
                "chapter":chapter, "page":page, "content":new_content,
                "font_size": "medium", "edited_chapters": []
            },
            new_book_path:{
                "chapter":new_chapter, "page":new_page, "content":new_content,
                "font_size": "medium", "edited_chapters": []
            }
        }));

        delete_savedata();
        restore_existing_file(get_savedata_path());
    }

    #[test]
    #[ignore]
    fn save_book_with_edited_chapter() {
        let book_path = "test_book";
        let chapter = 1;
        let page = 1;
        let content = "inventa gli ordigni fuori del suo corpo";

        copy_existing_file(get_savedata_path());
        delete_savedata();

        // assert that function returns Ok
        assert!(save_data(book_path, chapter, page, content, FontSize::MEDIUM, true).is_ok());

        // assert that file exists
        assert_eq!(get_savedata_path().exists(), true);

        // assert that file contains correct data
        // assert that file contains new data
        let file = File::open(get_savedata_path()).unwrap();
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader).unwrap();
        assert_eq!(json, json!({
            book_path:{
                "chapter":chapter, "page":page, "content":content,
                "font_size": "medium", "edited_chapters": [chapter]
            }
        }));

        delete_savedata();
        restore_existing_file(get_savedata_path());
    }

    // fn load_data
    #[test]
    #[ignore]
    fn load_data_from_existing_file() {
        let book_path = "test_book";
        let chapter = 1;
        let page = 1;
        let content = "inventa gli ordigni fuori del suo corpo";
        let font_size = FontSize::MEDIUM;

        copy_existing_file(get_savedata_path());
        delete_savedata();

        // assert that function returns Ok
        assert!(save_data(book_path, chapter, page, content, font_size, false).is_ok());

        // assert that file exists
        assert_eq!(get_savedata_path().exists(), true);

        // assert that function returns Ok
        assert!(load_data(book_path, false).is_ok());

        // assert that function returns correct data
        let (chapter, page, size) = load_data(book_path, false).unwrap();
        assert_eq!(chapter, chapter);
        assert_eq!(page, page);
        assert_eq!(size, FontSize::MEDIUM.to_f64());

        delete_savedata();
        restore_existing_file(get_savedata_path());
    }

    #[test]
    #[ignore]
    fn load_data_from_non_existing_file() {
        let book_path = "test_book";

        copy_existing_file(get_savedata_path());
        delete_savedata();

        // assert that function returns Ok
        assert!(load_data(book_path, false).is_ok());

        // assert that function returns correct data
        let (chapter, page, size) = load_data(book_path, false).unwrap();
        assert_eq!(chapter, 1);
        assert_eq!(page, 0);
        assert_eq!(size, FontSize::MEDIUM.to_f64());

        delete_savedata();
        restore_existing_file(get_savedata_path());
    }

    #[test]
    #[ignore]
    fn load_data_with_diff_font_size() {
        let binding = get_epub_dir().join("svevo_la_coscienza_di_zeno.epub");
        // TO RUN THIS TEST YOU NEED TO HAVE THE EPUB IN THE EPUB DIRECTORY
        assert!(binding.exists());

        let book_path = binding.to_str().unwrap();

        let json = json!({
            book_path: {
                "chapter": 5,
                "content": "Ricordo di aver fumato molto, celato in tutti i luoghi possibili. Perché seguito da un forte disgusto fisico, ricordo un soggiorno prolungato per una mezz’ora in una cantina oscura insieme a due altri fanciulli di cui non ritrovo nella memoria altro che la puerilità del vestito: Due paia di calzoncini che stanno in piedi perché dentro c’è stato un corpo che il tempo eliminò. Avevamo molte sigarette e volevamo vedere chi ne sapesse bruciare di più nel breve tempo. Io vinsi, ed eroicamente celai il malessere che mi derivò dallo strano esercizio. Poi uscimmo al sole e all’aria. Dovetti chiudere gli occhi per non cadere stordito. Mi rimisi e mi vantai della vittoria. Uno dei due piccoli omini mi disse allora:\n\n\\- A me non importa di aver perduto perché io non fumo che quanto m’occorre. \n\nRicordo la parola sana e non la faccina certamente sana anch’essa che a me doveva essere rivolta in quel momento. \n\nMa allora io non sapevo se amavo o odiavo la sigaretta e il suo sapore e lo stato in cui la nicotina mi metteva. Quando seppi di odiare tutto ciò fu peggio. E lo seppi a vent’anni circa. Allora soffersi per qualche settimana di un violento male di gola accompagnato da febbre. Il dottore prescrisse il letto e l’assoluta astensione dal fumo. Ricordo questa parola *assoluta*! Mi ferì e la febbre la colorì: Un vuoto grande e niente per resistere all’enorme pressione che subito si produce attorno ad un vuoto. \n\nQuando il dottore mi lasciò, mio padre (mia madre era morta da molti anni) con tanto di sigaro in bocca restò ancora per qualche tempo a farmi compagnia. Andandosene, dopo di aver passata dolcemente la sua mano sulla mia fronte scottante, mi disse: \n\n\\- Non fumare, veh! \n\n",
                "edited_chapters": [],
                "font_size": "medium",
                "page": 3
            }
        });

        copy_existing_file(get_savedata_path());
        delete_savedata();

        // write new json file
        let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_savedata_path()).unwrap();

        serde_json::to_writer_pretty(file, &json).unwrap();

        // get font size from env
        let mut my_env = MYENV.lock().unwrap();
        let old_font = my_env.font.size;
        // set different font size to test
        let new_font = FontSize::SMALL;
        my_env.font.size = new_font.to_f64();
        my_env.save_to_env();

        drop(my_env);

        // assert that function returns Ok
        assert!(load_data(book_path, false).is_ok());

        // assert that function returns correct data
        let (chapter, page, _) = load_data(book_path, false).unwrap();
        assert_eq!(chapter, 5);
        assert_eq!(page, 2);

        // reset font size
        let mut my_env = MYENV.lock().unwrap(); 
        my_env.font.size = old_font;
        my_env.save_to_env();
        drop(my_env);

        delete_savedata();
        restore_existing_file(get_savedata_path());
    }

    // save_favorite
    #[test]
    #[ignore]
    fn save_book_as_favorite_when_not() {
        let json = json!({"favorite": "false",});
        let book = get_epub_dir().join("test_book.epub");
        let book_string = book.to_str().unwrap().to_string();
        // create dir for file
        let _ = std::fs::create_dir_all(get_saved_books_dir().join("test_book"));
        // create fake file
        let metadata_path = get_metadata_path(&book_string);
        let metadata_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(metadata_path)
            .unwrap();

        let _ = serde_json::to_writer_pretty(metadata_file, &json);

        assert_eq!(get_metadata_path(&book_string).exists(), true);

        assert!(save_favorite(&book_string, true).is_ok());

        let file = File::open(get_metadata_path(&book_string)).unwrap();
        let reader = BufReader::new(file);

        // assert that file contains correct data
        let saved: Value = serde_json::from_reader(reader).unwrap();
        assert_eq!(saved, json!({"favorite": "true",}));

        let _ = std::fs::remove_dir_all(get_saved_books_dir().join("test_book"));
        assert_eq!(get_saved_books_dir().join("test_book").exists(), false);
    }

    #[test]
    #[ignore]
    fn save_book_as_not_favorite_when_true() {
        let json = json!({"favorite": "true",});
        let book = get_epub_dir().join("test_book.epub");
        let book_string = book.to_str().unwrap().to_string();
        // create dir for file
        let _ = std::fs::create_dir_all(get_saved_books_dir().join("test_book"));
        // create fake file
        let metadata_path = get_metadata_path(&book_string);
        let metadata_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(metadata_path)
            .unwrap();

        let _ = serde_json::to_writer_pretty(metadata_file, &json);

        assert_eq!(get_metadata_path(&book_string).exists(), true);

        assert!(save_favorite(&book_string, false).is_ok());

        let file = File::open(get_metadata_path(&book_string)).unwrap();
        let reader = BufReader::new(file);

        // assert that file contains correct data
        let saved: Value = serde_json::from_reader(reader).unwrap();
        assert_eq!(saved, json!({"favorite": "false",}));

        let _ = std::fs::remove_dir_all(get_saved_books_dir().join("test_book"));
        assert_eq!(get_saved_books_dir().join("test_book").exists(), false);
    }
    
    #[test]
    #[ignore]
    fn save_book_as_fav_when_true() {
        let json = json!({"favorite": "true",});
        let book = get_epub_dir().join("test_book.epub");
        let book_string = book.to_str().unwrap().to_string();
        // create dir for file
        let _ = std::fs::create_dir_all(get_saved_books_dir().join("test_book"));
        // create fake file
        let metadata_path = get_metadata_path(&book_string);
        let metadata_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(metadata_path)
            .unwrap();

        let _ = serde_json::to_writer_pretty(metadata_file, &json);

        assert_eq!(get_metadata_path(&book_string).exists(), true);

        assert!(save_favorite(&book_string, true).is_ok());

        let file = File::open(get_metadata_path(&book_string)).unwrap();
        let reader = BufReader::new(file);

        // assert that file contains correct data
        let saved: Value = serde_json::from_reader(reader).unwrap();
        assert_eq!(saved, json!({"favorite": "true",}));

        let _ = std::fs::remove_dir_all(get_saved_books_dir().join("test_book"));
        assert_eq!(get_saved_books_dir().join("test_book").exists(), false);
    }

    // get_chapter_bytes
    #[test]
    #[ignore]
    fn get_chapter_bytes_when_txt() {
        let (folder_name, chapter, content) = create_file_for_bytes(FileExtension::TXT);
        
        assert_eq!(get_chapter_bytes(&folder_name, chapter, FileExtension::TXT).unwrap(), content);

        let _ = std::fs::remove_dir_all(get_edited_books_dir().join(&folder_name));

        assert_eq!(get_edited_books_dir().join(folder_name).exists(), false);
    }

    #[test]
    #[ignore]
    fn get_chapter_bytes_when_html() {
        let (folder_name, chapter, content) = create_file_for_bytes(FileExtension::HTML);
        
        assert_eq!(get_chapter_bytes(&folder_name, chapter, FileExtension::HTML).unwrap(), content);

        let _ = std::fs::remove_dir_all(get_saved_books_dir().join(&folder_name));

        assert_eq!(get_saved_books_dir().join(folder_name).exists(), false);
    }

    #[test]
    #[ignore]
    fn get_chapter_bytes_when_epub() {
        let folder_name = "test_bytes";
        let chapter = 1;

        let (_, _, _) = create_file_for_bytes(FileExtension::TXT);
        let (_, _, _) = create_file_for_bytes(FileExtension::HTML);

        assert!(get_chapter_bytes(folder_name, chapter, FileExtension::EPUB).is_err());

        let _ = std::fs::remove_dir_all(get_edited_books_dir().join(folder_name));
        assert_eq!(get_edited_books_dir().join(folder_name).exists(), false);

        let _ = std::fs::remove_dir_all(get_saved_books_dir().join(folder_name));
        assert_eq!(get_saved_books_dir().join(folder_name).exists(), false);

    }

    #[test]
    #[ignore]
    fn get_chapter_bytes_when_no_page() {
        let folder_name = "test_bytes";
        let chapter = 1;

        // create dir for file
        let _ = std::fs::create_dir_all(get_saved_books_dir().join(folder_name));

        assert!(get_chapter_bytes(folder_name, chapter, FileExtension::HTML).is_err());

        let _ = std::fs::remove_dir_all(get_saved_books_dir().join(folder_name));
        assert_eq!(get_saved_books_dir().join(folder_name).exists(), false);

    }
    
    // save_note
    #[test]
    #[ignore]
    fn save_create_note() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn save_overwrite_note() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn save_other_book_note() {
        unimplemented!();
    }

    // delete_notes
    #[test]
    #[ignore]
    fn delete_notes_when_existing() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn delete_notes_when_not_existing() {
        unimplemented!();
    }

    // load_notes
    #[test]
    #[ignore]
    fn get_notes_when_existing() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn get_notes_when_not_existing() {
        unimplemented!();
    }

    // delete_book
    #[test]
    #[ignore]
    fn delete_existing_book() {
        let path = get_epub_dir().join("test.epub").to_str().unwrap().to_string();
        let epub = Path::new(&path);
        
        let saved_book = get_saved_books_dir().join(epub.file_stem().unwrap());
        assert!(std::fs::create_dir_all(&saved_book).is_ok());
        assert!(File::create(&path).is_ok());
    
        assert!(delete_book(&path).is_ok());

        assert!(!epub.exists());
        assert!(!saved_book.exists());

    }
    #[test]
    #[ignore]
    fn delete_not_existing_book() {
        let path = "not_existing_book.epub".to_string();

        let old_epub_content = std::fs::read_dir(get_epub_dir()).unwrap().count();

        assert!(delete_book(&path).is_ok());

        assert_eq!(old_epub_content, std::fs::read_dir(get_epub_dir()).unwrap().count());
    }

    
}
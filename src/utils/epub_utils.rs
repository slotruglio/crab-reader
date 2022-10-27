use super::saveload::{get_chapter, FileExtension};
use epub::doc::EpubDoc;
use serde_json::json;
use std::{
    collections::HashMap,
    error,
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    path::Path,
    rc::Rc,
};
/// Method to extract metadata from epub file
/// and returns explicit metadata.
/// title: title of the book
/// author: author of the book
/// lang: language of the book
/// chapters: number of chapters in the book as String
/// source: source of the book
/// date: date of the book
/// rights: rights of the book
/// identifier: identifier of the book

const SAVED_BOOKS_PATH: &str = "saved_books/";

#[allow(dead_code)]
const SAVED_BOOKS_COVERS_PATH: &str = "saved_books/covers/";

fn get_metadata_from_epub(
    book: &EpubDoc<File>,
) -> Result<HashMap<String, String>, Box<dyn error::Error>> {
    for key in book.metadata.keys() {
        println!("DEBUG: {}: {}", key, book.mdata(key).unwrap());
    }
    let mut metadata = HashMap::new();
    metadata.insert(
        "title".to_string(),
        book.mdata("title").unwrap_or("no title".to_string()),
    );

    metadata.insert(
        "author".to_string(),
        book.mdata("creator").unwrap_or("no author".to_string()),
    );

    metadata.insert(
        "lang".to_string(),
        book.mdata("language").unwrap_or("no lang".to_string()),
    );
    /*
    metadata.insert(
        "cover".to_string(),
        String::from_utf8(book.get_cover().unwrap_or(Vec::<u8>::default()))?,
    );
    */

    metadata.insert(
        "source".to_string(),
        book.mdata("source").unwrap_or("no source".to_string()),
    );

    metadata.insert(
        "date".to_string(),
        book.mdata("date").unwrap_or("no date".to_string()),
    );

    metadata.insert(
        "rights".to_string(),
        book.mdata("rights").unwrap_or("no rights".to_string()),
    );

    metadata.insert(
        "identifier".to_string(),
        book.mdata("identifier")
            .unwrap_or("no indetifier".to_string()),
    );

    metadata.insert("chapters".to_string(), book.get_num_pages().to_string());

    Ok(metadata)
}

/// Method to save the cover of the book as a png file
/// in the path specified.
/// image: String of vec[u8] (as u8) of the cover
/// name: name of the file
/// path: path where to save the cover
/// returns: Result with the path of the cover saved
#[allow(dead_code)]
pub fn save_book_cover(image: &Vec<u8>, name: &String) -> Result<String, Box<dyn error::Error>> {
    // create dir
    std::fs::create_dir_all(SAVED_BOOKS_COVERS_PATH)?;

    let path = format!("{}{}.png", SAVED_BOOKS_COVERS_PATH, &name);
    let mut file = File::create(&path)?;
    file.write_all(image)?;

    Ok(path)
}

pub fn edit_chapter(
    path: &str,
    chapter_number: usize,
    text: impl Into<String>,
) -> Result<(), Box<dyn error::Error>> {
    let book_name = path.split("/").last().unwrap().split(".").next().unwrap();

    let mut saved_book_chapter_path = format!("{}{}", SAVED_BOOKS_PATH, book_name);

    std::fs::create_dir_all(&saved_book_chapter_path)?;

    saved_book_chapter_path.push_str(format!("/page_{}.txt", chapter_number).as_str());
    println!("DEBUG: path to get chapter: {}", saved_book_chapter_path);

    let mut page_html = OpenOptions::new()
        .write(true)
        .create(true)
        .open(saved_book_chapter_path)?;

    let text = String::from(text.into());

    page_html.write_all(text.as_bytes())?;
    Ok(())
}

pub fn extract_all(path: &str) -> Result<(), Box<dyn error::Error>> {
    let file_name = path.split("/").last().unwrap();
    let folder_name = file_name.split(".").next().unwrap();
    let path_name = format!("{}{}", SAVED_BOOKS_PATH, folder_name);
    println!("DEBUG: Folder path: {}", path_name);
    std::fs::create_dir_all(&path_name)?;
    let mut book = EpubDoc::new(path)?;

    let mut metadata_file = File::create(format!("{}/metadata.json", path_name)).unwrap();
    let metadata_map = get_metadata_from_epub(&book)?;

    let json = json!(metadata_map);
    //json["cover"] = json!(book.get_cover().unwrap_or_default());
    metadata_file
        .write_all(json.to_string().as_bytes())
        .unwrap();
    let len = book.get_num_pages();

    //extract all chapters
    let mut i = 0;
    while i < len {
        let chapter = book.get_current_str().unwrap();
        let mut file = File::create(format!("{}/page_{}.html", &path_name, i)).unwrap();
        file.write_all(chapter.as_bytes()).unwrap();
        if let Err(_) = book.go_next() {
            break;
        }
        i += 1;
    }

    Ok(())
}

pub fn extract_metadata(path: &str) -> Result<HashMap<String, String>, Box<dyn error::Error>> {
    let file_name = path.split("/").last().unwrap();
    let folder_name = file_name.split(".").next().unwrap();
    println!("DEBUG: Folder name: {}", folder_name);
    let path_name = format!("{}{}", SAVED_BOOKS_PATH, folder_name);
    println!("DEBUG: Folder path: {}", path_name);
    std::fs::create_dir_all(&path_name)?;

    let mut metadata_file = File::create(format!("{}/metadata.json", path_name)).unwrap();
    let book = EpubDoc::new(path)?;
    let metadata_map = get_metadata_from_epub(&book)?;

    let json = json!(metadata_map);
    //json["cover"] = json!(book.get_cover().unwrap_or_default());
    metadata_file
        .write_all(json.to_string().as_bytes())
        .unwrap();
    Ok(metadata_map)
}

pub fn extract_chapters(path: &str) -> Result<(), Box<dyn error::Error>> {
    let file_name = path.split("/").last().unwrap();
    let folder_name = file_name.split(".").next().unwrap();
    println!("DEBUG: Folder name: {}", folder_name);
    let path_name = format!("{}{}", SAVED_BOOKS_PATH, folder_name);
    println!("DEBUG: Folder path: {}", path_name);
    std::fs::create_dir_all(&path_name)?;

    let mut book = EpubDoc::new(path)?;

    let len = book.get_num_pages();

    //extract all chapters
    let mut i = 0;
    while i < len {
        let chapter = book.get_current_str().unwrap();
        let mut file = File::create(format!("{}/page_{}.html", &path_name, i)).unwrap();
        file.write_all(chapter.as_bytes()).unwrap();
        if let Err(_) = book.go_next() {
            break;
        }
        i += 1;
    }
    Ok(())
}

pub fn get_chapter_text(path: &str, chapter_number: usize) -> Rc<String> {
    let file_name = path.split("/").last().unwrap();
    let folder_name = file_name.split(".").next().unwrap();
    let mut text_rc: Rc<String> = Rc::from(String::new());

    // try to read from txt files (where edited text is saved)
    if let Ok(text) = get_chapter(folder_name, chapter_number, FileExtension::TXT) {
        println!("DEBUG: reading from txt file");
        text_rc = text.into();
    }
    // try to read from html files
    else if let Ok(text) = get_chapter(folder_name, chapter_number, FileExtension::HTML) {
        println!("DEBUG: reading from html files");
        text_rc = text.into();
    }
    // if it fails, read from epub and save html page
    else if let Ok(mut book) = EpubDoc::new(path) {
        println!("DEBUG: reading from epub file");
        book.set_current_page(chapter_number).unwrap();
        let content = book.get_current_str().unwrap();
        let text = html2text::from_read(content.as_bytes(), 100);
        // save html page
        let mut file = File::create(format!("{}/page_{}.html", &path, chapter_number)).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        text_rc = text.into()
    }
    text_rc
}

pub fn get_metadata_of_book(path: &str) -> HashMap<String, String> {
    let book_filename = Path::new(path).file_name().unwrap().to_str().unwrap();
    let book_name = book_filename.split(".").next().unwrap();

    let metadata_path = format!("{}{}/metadata.json", SAVED_BOOKS_PATH, book_name);
    if let Ok(metadata_file) = File::open(metadata_path) {
        let reader = BufReader::new(metadata_file);
        if let Ok(metadata) = serde_json::from_reader(reader) {
            return metadata;
        }
    }

    // if it fails, read from epub, saves and return metadata
    let metadata = extract_metadata(path).expect("Failed to extract metadata from epub");
    metadata
}

/// Function that split the text of the chapter
/// into a vector of strings, each string is a paragraph
/// calculated by the number of lines and (not now) the font size
/// You  have to provide the path. Number of lines and font size
/// You can provide the text of the chapter as a RC String or
/// you can provide the chapter number
pub fn split_chapter_in_vec <S: Into<Option<Rc<String>>>, U: Into<Option<usize>>>(path: &str, opt_text: S, chapter_number: U, number_of_lines: usize, font_size: usize) -> Vec<Rc<String>> {
    // todo(): consider also the font size
    
    let text = match opt_text.into() {
        Some(book_chapter_text) => book_chapter_text,
        None => get_chapter_text(path, chapter_number.into().unwrap_or(0)),
    };
    let lines = text.split("\n\n").collect::<Vec<&str>>();
    lines
    .into_iter()
    .enumerate()
    .map(|(idx, line)| match idx % number_of_lines {
        0 => Rc::new(line.to_string()),
        _ => Rc::new(format!("{}{}", "\n\n", line)),
    })
    .collect()
}

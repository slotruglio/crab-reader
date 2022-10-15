use std::{collections::HashMap, error, fs::{File, OpenOptions}, io::{Write, Read}};
use super::saveload::get_chapter_html;
use epub::doc::EpubDoc;
use serde_json::json;
/// Method to extract metadata from epub file
/// and returns explicit metadata.
/// title: title of the book
/// author: author of the book
/// lang: language of the book
/// cover: png image of the cover of the book as a vector of bytes (as u8)
/// source: source of the book
/// date: date of the book
/// rights: rights of the book
/// identifier: identifier of the book
pub fn get_metadata_from_epub(path: &str) -> Result<HashMap<String, String>, Box<dyn error::Error>>{
    let mut book = EpubDoc::new(path)?;
    for key in book.metadata.keys() {
        println!("{}: {}", key, book.mdata(key).unwrap());
    }
    let mut metadata = HashMap::new();
    metadata
    .insert("title".to_string(), book.mdata("title")
    .unwrap_or("no title".to_string()));

    metadata
    .insert("author".to_string(), book.mdata("creator")
    .unwrap_or("no author".to_string()));

    metadata
    .insert("lang".to_string(), book.mdata("language")
    .unwrap_or("no lang".to_string()));

    metadata
    .insert("cover".to_string(), String::from_utf8(book.get_cover()
    .unwrap_or(Vec::<u8>::default()))?);

    metadata
    .insert("source".to_string(), book.mdata("source")
    .unwrap_or("no source".to_string()));

    metadata
    .insert("date".to_string(), book.mdata("date")
    .unwrap_or("no date".to_string()));

    metadata
    .insert("rights".to_string(), book.mdata("rights")
    .unwrap_or("no rights".to_string()));

    metadata
    .insert("identifier".to_string(), book.mdata("identifier")
    .unwrap_or("no indetifier".to_string()));

    Ok(metadata)
}

/// Method to save the cover of the book as a png file
/// in the path specified.
/// image: String of vec[u8] (as u8) of the cover
/// name: name of the file
/// path: path where to save the cover
pub fn save_book_cover(image: String, name: String, path_to_save: String) -> Result<(), Box<dyn error::Error>>{
    let cover = image.as_bytes();
    let mut filename = path_to_save;
    filename.push_str(&name);
    filename.push_str(".png");
    let mut file = File::create(filename)?;
    file.write_all(&cover)?;
    Ok(())
}

pub fn edit_chapter(path: &str, chapter_number: usize, old_text: impl Into<String>, text: impl Into<String>) -> Result<(), Box<dyn error::Error>>{
    let book_name = path.split("/").last().unwrap().split(".").next().unwrap();
    let saved_book_chapter_path = format!("assets/books/{}/page_{}.html", book_name, chapter_number);
    println!("path to get chapter: {}", saved_book_chapter_path);
    let mut page_html = OpenOptions::new().read(true).write(true).open(saved_book_chapter_path)?;

    let mut content = String::new();
    page_html.read_to_string(&mut content).unwrap();

    let new_content = content.replace(&old_text.into(), &text.into());


    page_html.write_all(new_content.as_bytes())?;
    Ok(())
}

pub fn extract_pages(path: &str) -> Result<(), Box<dyn error::Error>>{
    let file_name = path.split("/").last().unwrap();
    let folder_name = file_name.split(".").next().unwrap();
    println!("Folder name: {}", folder_name);
    let path_name = format!("assets/books/{}", folder_name);
    println!("Folder path: {}", path_name);
    std::fs::create_dir_all(format!("/Users/slotruglio/pds/crab-reader/{}", path_name.as_str()))?;
    
    let mut book = EpubDoc::new(path)?;
    
    let mut metadata_file = File::create(format!("{}/metadata.json", path_name)).unwrap();
    let mut metadata_map = HashMap::new();
    metadata_map.insert("title", book.mdata("title").unwrap_or("No title".to_string()));
    metadata_map.insert("author", book.mdata("creator").unwrap_or("No author".to_string()));
    metadata_map.insert("lang", book.mdata("language").unwrap_or("No lang".to_string()));

    metadata_map
    .insert("source", book.mdata("source")
    .unwrap_or("no source".to_string()));

    metadata_map
    .insert("date", book.mdata("date")
    .unwrap_or("no date".to_string()));

    metadata_map
    .insert("rights", book.mdata("rights")
    .unwrap_or("no rights".to_string()));

    metadata_map
    .insert("identifier", book.mdata("identifier")
    .unwrap_or("no indetifier".to_string()));
    let json = json!(metadata_map);
    //json["cover"] = json!(book.get_cover().unwrap_or_default());
    metadata_file.write_all(json.to_string().as_bytes()).unwrap();

    let len = book.get_num_pages();
    //extract all chapters
    let mut i = 0;
    while i < len {
        let chapter = book.get_current_str().unwrap();
        let mut file = File::create(format!("{}/page_{}.html", path_name.as_str(), i)).unwrap();
        file.write_all(chapter.as_bytes()).unwrap();
        if let Err(_) = book.go_next() {
            break;
        }
        i += 1;
    }
    Ok(())
}

pub fn get_chapter_text(path: &str, chapter_number: usize) -> String {
    let file_name = path.split("/").last().unwrap();
    let folder_name = file_name.split(".").next().unwrap();

    // try to read from html files
    if let Ok(text) = get_chapter_html(folder_name, chapter_number) {
        println!("reading from html files");
        return text;
    }

    // if it fails, read from epub
    if let Ok(mut book) = EpubDoc::new(path){
        println!("reading from epub file");
        book.set_current_page(chapter_number).unwrap();
        let content = book.get_current_str().unwrap();
        let text = html2text::from_read(content.as_bytes(), 100);
        text
    } else { String::from("No text") }
}
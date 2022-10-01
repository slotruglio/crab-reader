use std::{collections::HashMap, error, fs::File, io::Write};
use epub::doc::EpubDoc;
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
use std::{fs::{File, OpenOptions}, io::{BufReader, Write}};

use serde_json::{json, Value};

/// function to save page of chapter of currently opened book
pub fn save_page_of_chapter(book_path: String, chapter: usize, page: usize) -> Result<(), Box<dyn std::error::Error>>{
    println!("DEBUG saving data: {} {} {}", chapter.clone(), page.clone(), book_path.clone());
    let filename = String::from("src/conf/books_saved.json");
    let value = json!({ "chapter":chapter, "page":page});
    
    if let Ok(file) = File::open(filename.clone()) {
        let reader = BufReader::new(file);
        let mut json: Value = serde_json::from_reader(reader)?;
        json[book_path] = value;
        let mut file = OpenOptions::new().write(true).open(filename.clone())?;
        file.write_all(json.to_string().as_bytes())?;
    }else{
        let mut file = OpenOptions::new().write(true).create(true).open(filename)?;
        let mut json = json!({});
        json[book_path] = value;
        file.write_all(json.to_string().as_bytes())?;
    }
    Ok(())
}

/// function to load the last page of a chapter given the path of the book
pub fn get_page_of_chapter(book_path: String) -> Result<(usize, usize), Box<dyn std::error::Error>>{
    let filename = String::from("src/conf/books_saved.json");
    let mut chapter = 1;
    let mut page = 0;

    if let Ok(file) = File::open(filename) {
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader)?;

        if let Some(value) = json.get(book_path.clone()){
            chapter = value.get("chapter").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
            page = value.get("page").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            
        };
        
    }
    println!("DEBUG reading data: {} {} {}", chapter.clone(), page.clone(), book_path);
    Ok((chapter, page))
}
use std::{fs::{File, OpenOptions}, io::{BufReader, Read}, sync::mpsc::channel};

use serde_json::{json, Value};

/// function to save page of chapter of currently opened book
pub fn save_page_of_chapter<T: Into<String> + Clone>(book_path: T, chapter: usize, page: usize) -> Result<(), Box<dyn std::error::Error>>{
    println!("DEBUG saving data: {} {} {}", chapter.clone(), page.clone(), book_path.clone().into());
    let filename = String::from("src/conf/books_saved.json");
    let filename2 = filename.clone();
    let (tx, rx) = channel();

    let thread = std::thread::spawn(move || {
        let mut json = json!({});
        if let Ok(opened_file) = File::open(filename2) {
            println!("DEBUG file exists");
            let reader = BufReader::new(opened_file);
            if let Ok(content) = serde_json::from_reader(reader) { json = content};
        }
        tx.send(json).unwrap();
    });

    let value = json!({"chapter":chapter, "page":page});

    if let Ok(()) = thread.join() {
        let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;

        let mut json = rx.recv().unwrap();

        json[book_path.into()] = value;

        serde_json::to_writer_pretty(file, &json)?;
        drop(rx);
        Ok(())
    }else{ Err("Error while saving data".into()) }
}

/// function to load the last page of a chapter given the path of the book
pub fn get_page_of_chapter<T: Into<String> + Clone>(book_path: T) -> Result<(usize, usize), Box<dyn std::error::Error>>{
    let filename = String::from("src/conf/books_saved.json");
    let mut chapter = 1;
    let mut page = 0;

    if let Ok(file) = File::open(filename) {
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader)?;

        if let Some(value) = json.get(book_path.clone().into()){
            chapter = value.get("chapter").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
            page = value.get("page").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            
        };
        
    }
    println!("DEBUG reading data: {} {} {}", chapter.clone(), page.clone(), book_path.into());
    Ok((chapter, page))
}

pub fn get_chapter_html(folder_name: &str, chapter: usize) -> Result<String, Box<dyn std::error::Error>>{
    let filename = format!("assets/books/{}/page_{}.html", folder_name, chapter);
    println!("filename from where get page: {}", filename);
    let file = File::open(filename)?;
    let mut content = String::new();
    BufReader::new(file).read_to_string(&mut content)?;
    let text = html2text::from_read(content.as_bytes(), 100);
    
    Ok(text)
}

pub fn get_chapter_txt(folder_name: &str, chapter: usize) -> Result<String, Box<dyn std::error::Error>>{
    let filename = format!("assets/books/{}/page_{}.txt", folder_name, chapter);
    println!("filename from where get page: {}", filename);
    let file = File::open(filename)?;
    let mut content = String::new();
    BufReader::new(file).read_to_string(&mut content)?;
    
    Ok(content)
}
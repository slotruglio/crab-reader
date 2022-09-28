let path = "/Users/slotruglio/pds/crab-reader/src/assets/books/pg69058-images.epub";
    
    let mut book = EpubDoc::new(path).unwrap();
    for key in book.metadata.keys() {
        println!("{}: {}", key, book.mdata(key).unwrap());
    }

    let title = book.mdata("title").unwrap();
    let author = book.mdata("creator").unwrap();
/*
    // this is to save the cover image
    let cover_data = book.get_cover().unwrap();

    let mut title_to_save = title;
    title_to_save.push_str(".png");

    println!("Saving cover to {}", title_to_save);

    let f = File::create(title_to_save);
    assert!(f.is_ok());
    let mut f = f.unwrap();
    let resp = f.write_all(&cover_data);
*/
    let chapter_number = book.get_current_page();
    let content = book.get_current_str().unwrap();
    let text = html2text::from_read(content.as_bytes(), 100);

    //let json = Dom::parse(tpage.as_str()).unwrap().to_json_pretty().unwrap();
    //println!("{}", json);
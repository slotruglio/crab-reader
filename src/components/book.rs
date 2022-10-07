use druid::Widget;
use druid::widget::{Flex, Container, TextBox, Label, Image, FillStrat, Padding, Scroll, Button};
use druid::{Color, Data, Lens};
use std::io::Read;
use std::io::Write;
use std::fs::File;
use epub::doc::EpubDoc;
use crate::utils::saveload;

/// Struct that models EPUB file
/// Metadata are attributes
#[derive(Clone, Data, Lens)]
pub struct Book {
    title: String,
    author: String,
    lang: String,
    cover: String,
    path: String,
    chapter_number: usize,
    current_page: usize
}

impl Book {
    /// Method that instantiates a new Book from a epub file
    /// given its path
    pub fn new(path: &str) -> Book {
        let mut book = EpubDoc::new(path).unwrap();
        
        let title = book.mdata("title").unwrap_or("No title".to_string());
        let author = book.mdata("creator").unwrap_or("No author".to_string());
        let lang = book.mdata("language").unwrap_or("No lang".to_string());
        let cover_data = book.get_cover().unwrap_or(vec![0]);

        let mut title_to_save = "/Users/slotruglio/pds/crab-reader/src/assets/covers/".to_string();
        title_to_save.push_str(title.as_str());
        title_to_save.push_str(".png");

        println!("Saving cover to {}", title_to_save);

        let copy_title = title_to_save.clone();
        let f = File::create(copy_title);
        assert!(f.is_ok());
        let mut f = f.unwrap();
        let resp = f.write_all(&cover_data);

        let (chapter_number, current_page) = saveload::get_page_of_chapter(String::from(path)).unwrap();
        Book{title, author, lang, cover: title_to_save, path: String::from(path), chapter_number: chapter_number, current_page: current_page}
    }
    
    /// Method that returns the current chapter number
    pub fn get_chapter_number(&self) -> usize {
        self.chapter_number
    }
    /// Method that set the chapter number
    pub fn set_chapter_number(&mut self, chapter: usize) {
        self.chapter_number = chapter;
        self.current_page = 0;
    }
    /// Method that returns the number of 
    /// the last page of the current chapter
    pub fn get_last_page(&self) -> usize {
        self.split_chapter_in_pages().len()-1 as usize
    }
    /// Method that returns the current page number
    pub fn get_current_page(&self) -> usize {
        self.current_page
    }
    /// Method that set the current page number
    /// you can use it to change page
    /// Example: go back and go forward
    pub fn set_chapter_current_page(&mut self, page: usize) {
        self.current_page = page;
    }
    /// Method that returns the text of the current chapter
    pub fn get_chapter_text(&self) -> String {
        let mut book = EpubDoc::new(self.path.as_str()).unwrap();
        book.set_current_page(self.chapter_number).unwrap();
        let content = book.get_current_str().unwrap();
        let text = html2text::from_read(content.as_bytes(), 100);
        text
    }
    /// Method that splits the chapter in blocks of 12 lines ATM
    /// and returns a vector of strings. Each string is a page of the chapter
    pub fn split_chapter_in_pages(&self) -> Vec<String> {
        // TODO() number_of_lines as parameter
        let number_of_lines = 4;
        let text = self.get_chapter_text();
        let lines = text.split("\n\n").collect::<Vec<&str>>();
        let mut pages = Vec::new();

        let mut counter_pages = 0;
        for (i, line) in lines.iter().enumerate() {
            if i % number_of_lines == 0 {
                if i != 0 {
                    counter_pages += 1;
                }
                pages.push(line.to_string());
            }else{
                pages[counter_pages].push_str("\n\n");
                pages[counter_pages].push_str(line);
            }
        }
        pages
        
    }
    /// Method that returns the page as String of the current chapter
    pub fn get_page_of_chapter(&self) -> String {
        let page = self.split_chapter_in_pages();
        page[self.current_page].clone()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
    /*
    fn get_widget_chapter(&self) -> impl Widget<Book> {
        let chapter_number = Label::new(format!("Chapter {}", self.chapter_number));
        let chapter_text = Label::new(self.get_chapter_text());
        println!("Chapter text: {}", self.get_chapter_text());
        Padding::new(
            10.0, 
            Container::new(
                Flex::column()
                .with_child(Button::from_label(chapter_number).on_click(|ctx, data: &mut Book, _env| {
                    data.set_chapter_number(data.get_chapter_number()+1);
                    ctx.request_paint();
                    println!("Chapter: {}", data.get_chapter_number());
                }))
                .with_child(chapter_text)
            ).border(Color::rgb8(255, 255, 255), 1.0)
        )
    }
    */

    /*
    /// Method that returns the widget of the preview of the book
    fn get_widget_library(&self) -> impl Widget<()> {
        let title = Label::new(self.title.clone());
        let author = Label::new(self.author.clone());
        let lang = Label::new(self.lang.clone());
        let path = Label::new(self.path.clone());

        Padding::new(
            10.0, 
            Container::new(
                Flex::column()
                .with_child(title)
                .with_child(author)
                .with_child(lang)
                .with_child(path)
                
            ).border(Color::rgb8(255, 255, 255), 1.0)
        )
    }
    */
}

use druid::{Widget, ArcStr};
use druid::piet::TextStorage;
use druid::text::RichText;
use druid::widget::{Flex, Container, TextBox, Label, Image, FillStrat, Padding, Scroll, Button};
use druid::{Color, Data, Lens};
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::sync::Arc;
use epub::doc::EpubDoc;

use crate::utils::text_descriptor::get_chapter_rich_text;
use crate::utils::{saveload, text_descriptor};

const NUMBER_OF_LINES: usize = 60;

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
    current_page: usize,
    rich_text: RichText,
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
        let rich_text = get_chapter_rich_text(path, chapter_number, NUMBER_OF_LINES).get(current_page).unwrap().clone();


        Book{
            title, 
            author, 
            lang, 
            cover: title_to_save, 
            path: String::from(path), 
            chapter_number, 
            current_page, 
            rich_text,
        }
    }
    
    /// Method that returns the current chapter number
    pub fn get_chapter_number(&self) -> usize {
        self.chapter_number
    }
    /// Method that set the chapter number
    pub fn set_chapter_number(&mut self, chapter: usize) {
        self.chapter_number = chapter;
        self.current_page = 0;
        self.rich_text = text_descriptor::get_chapter_rich_text(self.path.clone().as_str(), chapter, NUMBER_OF_LINES)[0].clone();
    }
    /// Method that returns the number of 
    /// the last page of the current chapter
    pub fn get_last_page(&self) -> usize {
        text_descriptor::get_chapter_rich_text(self.path.clone().as_str(), self.chapter_number, NUMBER_OF_LINES).len()-1
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
        self.rich_text = text_descriptor::get_chapter_rich_text(self.path.clone().as_str(), self.chapter_number, NUMBER_OF_LINES)[page].clone();
    }
    /// Method that returns the text of the current chapter
    
    /* DEPRECATED
    pub fn get_chapter_text(&self) -> String {
        if let Ok(mut book) = EpubDoc::new(self.path.as_str()){
            book.set_current_page(self.chapter_number).unwrap();
            let content = book.get_current_str().unwrap();
            let text = html2text::from_read(content.as_bytes(), 100);
            text
        } else { String::from("No text") }
    }
    */
    /* DEPRECATED
    /// Method that returns rich text of the current chapter
    /// ERROR MANAGEMENT: NOT DONE
    pub fn get_chapter_rich_text(&self) -> RichText {
        let mut book = EpubDoc::new(self.path.as_str()).unwrap();
        book.set_current_page(self.chapter_number).unwrap();
        let content = book.get_current_str().unwrap();
        let vec_tagged = html2text::from_read_rich(content.as_bytes(), 50);
        
        let mut text = String::new();
        // first usize is the starting index of the tag
        // last usize is the ending index of the tag
        // String is the tag
        let mut tags = Vec::<(usize, usize, String)>::new();
        for line in vec_tagged {
            println!("line: {:?}", line);
            for item in line.into_tagged_strings() {

                let starting_index = text.len();
                if item.s.starts_with("*") & item.s.ends_with("*") {
                    text.push_str(item.s.replace("*", "").as_str());
                }else{
                    text.push_str(item.s.as_str());
                }
                let ending_index = text.len()-1;

                if let Some(tag) = item.tag.get(0) {
                    let tag = match tag {
                        html2text::render::text_renderer::RichAnnotation::Default => "default",
                        //html2text::render::text_renderer::RichAnnotation::Link(link) => "a",
                        html2text::render::text_renderer::RichAnnotation::Link(link) => "em",
                        //html2text::render::text_renderer::RichAnnotation::Image(link) => "img",
                        html2text::render::text_renderer::RichAnnotation::Emphasis => "em",
                        html2text::render::text_renderer::RichAnnotation::Strong => "strong",
                        //html2text::render::text_renderer::RichAnnotation::Strikeout => "strike",
                        //html2text::render::text_renderer::RichAnnotation::Code => "code",
                        //html2text::render::text_renderer::RichAnnotation::Preformat(boolean) => "preformat",
                        _ => "none"
                    };
                    tags.push((starting_index, ending_index, tag.to_string()));
                }

            }
            text.push_str("\n");
        }
        text_descriptor::get_rich_text(text, tags)

    }
    */
    
    /* DEPRECATED
    /// Method that splits the chapter in blocks of 8 lines ATM
    /// and returns a vector of strings. Each string is a page of the chapter
    pub fn split_chapter_in_pages(&self) -> Vec<String> {
        // TODO() number_of_lines as parameter
        
        let text = self.get_chapter_text();
        let lines = text.split("\n\n").collect::<Vec<&str>>();
        let mut pages = Vec::new();

        let mut counter_pages = 0;
        for (i, line) in lines.iter().enumerate() {
            if i % NUMBER_OF_LINES == 0 {
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
    */

    /// Method that returns the page as String of the current chapter
    pub fn get_page_of_chapter(&self) -> RichText {
        self.rich_text.clone()
    }
/* 
    pub fn get_dual_pages(&self) -> (RichText, RichText) {
        let page = self.rich_text;
        let mut left_page;
        let mut right_page;
        if self.current_page % 2 == 0 {
            left_page = pages[self.current_page].clone();
            if self.current_page+1 < pages.len() {
                right_page = pages[self.current_page+1].clone();
            }
        }else{
            right_page = pages[self.current_page].clone();
            if self.current_page-1 < pages.len() {
                left_page = pages[self.current_page-1].clone();
            }
        }
        (left_page, right_page)
    }
    */

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

use crate::utils::{epub_utils, saveload, text_descriptor};
use std::rc::Rc;
use std::string::String;

/// This trait defines all the methods that a `Book` struct must implement
/// in order to be rendered visually correct in the GUI of the application.
pub trait GUIBook {
    /// Returns the title
    fn get_title(&self) -> Rc<String>;

    /// Builder pattern for title
    fn with_title(self, title: impl Into<String>) -> Self;

    /// Sets the title for the book
    fn set_title(&mut self, title: impl Into<String>);

    /// Returns the author
    fn get_author(&self) -> Rc<String>;

    /// Builder pattern for author
    fn with_author(self, author: impl Into<String>) -> Self;

    /// Sets the author for the book
    fn set_author(&mut self, author: impl Into<String>);

    /// Returns the number of pages
    fn get_number_of_pages(&self) -> usize;

    /// Builder pattern for number of pages
    fn with_number_of_pages(self, npages: usize) -> Self;

    /// Sets the number of pages for the book
    fn set_number_of_pages(&mut self, npages: usize);

    /// Returns the number of read pages
    fn get_number_of_read_pages(&self) -> usize;

    /// Builder pattern for number of read pages
    fn with_number_of_read_pages(self, read_pages: usize) -> Self;

    /// Sets the number of read pages for the book
    fn set_number_of_read_pages(&mut self, read_pages: usize);

    /// Returns the index of the book.
    ///
    /// The idx is intended to be the position in the array of the `Library` struct (relax this constraint?)
    fn get_index(&self) -> usize;

    /// Builder pattern for index
    ///
    /// The idx is intended to be the position in the array of the `Library` struct (relax this constraint?)
    fn with_index(self, idx: usize) -> Self;

    /// Sets the index of the book.
    ///
    /// The idx is intended to be the position in the array of the `Library` struct (relax this constraint?)
    fn set_index(&mut self, idx: usize);

    /// Returns the cover image, codified as a &[u8].
    /// The format is for now to be intended to be as RGB8
    ///
    fn get_cover(&self) -> Rc<&[u8]>;

    /// Builder method for the cover image, codified as a &[u8].
    /// The format is for now to be intended to be as RGB8
    ///
    fn with_cover(self, img: &[u8]) -> Self;

    /// Sets the he cover image, codified as a &[u8].
    /// The format is for now to be inteded as RGB8
    ///
    fn set_cover(&mut self, img: &[u8]);

    /// Returns the description (i.e, like a synopsis for the book)
    fn get_description(&self) -> Rc<String>;

    /// Builder pattern for the description (i.e, like a synopsis for the book)
    fn with_description(self, description: impl Into<String>) -> Self;

    /// Sets the description (i.e, like a synopsis for the book)
    fn set_description(&mut self, description: impl Into<String>);

    /// Returns the selected state of the book
    fn is_selected(&self) -> bool;

    /// Sets the book as selected
    fn set_selected(&mut self, selected: bool);

    /// Set the book as selected
    fn select(&mut self);

    /// Set the book as unselected
    fn unselect(&mut self);
}
use druid::text::RichText;

use druid::{Data, Lens};
use epub::doc::EpubDoc;
use std::fs::File;
use std::io::Write;

const NUMBER_OF_LINES: usize = 8;

/// trait that describes the book reading functions
pub trait BookReading {
    /// Method that returns the current chapter number
    fn get_chapter_number(&self) -> usize;

    /// Method that set the chapter number
    fn set_chapter_number(&mut self, chapter: usize);

    /// Method that returns the number of
    /// the last page of the current chapter
    fn get_last_page_number(&self) -> usize;

    /// Method that returns the current page number
    fn get_current_page_number(&self) -> usize;

    /// Method that set the current page number
    /// you can use it to change page
    /// Example: go back and go forward
    fn set_chapter_current_page_number(&mut self, page: usize);

    /// Method that returns the text of the current chapter
    fn get_chapter_text(&self) -> Rc<String>;

    /// Method that returns rich text of the current chapter
    fn get_chapter_rich_text(&self) -> RichText;

    /// Method that returns the page as Rc<String> of the current chapter
    fn get_page_of_chapter(&self) -> Rc<String>;

    /// Method that returns two pages dealing with two page mode
    fn get_dual_pages(&self) -> (Rc<String>, Rc<String>);
}

/// Trait that describes book management functions
/// not related directly to the reading
pub trait BookManagement {
    /// Method that returns the path of the book
    fn get_path(&self) -> Rc<String>;

    /// Method that splits the chapter in blocks of const NUMBER_OF_LINES
    /// and returns a vector of strings. Each string is a page of the chapter
    fn split_chapter_in_pages(&self) -> Vec<String>;

    /// Method that edits the text of the current chapter
    /// new text is already in self.chapter_page_text
    fn edit_text(&mut self, old_text: String);
}

/// Struct that models EPUB file
/// Metadata are attributes
#[derive(Clone, PartialEq, Data, Lens)]
pub struct Book {
    chapter_number: usize,
    current_page: usize,
    idx: usize,
    selected: bool,
    title: Rc<String>,
    author: Rc<String>,
    lang: Rc<String>,
    path: Rc<String>,
    chapter_text: Rc<String>,
    chapter_page_text: Rc<String>,
    description: Rc<String>,
}

impl Book {
    /// Method that instantiates a new Book from a epub file
    /// given its path
    pub fn new(path: impl Into<String>) -> Book {
        let path: String = path.into();
        let _result = epub_utils::extract_pages(path.as_str()).unwrap();
        let mut book = EpubDoc::new(path.as_str()).unwrap();

        let title = book.mdata("title").unwrap_or("No title".to_string());

        let author = book.mdata("creator").unwrap_or("No author".to_string());

        let lang = book.mdata("language").unwrap_or("No lang".to_string());

        let cover_data = book.get_cover().unwrap_or(vec![0]);

        let title_to_save = format!(
            "{}{}{}",
            "/Users/slotruglio/pds/crab-reader/assets/covers/",
            title.as_str(),
            ".png"
        );

        println!("DEBUG: Saving cover to {}", title_to_save);

        let copy_title = title_to_save.clone();
        let f = File::create(copy_title);
        assert!(f.is_ok());
        let mut f = f.unwrap();
        let _resp = f.write_all(&cover_data);

        let (chapter_number, current_page) = saveload::get_page_of_chapter(path).unwrap();
        let chapter_text = epub_utils::get_chapter_text(path.as_str(), chapter_number);
        let chapter_page_text = chapter_text[0..200].to_string();
        todo!()
        // Book {
        // title,
        // author,
        // lang,
        // cover: title_to_save,
        // path: String::from(path),
        // chapter_number,
        // current_page,
        // chapter_text,
        // chapter_page_text,
        // }
    }
}

impl BookReading for Book {
    fn get_chapter_number(&self) -> usize {
        self.chapter_number
    }

    fn set_chapter_number(&mut self, chapter: usize) {
        self.chapter_number = chapter;
        self.current_page = 0;
        self.chapter_text = epub_utils::get_chapter_text(self.path.clone().as_str(), chapter);
    }

    fn get_last_page_number(&self) -> usize {
        self.split_chapter_in_pages().len() - 1 as usize
    }

    fn get_current_page_number(&self) -> usize {
        self.current_page
    }

    fn set_chapter_current_page_number(&mut self, page: usize) {
        self.current_page = page;
        self.chapter_page_text = self.split_chapter_in_pages()[page].clone();
    }

    fn get_chapter_text(&self) -> Rc<String> {
        if self.chapter_text.len() > 0 {
            return self.chapter_text.clone();
        }
        epub_utils::get_chapter_text(self.path.clone().as_str(), self.chapter_number)
    }

    fn get_chapter_rich_text(&self) -> RichText {
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
            println!("DEBUG: line: {:?}", line);
            for item in line.into_tagged_strings() {
                let starting_index = text.len();
                if item.s.starts_with("*") & item.s.ends_with("*") {
                    text.push_str(item.s.replace("*", "").as_str());
                } else {
                    text.push_str(item.s.as_str());
                }
                let ending_index = text.len() - 1;

                if let Some(tag) = item.tag.get(0) {
                    let tag = match tag {
                        html2text::render::text_renderer::RichAnnotation::Default => "default",
                        //html2text::render::text_renderer::RichAnnotation::Link(link) => "a",
                        html2text::render::text_renderer::RichAnnotation::Link(_link) => "em",
                        //html2text::render::text_renderer::RichAnnotation::Image(link) => "img",
                        html2text::render::text_renderer::RichAnnotation::Emphasis => "em",
                        html2text::render::text_renderer::RichAnnotation::Strong => "strong",
                        //html2text::render::text_renderer::RichAnnotation::Strikeout => "strike",
                        //html2text::render::text_renderer::RichAnnotation::Code => "code",
                        //html2text::render::text_renderer::RichAnnotation::Preformat(boolean) => "preformat",
                        _ => "none",
                    };
                    tags.push((starting_index, ending_index, tag.to_string()));
                }
            }
            text.push_str("\n");
        }
        text_descriptor::get_rich_text(text, tags)
    }

    fn get_page_of_chapter(&self) -> Rc<String> {
        let page = self.split_chapter_in_pages();
        page[self.current_page].clone()
    }

    fn get_dual_pages(&self) -> (String, String) {
        let page = self.split_chapter_in_pages();
        let mut left_page = String::new();
        let mut right_page = String::new();
        if self.current_page % 2 == 0 {
            left_page = page[self.current_page].clone();
            if self.current_page + 1 < page.len() {
                right_page = page[self.current_page + 1].clone();
            }
        } else {
            right_page = page[self.current_page].clone();
            if self.current_page - 1 < page.len() {
                left_page = page[self.current_page - 1].clone();
            }
        }
        (left_page, right_page)
    }
}

impl BookManagement for Book {
    fn get_path(&self) -> Rc<String> {
        self.path.clone()
    }

    fn split_chapter_in_pages(&self) -> Vec<String> {
        // TODO() number_of_lines as parameter
        // TODO() get first from attribute and then from
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
            } else {
                pages[counter_pages].push_str("\n\n");
                pages[counter_pages].push_str(line);
            }
        }
        pages
    }

    fn edit_text(&mut self, old_text: String) {
        let new_text = self.chapter_page_text.clone();
        for (old_line, new_line) in old_text.lines().zip(new_text.lines()) {
            if old_line != new_line {
                println!("DEBUG: old_line: {}", old_line);
                println!("DEBUG: new_line: {}", new_line);
                let new_chapter = self
                    .chapter_text
                    .replace(old_text.as_str(), new_text.as_str());
                if let Ok(()) = epub_utils::edit_chapter(
                    self.path.clone().as_str(),
                    self.chapter_number,
                    new_chapter,
                ) {
                    println!("DEBUG: Text edited");
                    self.chapter_text = epub_utils::get_chapter_text(
                        self.path.clone().as_str(),
                        self.chapter_number,
                    );
                    self.chapter_page_text =
                        self.split_chapter_in_pages()[self.current_page].clone();
                }
            }
        }
    }
}

impl GUIBook for Book {
    fn get_title(&self) -> Rc<String> {
        self.title.clone()
    }

    fn with_title(mut self, title: impl Into<String>) -> Self {
        self.set_title(title);
        self
    }

    fn set_title(&mut self, title: impl Into<String>) {
        self.title = Rc::new(title.into());
    }

    fn get_author(&self) -> Rc<String> {
        self.author.clone()
    }

    fn with_author(mut self, author: impl Into<String>) -> Self {
        self.set_author(author);
        self
    }

    fn set_author(&mut self, author: impl Into<String>) {
        self.author = Rc::new(author.into());
    }

    fn get_number_of_pages(&self) -> usize {
        self.npages
    }

    fn with_number_of_pages(mut self, npages: usize) -> Self {
        self.set_number_of_pages(npages);
        self
    }

    fn set_number_of_pages(&mut self, npages: usize) {
        self.npages = npages;
    }

    fn get_number_of_read_pages(&self) -> usize {
        self.read_pages
    }

    fn with_number_of_read_pages(mut self, read_pages: usize) -> Self {
        self.set_number_of_read_pages(read_pages);
        self
    }

    fn set_number_of_read_pages(&mut self, read_pages: usize) {
        self.read_pages = read_pages;
    }

    fn get_index(&self) -> usize {
        self.idx
    }

    fn with_index(mut self, idx: usize) -> Self {
        self.set_index(idx);
        self
    }

    fn set_index(&mut self, idx: usize) {
        self.idx = idx as usize
    }

    fn with_description(mut self, description: impl Into<String>) -> Self {
        self.set_description(description);
        self
    }

    fn set_description(&mut self, description: impl Into<String>) {
        self.description = Rc::new(description.into());
    }

    fn is_selected(&self) -> bool {
        self.selected == true
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    fn select(&mut self) {
        self.set_selected(true);
    }

    fn unselect(&mut self) {
        self.set_selected(false);
    }

    fn get_cover(&self) -> Rc<&[u8]> {
        todo!()
    }

    fn with_cover(self, img: &[u8]) -> Self {
        todo!()
    }

    fn set_cover(&mut self, img: &[u8]) {
        todo!()
    }
}

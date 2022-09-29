use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt, Color, Data, Lens, ImageBuf};
use druid::widget::{Flex, Container, TextBox, Label, Image, FillStrat, Padding, Scroll, Button};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use epub::doc::EpubDoc;


#[derive(Clone, Data, Lens)]
struct Book {
    title: String,
    author: String,
    lang: String,
    cover: String,
    path: String,
    chapter_number: usize,
}

impl Book {
    fn new(path: &str) -> Book {
        let mut book = EpubDoc::new(path).unwrap();
        
        let title = book.mdata("title").unwrap_or("No title".to_string());
        let author = book.mdata("creator").unwrap_or("No author".to_string());
        let lang = book.mdata("language").unwrap_or("No lang".to_string());
        let cover_data = book.get_cover().unwrap_or(vec![0]);

        let book = EpubDoc::new(path).unwrap();
        let chapter_number = book.get_current_page()+1;

        let mut title_to_save = "/Users/slotruglio/pds/crab-reader/src/assets/covers/".to_string();
        title_to_save.push_str(title.as_str());
        title_to_save.push_str(".png");

        println!("Saving cover to {}", title_to_save);

        let copy_title = title_to_save.clone();
        let f = File::create(copy_title);
        assert!(f.is_ok());
        let mut f = f.unwrap();
        let resp = f.write_all(&cover_data);

        Book{title, author, lang, cover: title_to_save, path: String::from(path), chapter_number: chapter_number}
    }
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
    fn get_chapter_number(&self) -> usize {
        self.chapter_number
    }
    fn set_chapter_number(&mut self, chapter: usize) {
        self.chapter_number = chapter;
    }
    fn get_chapter_text(&self) -> String {
        let mut book = EpubDoc::new(self.path.as_str()).unwrap();
        book.set_current_page(self.chapter_number).unwrap();
        let content = book.get_current_str().unwrap();
        let text = html2text::from_read(content.as_bytes(), 100);
        text
    }
    fn get_widget_chapter(&self) -> impl Widget<Book> {
        let chapter_number = Label::new(format!("Chapter {}", self.chapter_number));
        let chapter_text = Label::new(self.get_chapter_text());
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
}

fn build_ui() -> impl Widget<Book> {
    let dynamic_chapter = Label::dynamic(|data: &Book, _env: &_| data.get_chapter_number().clone().to_string());
    let dynamic_text = Scroll::new(Label::dynamic(|data: &Book, _env: &_| data.get_chapter_text().clone()));

    let button = Button::from_label(dynamic_chapter).on_click(|ctx, data: &mut Book, _env| {
        data.set_chapter_number(data.get_chapter_number()+1);
        ctx.request_paint();
        println!("Chapter: {}", data.get_chapter_number());
    });

    Flex::column()
    .with_flex_child(button, 1.0)
    .with_flex_child(dynamic_text, 2.0)
}

fn main() -> Result<(), PlatformError> {

    let book = Book::new("/Users/slotruglio/pds/crab-reader/src/assets/books/pg69058-images.epub");
    AppLauncher::with_window(WindowDesc::new(|| build_ui())).launch(book)?;
    Ok(())
}
use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt, Color, Data, Lens, ImageBuf};
use druid::widget::{Flex, Container, TextBox, Label, Image, FillStrat};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use epub::doc::EpubDoc;

#[derive(Clone, Data, Lens)]
struct Book {
    title: String,
    author: String,
    cover: String,
}

impl Book {
    fn new(path: &str) -> Book {
        let mut book = EpubDoc::new(path).unwrap();
        
        let title = book.mdata("title").unwrap();
        let author = book.mdata("creator").unwrap();
        let cover_data = book.get_cover().unwrap();

        let mut title_to_save = title.clone();
        title_to_save.push_str(".png");

        println!("Saving cover to {}", title_to_save);

        let copy_title = title_to_save.clone();
        let f = File::create(copy_title);
        assert!(f.is_ok());
        let mut f = f.unwrap();
        let resp = f.write_all(&cover_data);

        Book{title, author, cover: title_to_save}
    }
}

fn build_ui() -> impl Widget<Book> {

    Flex::row()
        .with_flex_child(
            Flex::column()
                .with_child(
                    Label::new(|data: &Book, _env: &_| data.title.clone())
                    .with_text_color(Color::WHITE)
                    .with_text_size(24.0)
                    
                )
                .with_child(
                    Label::new(|data: &Book, _env: &_| data.author.clone())
                    .with_text_color(Color::GRAY)
                    .with_text_size(20.0)
                    
                ),
            4.0
        )
        .with_flex_child(
            Label::new(|data: &Book, _env: &_| data.cover.clone())
                    .with_text_color(Color::WHITE)
                    .with_text_size(14.0)
            ,
            1.0
        )
}

fn main() -> Result<(), PlatformError> {
    
    let book = Book::new("/Users/slotruglio/pds/crab-reader/src/assets/books/pg69058-images.epub");

    AppLauncher::with_window(WindowDesc::new(|| build_ui())).launch(book)?;
    Ok(())
}
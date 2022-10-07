mod components;
mod utils;

use utils::{epub, saveload};
use components::book::Book;
use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt};
use druid::widget::{Flex, Label, Scroll, Button, LineBreaking};


fn build_ui() -> impl Widget<Book> {
    let dynamic_chapter = Label::dynamic(|data: &Book, _env: &_| data.get_chapter_number().clone().to_string());
    let dynamic_page_number = Label::dynamic(|data: &Book, _env: &_| data.get_current_page().clone().to_string());
    let dynamic_text = Scroll::new(Label::dynamic(|data: &Book, _env: &_| data.get_page_of_chapter().clone()).with_line_break_mode(LineBreaking::WordWrap).fix_size(400.0, 300.0)).vertical();

    let button = Button::from_label(dynamic_chapter).on_click(|ctx, data: &mut Book, _env| {
        let current_page = data.get_current_page();
        if current_page == data.get_last_page() {
            data.set_chapter_number(data.get_chapter_number()+1);
            println!("Last page of chapter, changing chapter");
        }else{
            data.set_chapter_current_page(current_page+1);
            println!("Changing page of chapter");
        }
        // function to save the page that the user is reading
        saveload::save_page_of_chapter(data.get_path(), data.get_chapter_number(), data.get_current_page()).unwrap();
        ctx.request_paint();
        println!("Chapter: {}", data.get_chapter_number());
    });

    Flex::column()
    .with_flex_child(button, 1.0)
    .with_flex_child(dynamic_page_number, 1.0)
    .with_flex_child(dynamic_text, 2.0)
}

fn main() -> Result<(), PlatformError> {

    let mut paths = Vec::new();
    paths.push("/Users/slotruglio/pds/crab-reader/src/assets/books/pg69058-images.epub");
    paths.push("/Users/slotruglio/pds/crab-reader/src/assets/books/collodi_pinocchio.epub");

    for path in paths {
        let _data = epub::get_metadata_from_epub(path);
        println!("---new book---");
    }

    let book = Book::new("/Users/slotruglio/pds/crab-reader/src/assets/books/pg69058-images.epub");
    
    
    AppLauncher::with_window(WindowDesc::new(|| build_ui())).launch(book)?;
    Ok(())
}
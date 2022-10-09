mod components;
mod utils;

use utils::{epub, saveload};
use components::book::Book;
use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt, Data, Lens, LensExt};
use druid::widget::{Flex, Label, Scroll, Button, LineBreaking, Switch, Either};

use crate::components::book::book_derived_lenses::current_page;

#[derive(Clone, Data, Lens)]
struct AppState {
    single_view: bool,
    book: Book
}

// single page view for text reader
fn build_single_view() -> impl Widget<AppState> {
    Scroll::new(Label::dynamic(|data: &AppState, _env: &_| data.book.get_page_of_chapter().clone()).with_line_break_mode(LineBreaking::WordWrap).fix_size(400.0, 300.0)).vertical()
}

// dual page view for text reader
fn build_dual_view() -> impl Widget<AppState> {
    Flex::row()
        .with_child(Scroll::new(Label::dynamic(|data: &AppState, _env: &_| data.book.get_dual_pages().0.clone()).with_line_break_mode(LineBreaking::WordWrap).fix_size(400.0, 300.0)).vertical())
        .with_child(Scroll::new(Label::dynamic(|data: &AppState, _env: &_| data.book.get_dual_pages().1.clone()).with_line_break_mode(LineBreaking::WordWrap).fix_size(400.0, 300.0)).vertical())
}

fn build_ui() -> impl Widget<AppState> {
    let dynamic_chapter = Label::dynamic(|data: &AppState, _env: &_| data.book.get_chapter_number().clone().to_string());
    let dynamic_page_number = Label::dynamic(|data: &AppState, _env: &_| data.book.get_current_page().clone().to_string());
    
    let switch = Switch::new().lens(AppState::single_view);
    let switch_view = Either::new(|data: &AppState, _env: &_| data.single_view, 
        build_single_view(), 
        build_dual_view()
    );

    let button = Button::from_label(dynamic_chapter).on_click(|ctx, data: &mut AppState, _env| {
        let increaser = match data.single_view {
            true => 1,
            false => 2
        };
        
        let new_page = data.book.get_current_page() + increaser;

        if new_page > data.book.get_last_page() {
            data.book.set_chapter_number(data.book.get_chapter_number()+1);
            println!("Last page of chapter, changing chapter");
        }else{
            data.book.set_chapter_current_page(new_page);
            println!("Changing page of chapter");
        }
        // function to save the page that the user is reading
        saveload::save_page_of_chapter(data.book.get_path(), data.book.get_chapter_number(), data.book.get_current_page()).unwrap();
        ctx.request_paint();
        println!("Chapter: {}", data.book.get_chapter_number());
    });

    Flex::column()
    .with_flex_child(button, 1.0)
    .with_flex_child(dynamic_page_number, 1.0)
    .with_flex_child(switch_view, 2.0)
    .with_flex_child(switch, 1.0)
}

fn main() -> Result<(), PlatformError> {

    let mut paths = Vec::new();
    paths.push("/Users/slotruglio/pds/crab-reader/src/assets/books/pg69058-images.epub");
    paths.push("/Users/slotruglio/pds/crab-reader/src/assets/books/collodi_pinocchio.epub");

    for path in paths {
        let _data = epub::get_metadata_from_epub(path);
        println!("---new book---");
    }

    let app_state = AppState {
        single_view: true,
        book: Book::new("/Users/slotruglio/pds/crab-reader/src/assets/books/collodi_pinocchio.epub")
    };

    let book = Book::new("/Users/slotruglio/pds/crab-reader/src/assets/books/pg69058-images.epub");
    
    
    AppLauncher::with_window(WindowDesc::new(|| build_ui())).launch(app_state)?;
    Ok(())
}
mod components;
mod utils;

use utils::{epub_utils, saveload, button_functions};
use components::book::Book;
use components::reader_view::*;
use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt, Data, Lens, LensExt};
use druid::widget::{Flex, Label, Scroll, Button, LineBreaking, Switch, Either, TextBox};

use crate::components::book::{BookReading, BookManagement};

#[derive(Clone, Data, Lens)]
struct AppState {
    single_view: bool,
    is_editing: bool,
    book: Book,
    text: String,
}



fn build_ui() -> impl Widget<AppState> {
    let dynamic_chapter = Label::dynamic(|data: &AppState, _env: &_| data.book.get_chapter_number().clone().to_string());
    let dynamic_page_number = Label::dynamic(|data: &AppState, _env: &_| data.book.get_current_page_number().clone().to_string());
    
    let edit_button = Button::from_label(
        Label::dynamic(|data: &AppState, _env: &_| format!("Edit is: {}", data.is_editing.to_string()))
    ).on_click(
        |ctx, data: &mut AppState, _env| {
            let (new_bool, new_text) = button_functions::edit_button(ctx, &mut data.book, data.text.clone(), data.is_editing);
            data.is_editing = new_bool;
            data.text = new_text;
        }
    );
    let edit_row = Flex::row()
    .with_child(edit_button);

    let switch = Switch::new().lens(AppState::single_view);
    let switch_row = Flex::row()
    .with_child(Label::new(" Single Page View "))
    .with_child(switch);
    
    let switch_view = Either::new(|data: &AppState, _env: &_| data.single_view, 
        Either::new(|data: &AppState, _env: &_| data.is_editing, 
            build_single_view_edit(), 
            build_single_view()
        ),
        build_dual_view()
    );

    let button = Button::from_label(dynamic_chapter)
    .on_click(|ctx, data: &mut AppState, _env| {
        button_functions::change_page(ctx, &mut data.book, data.is_editing, data.single_view, true);
    });

   let save_button = Button::from_label(Label::new("Save"))
   .on_click(|ctx, data: &mut AppState, _env| {
       data.book.edit_text(data.text.clone());
       ctx.request_paint();
    });
    

    Flex::column()
    .with_flex_child(button, 1.0)
    .with_flex_child(dynamic_page_number, 1.0)
    .with_flex_child(switch_view, 2.0)
    .with_flex_child(switch_row, 1.0)
    .with_flex_child(edit_row, 1.0)
    .with_flex_child(save_button, 1.0)
}

fn main() -> Result<(), PlatformError> {

    let mut paths = Vec::new();
    paths.push("/Users/slotruglio/pds/crab-reader/assets/books/epub/pg69058-images.epub");
    paths.push("/Users/slotruglio/pds/crab-reader/assets/books/epub/collodi_pinocchio.epub");

    for path in paths.iter() {
        let _data = epub_utils::get_metadata_from_epub(path.to_owned());
        println!("---new book---");
    }
    let book = Book::new(paths[0]);
    let page = book.get_page_of_chapter();

    let app_state = AppState {
        single_view: true,
        is_editing: false,
        book: book,
        text: page,
    };
    
    AppLauncher::with_window(WindowDesc::new(|| build_ui())).launch(app_state)?;
    Ok(())
}
mod components;
mod utils;

use utils::{epub_utils, saveload};
use components::book::Book;
use components::reader_view::*;
use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt, Data, Lens, LensExt};
use druid::widget::{Flex, Label, Scroll, Button, LineBreaking, Switch, Either, TextBox};

#[derive(Clone, Data, Lens)]
struct AppState {
    single_view: bool,
    is_editing: bool,
    book: Book,
    text: String,
}



fn build_ui() -> impl Widget<AppState> {
    let dynamic_chapter = Label::dynamic(|data: &AppState, _env: &_| data.book.get_chapter_number().clone().to_string());
    let dynamic_page_number = Label::dynamic(|data: &AppState, _env: &_| data.book.get_current_page().clone().to_string());
    
    let edit_button = Button::from_label(
        Label::dynamic(|data: &AppState, _env: &_| format!("Edit is: {}", data.is_editing.to_string()))
    ).on_click(
        |ctx, data: &mut AppState, _env| {
            data.is_editing = !data.is_editing;

            // text is the "old page"
            if data.is_editing {
                data.text = data.book.get_page_of_chapter().clone();
            }

            ctx.request_paint();
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
        
        if !data.is_editing {
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
        }
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

    let app_state = AppState {
        single_view: true,
        is_editing: false,
        book: Book::new(paths[0]),
        text: String::from("Questa è la storia di uno di noi")
    };
    
    AppLauncher::with_window(WindowDesc::new(|| build_ui())).launch(app_state)?;
    Ok(())
}
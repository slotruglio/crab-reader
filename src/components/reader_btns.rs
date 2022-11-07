use druid::widget::{Button, Label, Align};
use druid::{
    Widget,
    WidgetExt, EventCtx,
};

use crate::components::library::GUILibrary;
use crate::utils::button_functions::{change_page, edit_btn_fn, save_btn_fn, undo_btn_fn, page_number_switch_button};
use crate::CrabReaderState;

use super::book::{BookReading, GUIBook};

pub enum ReaderBtn {
    Leave,
    Edit,
    Save,
    Undo,
    NextPage,
    PrevPage,
    ViewsSwitch,
    PageNumberSwitch,
}

impl ReaderBtn {
    /// Returns a button with the correct label and function
    pub fn get_btn(&self) -> impl Widget<CrabReaderState> {
        match self {
            ReaderBtn::Leave => leave_btn(),
            ReaderBtn::Edit => edit_btn(),
            ReaderBtn::Save => save_btn(),
            ReaderBtn::Undo => undo_btn(),
            ReaderBtn::NextPage => next_btn(),
            ReaderBtn::PrevPage => back_btn(),
            ReaderBtn::ViewsSwitch => views_btn(),
            ReaderBtn::PageNumberSwitch => pages_number_btn(),
        }
    }
}

// button that let to go in library view
fn leave_btn() -> Align<CrabReaderState> {
    let leave_btn = Button::new("Go back to Browsing")
    .on_click(|_, data: &mut CrabReaderState, _| {
        data.reading = false;
    })
    .fix_height(64.0)
    .center();

    leave_btn
}

//* EDIT SECTION START */

// button that let to go to edit mode
fn edit_btn() -> Align<CrabReaderState> {
    let edit_btn = Button::new("Edit")
        .on_click(|_, data: &mut CrabReaderState, _| {
            println!("DEBUG: PRESSED EDIT BUTTON");

            edit_btn_fn(
                &mut data.reading_state, 
                data.library.get_selected_book().unwrap()
            );

        })
        .fix_height(64.0)
        .center();

    edit_btn
}

// button that let to go to save edited page
fn save_btn() -> Align<CrabReaderState> {
    let save_changes_btn = Button::new("Save")
    .on_click(|ctx: &mut EventCtx, data: &mut CrabReaderState, _| {
        println!("DEBUG: PRESSED SAVE BUTTON");

        save_btn_fn(
            ctx,
            &mut data.reading_state,
            &mut data.library.get_selected_book_mut().unwrap()
        );

    })
    .center();

    save_changes_btn
}

// button that let to go to undo last edit
fn undo_btn() -> Align<CrabReaderState> {
    let undo_changes_btn = Button::new("Undo")
    .on_click(|_, data: &mut CrabReaderState, _| {

        undo_btn_fn(&mut data.reading_state);

    })
    .center();

    undo_changes_btn
}

//* EDIT SECTION END */


// button that let to go to next page of book
fn next_btn() -> Align<CrabReaderState> {
    let next_btn = Button::new("Next")
    .on_click(|ctx, data: &mut CrabReaderState, _| {
        println!("DEBUG: PRESSED NEXT START");
        let book = data.library.get_selected_book_mut().unwrap();
        change_page(
            ctx, 
            book, 
            data.reading_state.is_editing.unwrap(), 
            data.reading_state.single_view.unwrap(), 
            true
        );
        println!("DEBUG: PRESSED NEXT END\n");
    })
    .center();

    next_btn
}

// button that let to go to previous page of book
fn back_btn() -> Align<CrabReaderState> {
    let back_btn = Button::new("Back")
        .on_click(|ctx, data: &mut CrabReaderState, _| {
            println!("DEBUG: PRESSED BACK START");
            let book = data.library.get_selected_book_mut().unwrap();
            change_page(
                ctx, 
                book, 
                data.reading_state.is_editing.unwrap(), 
                data.reading_state.single_view.unwrap(), 
                false
            );

            println!("DEBUG: PRESSED BACK END\n");
        })
        .center();

    back_btn
}

// button that let to switch between single and double page view
fn views_btn() -> Align<CrabReaderState> {
    let views_btn = Button::new("Single/Double View")
        .on_click(|_, data: &mut CrabReaderState, _| {
            data.reading_state.single_view = Some(!data.reading_state.single_view.unwrap())
        })
        .fix_height(64.0)
        .center();
    views_btn
}

// button that let to see page number with different views
fn pages_number_btn() -> Align<CrabReaderState> {

    let pages_number_label = Label::dynamic(
        |data: &CrabReaderState, _env: &_| {
            let page_number = data.library.get_selected_book().unwrap().get_cumulative_current_page_number();
            let chapter_page_number = data.library.get_selected_book().unwrap().get_current_page_number();
            match data.reading_state.pages_btn_style.unwrap() {
                1 => {
                    let pages_to_end = data.library.get_selected_book().unwrap().get_last_page_number() - chapter_page_number;
                    format!("Pages to end of chatpter: {}", pages_to_end.to_string())
                },
                2 => {
                    let pages_to_end = data.library.get_selected_book().unwrap().get_number_of_pages() - page_number;
                    format!("Pages to end of book: {}", pages_to_end.to_string())
                },
                _ => {
                    let odd = page_number % 2;
                    if data.reading_state.single_view.unwrap() {
                        format!("Page {}", page_number.to_string())
                    } else {
                        if odd == 0 {
                            format!("Pages {}-{}", page_number.to_string(), (page_number + 1).to_string())
                        } else {
                            format!("Pages {}-{}", (page_number - 1).to_string(), page_number.to_string())
                        }
                    }
                }

            }
        }
    ).with_text_size(12.0);

    let pages_number_btn = Button::from_label(pages_number_label)
    .on_click(|_, data: &mut CrabReaderState, _| {

        page_number_switch_button(&mut data.reading_state);

    }).center();

    pages_number_btn
}

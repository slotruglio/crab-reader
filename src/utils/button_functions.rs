use crate::{
    components::book::{Book, BookManagement, BookReading},
    utils::saveload, ReadingState,
};
use druid::EventCtx;

/// Activate or deactivate editing mode
/// return the new value of is_editing
/// and the new value of attribute text
#[allow(dead_code)]
pub fn edit_button(
    reading_state: &mut ReadingState,
    book: &Book,
) {
    if !reading_state.is_editing.unwrap() && reading_state.single_view.unwrap() {
        reading_state.is_editing = Some(true);
        reading_state.text = book.get_page_of_chapter().to_string();
    } else {
        println!("DEBUG: EDIT BUTTON DISABLED");
    }
}

/// Go to the next or previous page of the book
#[allow(dead_code)]
pub fn change_page(
    ctx: &mut EventCtx,
    book: &mut Book,
    is_editing: bool,
    single_view: bool,
    next: bool,
) {
    if !is_editing {
        let mut increaser = if single_view { 1 } else { 2 };

        if !next {
            increaser = -increaser;
        }

        let new_page = book.get_current_page_number() as isize + increaser;
        if new_page > book.get_last_page_number() as isize {
            book.set_chapter_number(book.get_chapter_number() + 1, true);
            println!("DEBUG: Last page of chapter, changing chapter");
        } else if new_page < 0 {
            book.set_chapter_number(book.get_chapter_number() - 1, false);
            println!("DEBUG: First page of chapter, changing chapter");
        } else {
            book.set_chapter_current_page_number(new_page as usize);
            println!("DEBUG: Changing page of chapter");
        }
        // function to save the page that the user is reading
        saveload::save_page_of_chapter(
            book.get_path().to_string(),
            book.get_chapter_number(),
            book.get_current_page_number(),
        )
        .unwrap();
        ctx.request_paint();
        println!("DEBUG: Chapter: {}", book.get_chapter_number());
    }
}

pub fn save_button(
    ctx: &mut EventCtx,
    reading_state: &mut ReadingState,
    book: &mut Book,
) {
    if reading_state.text != book.get_page_of_chapter().to_string() {
        book.edit_text(reading_state.text.clone());
        reading_state.is_editing = Some(false);
        reading_state.text = String::default();
        ctx.request_paint();
    } else {
        println!("DEBUG: NO CHANGES MADE");
    }
}

pub fn undo_button(
    reading_state: &mut ReadingState
) {
    reading_state.is_editing = Some(false);
    reading_state.text = String::default();
}
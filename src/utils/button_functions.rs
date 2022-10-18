use crate::{
    components::book::{Book, BookManagement, BookReading},
    utils::saveload,
};
use druid::EventCtx;

/// Activate or deactivate editing mode
/// return the new value of is_editing
/// and the new value of attribute text
pub fn edit_button(
    ctx: &mut EventCtx,
    book: &mut Book,
    text: String,
    is_editing: bool,
) -> (bool, String) {
    let mut is_editing = !is_editing;

    let mut text = text;

    // text is the "old page"
    if is_editing {
        text = *book.get_page_of_chapter().clone();
    }

    ctx.request_paint();
    (is_editing, text)
}

/// Go to the next page of the book
pub fn change_page(
    ctx: &mut EventCtx,
    book: &mut Book,
    is_editing: bool,
    single_view: bool,
    next: bool,
) {
    if !is_editing {
        let mut increaser = match single_view {
            true => 1,
            false => 2,
        };

        if !next {
            increaser = -increaser;
        }

        let new_page = book.get_current_page_number() as isize + increaser;

        if new_page > book.get_last_page_number() as isize {
            book.set_chapter_number(book.get_chapter_number() + 1);
            println!("DEBUG: Last page of chapter, changing chapter");
        } else if new_page < 0 {
            book.set_chapter_number(book.get_chapter_number() - 1);
            println!("DEBUG: First page of chapter, changing chapter");
        } else {
            book.set_chapter_current_page_number(new_page as usize);
            println!("DEBUG: Changing page of chapter");
        }
        // function to save the page that the user is reading
        saveload::save_page_of_chapter(
            *book.get_path().clone(),
            book.get_chapter_number(),
            book.get_current_page_number(),
        )
        .unwrap();
        ctx.request_paint();
        println!("DEBUG: Chapter: {}", book.get_chapter_number());
    }
}

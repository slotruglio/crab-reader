use crate::components::library::GUILibrary;
use crate::utils::button_functions::{
    change_page, edit_btn_fn, page_number_switch_button, save_btn_fn, undo_btn_fn,
};
use crate::CrabReaderState;
use druid::widget::{Align, Label};
use druid::{Color, WidgetExt};

use super::book::{BookReading, GUIBook};
use super::rbtn::RoundedButton;

#[allow(unused)]
pub enum ReaderBtn {
    Leave,
    Edit,
    Save,
    Undo,
    NextPage,
    PrevPage,
    ViewsSwitch,
    PageNumberSwitch,
    ChaptersList,
}

impl ReaderBtn {
    /// Returns a button with the correct label and function
    pub fn button(&self) -> RoundedButton<CrabReaderState> {
        match self {
            ReaderBtn::Leave => leave_btn(),
            ReaderBtn::Edit => edit_btn(),
            ReaderBtn::Save => save_btn(),
            ReaderBtn::Undo => undo_btn(),
            ReaderBtn::NextPage => next_btn(),
            ReaderBtn::PrevPage => back_btn(),
            ReaderBtn::ViewsSwitch => views_btn(),
            ReaderBtn::PageNumberSwitch => pages_number_btn(),
            ReaderBtn::ChaptersList => chapters_list_btn(),
        }
    }
}

// button that let to go in library view
fn leave_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("Torna a selezione libro")
        .with_on_click(|_, data: &mut CrabReaderState, _| {
            data.reading = false;
        })
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(24.0)
}

//* EDIT SECTION START */
// button that let to go to edit mode
fn edit_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::dynamic(|data: &CrabReaderState, _env: &_| {
        if data.reading_state.is_editing {
            "Termina modifica".into()
        } else {
            "Modifica testo".into()
        }
    })
    .with_color(Color::rgb8(70, 70, 70))
    .with_hot_color(Color::rgb8(50, 50, 50))
    .with_active_color(Color::rgb8(20, 20, 20))
    .with_text_color(Color::WHITE)
    .with_text_size(24.0)
    .with_on_click(|_, data: &mut CrabReaderState, _| {
        if data.reading_state.is_editing {
            data.reading_state.is_editing = false;
            // Chiedeere a sam se sta cosa è ok
            undo_btn_fn(&mut data.reading_state);
        } else {
            data.reading_state.single_view = true;
            edit_btn_fn(
                &mut data.reading_state,
                data.library.get_selected_book().unwrap(),
            );
        }
    })
}

// button that let to go to save edited page
fn save_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("Salva modifiche")
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(18.0)
        .with_on_click(|ctx, data: &mut CrabReaderState, _| {
            save_btn_fn(
                ctx,
                &mut data.reading_state,
                &mut data.library.get_selected_book_mut().unwrap(),
            );
        })
}

// button that let to go to undo last edit
fn undo_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("Annulla modifiche")
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(18.0)
        .with_on_click(|_, data: &mut CrabReaderState, _| {
            undo_btn_fn(&mut data.reading_state);
        })
}

//* EDIT SECTION END */
// button that let to go to next page of book
fn next_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("Prossima pagina")
        .with_on_click(|ctx, data: &mut CrabReaderState, _| {
            let book = data.library.get_selected_book_mut().unwrap();
            change_page(
                ctx,
                book,
                data.reading_state.is_editing,
                data.reading_state.single_view,
                true,
            );
        })
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(18.0)
}

// button that let to go to previous page of book
fn back_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("Pagina precedente")
        .with_on_click(|ctx, data: &mut CrabReaderState, _| {
            let book = data.library.get_selected_book_mut().unwrap();
            change_page(
                ctx,
                book,
                data.reading_state.is_editing,
                data.reading_state.single_view,
                false,
            );
        })
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(18.0)
}

// button that let to switch between single and double page view
fn views_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::dynamic(|data: &CrabReaderState, _env: &_| {
        if data.reading_state.single_view {
            "Attiva doppia pagina".into()
        } else {
            "Attiva singola pagina".into()
        }
    })
    .with_on_click(|_, data: &mut CrabReaderState, _| {
        data.reading_state.single_view = !data.reading_state.single_view;
    })
    .with_color(Color::rgb8(70, 70, 70))
    .with_hot_color(Color::rgb8(50, 50, 50))
    .with_active_color(Color::rgb8(20, 20, 20))
    .with_text_color(Color::WHITE)
    .with_text_size(24.0)
}

// button that let to see page number with different views
fn pages_number_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::dynamic(|data: &CrabReaderState, _env: &_| {
        let page_number = data
            .library
            .get_selected_book()
            .unwrap()
            .get_cumulative_current_page_number();
        let chapter_page_number = data
            .library
            .get_selected_book()
            .unwrap()
            .get_current_page_number();
        match data.reading_state.pages_btn_style {
            1 => {
                let pages_to_end = data
                    .library
                    .get_selected_book()
                    .unwrap()
                    .get_last_page_number()
                    - chapter_page_number;
                format!("Pages to end of chatpter: {}", pages_to_end.to_string())
            }
            2 => {
                let pages_to_end = data
                    .library
                    .get_selected_book()
                    .unwrap()
                    .get_number_of_pages()
                    - page_number;
                format!("Pages to end of book: {}", pages_to_end.to_string())
            }
            _ => {
                let odd = page_number % 2;
                if data.reading_state.single_view {
                    format!("Page {}", page_number.to_string())
                } else {
                    if odd == 0 {
                        format!(
                            "Pages {}-{}",
                            page_number.to_string(),
                            (page_number + 1).to_string()
                        )
                    } else {
                        format!(
                            "Pages {}-{}",
                            (page_number - 1).to_string(),
                            page_number.to_string()
                        )
                    }
                }
            }
        }
    })
    .with_text_size(12.0)
    .with_on_click(|_, data: &mut CrabReaderState, _| {
        page_number_switch_button(&mut data.reading_state);
    })
}

fn chapters_list_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("Chapters")
        .with_on_click(|_, data: &mut CrabReaderState, _| {
            data.reading_state.sidebar_open = !data.reading_state.sidebar_open;
        })
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(18.0)
}

pub fn chapter_label(number: usize) -> Align<CrabReaderState> {
    Label::new(format!("Chapter {}", number))
        .on_click(move |ctx, data: &mut CrabReaderState, _| {
            let book = data.library.get_selected_book_mut().unwrap();
            book.set_chapter_number(number, true);
        })
        .padding(5.0)
        .center()
}

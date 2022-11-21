use crate::components::library::GUILibrary;
use crate::utils::button_functions::{
    change_page, edit_btn_fn, page_number_switch_button, save_btn_fn, undo_btn_fn,
};
use crate::CrabReaderState;
use crate::utils::envmanager::FontSize;
use druid::commands::SHOW_OPEN_PANEL;
use druid::widget::{Align, Label};
use druid::{WidgetExt, Command, FileSpec, FileDialogOptions, Target};

use super::book::{BookReading, GUIBook, Book};
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
    Ocr,
}

enum PageCounterStyle {
    CUMULATIVE,
    ENDOFCHAPTER,
    ENDOFBOOK
}
impl PageCounterStyle {
    fn to_string(&self, book: &Book, single_view: bool) -> String {
        let page_number = book.get_cumulative_current_page_number();

        match self {
            PageCounterStyle::ENDOFCHAPTER => {
                let chapter_page_number = book.get_current_page_number();
                let pages_to_end = book.get_last_page_number() - chapter_page_number;
                format!("Pages to end of chapter: {}", pages_to_end.to_string())
            },
            PageCounterStyle::ENDOFBOOK => {
                let pages_to_end = book.get_number_of_pages() - page_number;
                format!("Pages to end of book: {}", pages_to_end.to_string())
            },
            PageCounterStyle::CUMULATIVE => {
                let odd = page_number % 2;
                if single_view {
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
            },
        }
    }
}
impl From<u8> for PageCounterStyle {
    fn from(value: u8) -> Self {
        match value {
            0 => PageCounterStyle::CUMULATIVE,
            1 => PageCounterStyle::ENDOFCHAPTER,
            2 => PageCounterStyle::ENDOFBOOK,
            _ => PageCounterStyle::CUMULATIVE,
        }
    }
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
            ReaderBtn::Ocr => ocr_btn(),
        }
    }
}

// button that let to go in library view
fn leave_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("Torna a selezione libro")
        .with_on_click(|_, data: &mut CrabReaderState, _| {
            data.reading = false;
        })
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
    .with_text_size(24.0)
}

// button that let to go to save edited page
fn save_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("Salva modifiche")
        .with_on_click(|ctx, data: &mut CrabReaderState, _| {
            save_btn_fn(
                ctx,
                &mut data.reading_state,
                &mut data.library.get_selected_book_mut().unwrap(),
            );
        })
        .with_text_size(18.0)
}

// button that let to go to undo last edit
fn undo_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("Annulla modifiche")
        .with_on_click(|_, data: &mut CrabReaderState, _| {
            undo_btn_fn(&mut data.reading_state);
        })
        .with_text_size(18.0)
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
        .disabled_if(|data: &CrabReaderState, _env: &_| {
            let book = data.library.get_selected_book().unwrap();
            let last_page = book.get_number_of_pages() - 1;
            // if last page -> disable button
            book.get_cumulative_current_page_number() == last_page
        })
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
        .disabled_if(|data: &CrabReaderState, _env: &_|{
            let book = data.library.get_selected_book().unwrap();
            // if first page -> disable button
            book.get_cumulative_current_page_number() == 0
        })
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
    .with_text_size(24.0)
}

// button that let to see page number with different views
fn pages_number_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::dynamic(|data: &CrabReaderState, _env: &_| {

        PageCounterStyle::from(data.reading_state.pages_btn_style)
        .to_string(
            data.library
            .get_selected_book()
            .unwrap(),
            data.reading_state.single_view
        )
    })
    .with_on_click(|_, data: &mut CrabReaderState, _| {
        page_number_switch_button(&mut data.reading_state);
    })
    .with_text_size(FontSize::SMALL.to_f64())
}

fn chapters_list_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("Chapters")
        .with_on_click(|_, data: &mut CrabReaderState, _| {
            data.reading_state.sidebar_open = !data.reading_state.sidebar_open;
        })
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

pub fn ocr_btn() -> RoundedButton<CrabReaderState> {
    RoundedButton::from_text("OCR")
        .with_on_click(|ctx, _: &mut CrabReaderState, _| {
            //Trigger a FILE PICKER
            let cmd = Command::new(
                SHOW_OPEN_PANEL,
                FileDialogOptions::new().allowed_types(vec![FileSpec::JPG, FileSpec::PNG]),
                Target::Auto,
            );

            ctx.submit_command(cmd);
        })
        .with_text_size(24.0)
}
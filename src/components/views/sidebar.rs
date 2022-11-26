use std::f32::consts::E;

use druid::{Widget, UnitPoint, widget::{Flex, Either, Scroll, TextBox, MainAxisAlignment}, LensExt, WidgetExt};

use crate::{CrabReaderState, components::{buttons::{rbtn::RoundedButton, reader_btns::ReaderBtn}, chapter_selector::ChapterSelector, note_widget::{get_notes_list}}, ReadingState, traits::{gui::GUILibrary, note::NoteManagement, reader::{BookReading, BookManagement}}, utils::colors};

pub enum Sidebar {
    LEFT,
    RIGHT
}

impl Sidebar {
    pub fn get(&self) -> impl Widget<CrabReaderState> {
        match self {
            Sidebar::LEFT => left_sidebar_widget(),
            Sidebar::RIGHT => right_sidebar_widget(),
        }
    }
}

fn left_sidebar_widget() -> Flex<CrabReaderState> {
    let views_btn = ReaderBtn::ViewsSwitch.button();

    let btn = RoundedButton::dynamic(|data: &ReadingState, _env: &_| {
        if !data.sidebar_open {
            "Apri selezione capitoli".into()
        } else {
            "Chiudi selezione capitoli".into()
        }
    })
    .with_on_click(|ctx, data: &mut ReadingState, _env| {
        data.sidebar_open = !data.sidebar_open;
    })
    .with_text_size(18.0)
    .align_horizontal(UnitPoint::CENTER)
    .lens(CrabReaderState::reading_state);

    let sidebar_closed = Flex::column();

    let cs = ChapterSelector::new().lens(CrabReaderState::library);
    let sidebar_open = Flex::column().with_child(cs);

    let sidebar = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.sidebar_open,
        Scroll::new(sidebar_open).vertical(),
        sidebar_closed,
    );

    Flex::column()
        .with_child(views_btn)
        .with_default_spacer()
        .with_child(btn)
        .with_default_spacer()
        .with_flex_child(sidebar, 1.0)
}

fn right_sidebar_widget() -> Flex<CrabReaderState> {
    let ocr_btn = ReaderBtn::Ocr.button()
        .expand_width();

    let ocr_inverse_btn = ReaderBtn::OcrInverse.button()
        .expand_width();


    // list of notes
    let notes = Scroll::new(get_notes_list()).vertical().expand();

    let tb = TextBox::multiline()
        .with_placeholder("Scrivi...")
        .with_text_color(colors::ON_BACKGROUND)
        .lens(CrabReaderState::reading_state.then(ReadingState::notes))
        .expand_width();
    
    let add_note = RoundedButton::from_text("Aggiungi nota")
        .disabled_if(|data: &CrabReaderState, _env: &_| data.library.get_selected_book().unwrap().get_notes().len() > 0)
        .with_on_click(|_, data: &mut CrabReaderState, _| {
            let book = data.library.get_selected_book().unwrap().clone();
            let note = data.reading_state.notes.clone();
            data.reading_state.notes = "".into();
            data.library.get_selected_book_mut().unwrap().get_notes_mut().add_note(&book, note);
        });

    let del_notes = RoundedButton::from_text("Elimina note")
        .disabled_if(|data: &CrabReaderState, _env: &_| data.library.get_selected_book().unwrap().get_notes().len() == 0)
        .with_on_click(|_, data: &mut CrabReaderState, _| {
            let book = data.library.get_selected_book().unwrap();
            let book_path = book.get_path().clone();
            let chapter = book.get_chapter_number();
            let page = book.get_current_page_number();

            data.library.get_selected_book_mut().unwrap().get_notes_mut().delete_notes(book_path, chapter, page);
        });
    
    Flex::column()
        .must_fill_main_axis(true)
        .with_child(ocr_btn)
        .with_default_spacer()
        .with_child(ocr_inverse_btn)
        .with_default_spacer()
        .with_flex_child(notes,2.0)
        .with_flex_spacer(1.0)
        .with_child(tb)
        .with_default_spacer()
        .with_child(add_note)
        .with_default_spacer()
        .with_child(del_notes)

}

use druid::widget::{Flex, Label, LineBreaking, Scroll, TextBox, Container, Either};
use druid::{Widget, WidgetExt, LensExt, TextAlignment};


use crate::{CrabReaderState, ReadingState};

use super::book::{BookReading, GUIBook};
use super::library::GUILibrary;
use super::reader_btns::ReaderBtn;

pub enum ReaderView {
    Single,
    SingleEdit,
    Dual,
    DualEdit,
}

impl ReaderView {
    pub fn get_view(&self) -> impl Widget<CrabReaderState> {
        match self {
            ReaderView::Single => single_view_widget(),
            ReaderView::SingleEdit => single_view_edit_widget(),
            ReaderView::Dual => dual_view_widget(),
            ReaderView::DualEdit => dual_view_edit_widget(),
        }
    }
    /// Returns a widget with the correct widget to show page(s) in reading or edit mode
    pub fn dynamic_view() -> impl Widget<CrabReaderState> {
        Either::new(
            |data: &CrabReaderState, _env| data.reading_state.single_view.unwrap(),
            Either::new(
                |data: &CrabReaderState, _env| data.reading_state.is_editing.unwrap(),
                ReaderView::SingleEdit.get_view(),
                ReaderView::Single.get_view()
            ),
            Either::new(
                |data: &CrabReaderState, _env| data.reading_state.is_editing.unwrap(),
                ReaderView::DualEdit.get_view(),
                ReaderView::Dual.get_view()
            )
        )
        .padding(10.0)
        .fix_size(800.0, 450.0)
    }

}

// single page view for text reader
fn single_view_widget() -> Container<CrabReaderState> {
    let view = Scroll::new(
        Label::dynamic(|data: &CrabReaderState, _env: &_| {
            data.library.get_selected_book().unwrap().get_page_of_chapter().to_string()
        })
        .with_text_alignment(TextAlignment::Justified)
        .with_line_break_mode(LineBreaking::WordWrap)
    )
    .vertical();

    Container::new(view)
    
}

// single page view for text editing
fn single_view_edit_widget() -> Container<CrabReaderState> {

    let text_box = TextBox::multiline()
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0));

    let view = Scroll::new(
        text_box.fix_size(500.0, 500.0)
        
    )
    .vertical();

    Container::new(view)
}

// dual page view for text reader
fn dual_view_widget() -> Container<CrabReaderState> {
    let views = Flex::row()
        .with_child(
            Scroll::new(
                Label::dynamic(|data: &CrabReaderState, _env: &_| {
                    data.library.get_selected_book().unwrap().get_dual_pages().0.to_string()
                })
                .with_text_alignment(TextAlignment::Justified)
                .with_line_break_mode(LineBreaking::WordWrap)
            )
            .vertical()
            .fix_size(400.0, 300.0),
        )
        .with_spacer(20.0)
        .with_child(
            Scroll::new(
                Label::dynamic(|data: &CrabReaderState, _env: &_| {
                    data.library.get_selected_book().unwrap().get_dual_pages().1.to_string()
                })
                .with_text_alignment(TextAlignment::Justified)
                .with_line_break_mode(LineBreaking::WordWrap)
            )
            .vertical()
            .fix_size(400.0, 300.0),
        );

    Container::new(views)
}

// dual page view for text editing
fn dual_view_edit_widget() -> Container<CrabReaderState> {

    let text_box_page_0 = TextBox::multiline()
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0));

    let text_box_page_1 = TextBox::multiline()
    .with_placeholder("Text editing is not yet implemented")
    .lens(CrabReaderState::reading_state.then(ReadingState::text_1));
    

    let views = Flex::row()
        .with_child(
            Scroll::new(
                text_box_page_0.fix_size(500.0, 500.0)
            )
            .vertical()
        )
        .with_spacer(10.0)
        .with_child(
            Scroll::new(
                text_box_page_1.fix_size(500.0, 500.0)
            )
            .vertical()
        );
        Container::new(views)
}

pub fn title_widget() -> impl Widget<CrabReaderState> {
    Label::dynamic(
        |data: &CrabReaderState, _env: &_| data.library.get_selected_book().unwrap().get_title().to_string(),
    )
    .with_line_break_mode(LineBreaking::Clip)
    .with_text_size(32.0)
    .padding(10.0)
    .center()
}

pub fn current_chapter_widget() -> impl Widget<CrabReaderState> {
    Label::dynamic(
        |data: &CrabReaderState, _env: &_| format!("Chapter {}",data.library.get_selected_book().unwrap().get_chapter_number().to_string())
    )
        .with_text_size(16.0)
        .padding(10.0)
        .center()
}
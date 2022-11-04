use druid::widget::{Flex, Label, LineBreaking, Scroll};
use druid::{Widget, WidgetExt};


use crate::CrabReaderState;

use super::book::BookReading;
use super::library::GUILibrary;

// single page view for text reader
#[allow(dead_code)]
pub fn build_single_view() -> impl Widget<CrabReaderState> {
    Scroll::new(
        Label::dynamic(|data: &CrabReaderState, _env: &_| {
            data.library.get_selected_book().unwrap().get_page_of_chapter().to_string()
        })
        .with_line_break_mode(LineBreaking::WordWrap)
    )
    .vertical()
}

// single page view for text editing
#[allow(dead_code)]
pub fn build_single_view_edit() -> impl Widget<CrabReaderState> {
    Scroll::new(
        // TextBox::multiline()
        // .fix_size(400.0, 300.0)
        // .lens(AppState::book.then(Book::chapter_page_text)),
        Label::new(
            "Text editing is not yet implemented"
        )
        .fix_size(400.0, 300.0)
    )
    .vertical()
}

// dual page view for text reader
#[allow(dead_code)]
pub fn build_dual_view() -> impl Widget<CrabReaderState> {
    Flex::row()
        .with_child(
            Scroll::new(
                Label::dynamic(|data: &CrabReaderState, _env: &_| {
                    data.library.get_selected_book().unwrap().get_dual_pages().0.to_string()
                })
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
                .with_line_break_mode(LineBreaking::WordWrap)
            )
            .vertical()
            .fix_size(400.0, 300.0),
        )
}
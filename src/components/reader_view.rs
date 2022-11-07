use druid::widget::{Flex, Label, LineBreaking, Scroll, TextBox};
use druid::{Widget, WidgetExt, LensExt};


use crate::{CrabReaderState, ReadingState};

use super::book::{BookReading};
use super::library::GUILibrary;

// single page view for text reader
#[allow(dead_code)]
pub fn single_view_widget() -> impl Widget<CrabReaderState> {
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
pub fn single_view_edit_widget() -> impl Widget<CrabReaderState> {

    let text_box = TextBox::multiline()
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0));

    Scroll::new(
        text_box.fix_size(500.0, 500.0)
        
    )
    .vertical()
}

// dual page view for text reader
#[allow(dead_code)]
pub fn dual_view_widget() -> impl Widget<CrabReaderState> {
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

// dual page view for text editing
#[allow(dead_code)]
pub fn dual_view_edit_widget() -> impl Widget<CrabReaderState> {

    let text_box_page_0 = TextBox::multiline()
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0));

    let text_box_page_1 = TextBox::multiline()
    .with_placeholder("Text editing is not yet implemented")
    .lens(CrabReaderState::reading_state.then(ReadingState::text_1));
    

    Flex::row()
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
        ).center()
        
}
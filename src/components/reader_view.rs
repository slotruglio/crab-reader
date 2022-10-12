use druid::{Widget, WidgetExt, LensExt};
use druid::widget::{Scroll, Flex, Label, TextBox, LineBreaking};

use crate::AppState;

use super::book::Book;

// single page view for text reader
pub fn build_single_view() -> impl Widget<AppState> {
    Scroll::new(Label::dynamic(|data: &AppState, _env: &_| data.book.get_page_of_chapter().clone()).with_line_break_mode(LineBreaking::WordWrap).fix_size(400.0, 300.0)).vertical()
}

// single page view for text editing
pub fn build_single_view_edit() -> impl Widget<AppState> {
    Scroll::new(
        TextBox::multiline()
        .fix_size(400.0, 300.0)
        .lens(AppState::book.then(Book::chapter_page_text))
    )
    .vertical()
}

// dual page view for text reader
pub fn build_dual_view() -> impl Widget<AppState> {
    Flex::row()
        .with_child(
            Scroll::new(
                Label::dynamic(
                    |data: &AppState, _env: &_| 
                    data.book.get_dual_pages().0.clone()
                ).with_line_break_mode(LineBreaking::WordWrap)
                .fix_size(400.0, 300.0))
                .vertical()
            )
        .with_child(
            Scroll::new(
                Label::dynamic(
                    |data: &AppState, _env: &_| 
                    data.book.get_dual_pages().1.clone()
                ).with_line_break_mode(LineBreaking::WordWrap)
                .fix_size(400.0, 300.0))
                .vertical()
            )
}
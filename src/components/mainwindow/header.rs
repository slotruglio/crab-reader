use druid::{
    widget::{Flex, Label, LineBreaking},
    Env, UnitPoint, WidgetExt,
};

use super::window::CrabReaderWindowState;

pub struct Header;

impl Header {
    pub fn build() -> Flex<CrabReaderWindowState> {
        let mut left_label_inner = Label::dynamic(|data: &CrabReaderWindowState, _: &Env| {
            format!("Bentornato, {}", data.username).into()
        });
        left_label_inner.set_line_break_mode(LineBreaking::WordWrap);

        let mut right_label_inner = Label::dynamic(|data: &CrabReaderWindowState, _: &Env| {
            format!("Hai {} libri da leggere.", data.library_state.books.len()).into()
        });
        right_label_inner.set_line_break_mode(LineBreaking::WordWrap);

        let left_label = left_label_inner
            .padding(10.0)
            .align_horizontal(UnitPoint::LEFT)
            .align_vertical(UnitPoint::TOP);
        let right_label = right_label_inner
            .padding(10.0)
            .align_horizontal(UnitPoint::RIGHT)
            .align_vertical(UnitPoint::TOP);

        Flex::row()
            .with_flex_child(left_label, 1.0)
            .with_flex_child(right_label, 1.0)
    }
}

use druid::{
    widget::{Container, Flex, Label, LineBreaking, RawLabel, Scroll, TextBox, ViewSwitcher},
    Data, Env, FontDescriptor, LensExt, TextAlignment, Widget, WidgetExt,
};

use crate::{
    models::library::LibrarySelectedBookLens,
    models::rich::custom_lens::{DualPage0Lens, DualPage1Lens, SelectedPageLens},
    traits::{gui::GUILibrary, reader::BookReading},
    utils::{colors, fonts},
    CrabReaderState, ReadingState,
};

#[derive(Clone, PartialEq, Data)]
pub enum ReaderView {
    Single,
    SingleEdit,
    Dual,
    DualEdit,
}

impl ReaderView {
    pub fn get_view(&self) -> Box<dyn Widget<CrabReaderState>> {
        let font = fonts::medium;

        match self {
            ReaderView::Single => single_view_widget(font),
            ReaderView::SingleEdit => single_view_edit_widget(font),
            ReaderView::Dual => dual_view_widget(font),
            ReaderView::DualEdit => dual_view_edit_widget(font),
        }
        .boxed()
    }

    /// Returns a widget with the correct widget to show page(s) in reading or edit mode
    pub fn dynamic_view() -> impl Widget<CrabReaderState> {
        let child_picker = |data: &CrabReaderState, _env: &_| match (
            data.reading_state.single_view,
            data.reading_state.is_editing,
        ) {
            (true, true) => ReaderView::SingleEdit,
            (true, false) => ReaderView::Single,
            (false, true) => ReaderView::DualEdit,
            (false, false) => ReaderView::Dual,
        };

        let child_builder = |view: &ReaderView, _data: &CrabReaderState, _: &Env| view.get_view();
        ViewSwitcher::new(child_picker, child_builder)
            .background(colors::BACKGROUND)
            .center()
            .expand()
    }
}

// single page view for text reader
fn single_view_widget(font: FontDescriptor) -> Container<CrabReaderState> {
    let raw_label = RawLabel::new()
        .with_text_color(colors::ON_BACKGROUND)
        .with_font(font)
        .with_text_alignment(TextAlignment::Justified)
        .with_line_break_mode(LineBreaking::WordWrap)
        .lens(
            CrabReaderState::library
                .then(LibrarySelectedBookLens)
                .then(SelectedPageLens),
        )
        .expand_width();

    let inner = Scroll::new(raw_label).vertical();

    Container::new(inner)
}

// single page view for text editing
fn single_view_edit_widget(font: FontDescriptor) -> Container<CrabReaderState> {
    let tb = TextBox::multiline()
        .with_text_color(colors::ON_BACKGROUND)
        .with_font(font)
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0))
        .expand_width();

    Container::new(Scroll::new(tb).vertical())
}

// dual page view for text reader
fn dual_view_widget(font: FontDescriptor) -> Container<CrabReaderState> {
    let page_0 = RawLabel::new()
        .with_text_color(colors::ON_BACKGROUND)
        .with_font(font.clone())
        .with_text_alignment(TextAlignment::Justified)
        .with_line_break_mode(LineBreaking::WordWrap)
        .lens(
            CrabReaderState::library
                .then(LibrarySelectedBookLens)
                .then(DualPage0Lens),
        )
        .expand_width();

    let page_1 = RawLabel::new()
        .with_text_color(colors::ON_BACKGROUND)
        .with_font(font)
        .with_text_alignment(TextAlignment::Justified)
        .with_line_break_mode(LineBreaking::WordWrap)
        .lens(
            CrabReaderState::library
                .then(LibrarySelectedBookLens)
                .then(DualPage1Lens),
        )
        .expand_width();

    let inner = Flex::row()
        .with_flex_child(Scroll::new(page_0).vertical(), 1.0)
        .with_flex_spacer(0.1)
        .with_flex_child(Scroll::new(page_1).vertical(), 1.0);
    Container::new(inner)
}

// dual page view for text editing
fn dual_view_edit_widget(font: FontDescriptor) -> Container<CrabReaderState> {
    let text_box_page_0 = TextBox::multiline()
        .with_text_color(colors::ON_BACKGROUND)
        .with_font(font.clone())
        .with_placeholder("There is no page here... but you can add one!")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0));

    let text_box_page_1 = TextBox::multiline()
        .with_text_color(colors::ON_BACKGROUND)
        .with_font(font)
        .with_placeholder("There is no page here... but you can add one!")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_1));

    let inner = Flex::row()
        .with_flex_child(Scroll::new(text_box_page_0).vertical(), 1.0)
        .with_flex_spacer(0.1)
        .with_flex_child(Scroll::new(text_box_page_1).vertical(), 1.0);

    Container::new(inner)
}

pub fn current_chapter_widget() -> Label<CrabReaderState> {
    Label::dynamic(|data: &CrabReaderState, _env: &_| {
        // + 1
        let display_number = data
            .library
            .get_selected_book()
            .unwrap()
            .get_chapter_number()
            + 1;

        format!("Chapter {}", display_number)
    })
    .with_text_color(colors::ON_BACKGROUND)
}

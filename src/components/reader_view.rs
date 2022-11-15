use druid::widget::{Container, Either, Flex, Label, LineBreaking, Scroll, TextBox};
use druid::{LensExt, TextAlignment, UnitPoint, Widget, WidgetExt};

use crate::{CrabReaderState, ReadingState};

use super::book::{BookReading, GUIBook};
use super::chapter_selector::ChapterSelector;
use super::library::GUILibrary;
use super::rbtn::RoundedButton;

use crate::MYENV;

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
        let either = Either::new(
            |data: &CrabReaderState, _env| data.reading_state.single_view,
            Either::new(
                |data: &CrabReaderState, _env| data.reading_state.is_editing,
                ReaderView::SingleEdit.get_view(),
                ReaderView::Single.get_view(),
            ),
            Either::new(
                |data: &CrabReaderState, _env| data.reading_state.is_editing,
                ReaderView::DualEdit.get_view(),
                ReaderView::Dual.get_view(),
            ),
        )
        .center()
        .padding(10.0)
        .fix_size(800.0, 450.0);

        either
    }
}

// single page view for text reader
fn single_view_widget() -> Container<CrabReaderState> {
    let myenv = MYENV.lock().unwrap();
    let font = myenv.font.clone();
    let font_color = myenv.font_color.clone();

    let view = Scroll::new(
        Label::dynamic(|data: &CrabReaderState, _env: &_| {
            data.library
                .get_selected_book()
                .unwrap()
                .get_page_of_chapter()
        })
        .with_text_color(font_color)
        .with_font(font)
        .with_text_alignment(TextAlignment::Justified)
        .with_line_break_mode(LineBreaking::WordWrap),
    )
    .vertical();

    Container::new(view)
}

// single page view for text editing
fn single_view_edit_widget() -> Container<CrabReaderState> {
    let myenv = MYENV.lock().unwrap();
    let font = myenv.font.clone();
    let font_color = myenv.font_color.clone();

    let text_box = TextBox::multiline()
        .with_text_color(font_color)
        .with_font(font)
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0));

    let view = Scroll::new(text_box.fix_size(500.0, 500.0)).vertical();

    Container::new(view)
}

// dual page view for text reader
fn dual_view_widget() -> Container<CrabReaderState> {
    let myenv = MYENV.lock().unwrap();
    let font = myenv.font.clone();
    let font_color = myenv.font_color.clone();

    let views = Flex::row()
        .with_flex_child(
            Scroll::new(
                Label::dynamic(|data: &CrabReaderState, _env: &_| {
                    data.library
                        .get_selected_book()
                        .unwrap()
                        .get_dual_pages()
                        .0
                })
                .with_text_color(font_color.clone())
                .with_font(font.clone())
                .with_text_alignment(TextAlignment::Justified)
                .with_line_break_mode(LineBreaking::WordWrap),
            )
            .vertical()
            .fix_size(400.0, 300.0),
            1.0,
        )
        .with_spacer(20.0)
        .with_flex_child(
            Scroll::new(
                Label::dynamic(|data: &CrabReaderState, _env: &_| {
                    data.library
                        .get_selected_book()
                        .unwrap()
                        .get_dual_pages()
                        .1
                })
                .with_text_color(font_color)
                .with_font(font)
                .with_text_alignment(TextAlignment::Justified)
                .with_line_break_mode(LineBreaking::WordWrap),
            )
            .vertical()
            .fix_size(400.0, 300.0),
            1.0,
        );

    Container::new(views)
}

// dual page view for text editing
fn dual_view_edit_widget() -> Container<CrabReaderState> {
    let myenv = MYENV.lock().unwrap();
    let font = myenv.font.clone();
    let font_color = myenv.font_color.clone();

    let text_box_page_0 = TextBox::multiline()
        .with_text_color(font_color.clone())
        .with_font(font.clone())
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0));

    let text_box_page_1 = TextBox::multiline()
        .with_text_color(font_color)
        .with_font(font)
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_1));

    let views = Flex::row()
        .with_child(Scroll::new(text_box_page_0.fix_size(500.0, 500.0)).vertical())
        .with_spacer(10.0)
        .with_child(Scroll::new(text_box_page_1.fix_size(500.0, 500.0)).vertical());
    Container::new(views)
}

pub fn title_widget() -> impl Widget<CrabReaderState> {
    Label::dynamic(|data: &CrabReaderState, _env: &_| {
        data.library
            .get_selected_book()
            .unwrap()
            .get_title()
            .to_string()
    })
    .with_line_break_mode(LineBreaking::Clip)
    .with_text_size(32.0)
    .padding(10.0)
    .center()
}

pub fn current_chapter_widget() -> Label<CrabReaderState> {
    Label::dynamic(|data: &CrabReaderState, _env: &_| {
        // + 1
        let display_number = data.library
            .get_selected_book()
            .unwrap()
            .get_chapter_number() + 1;
            
        format!(
            "Chapter {}",
            display_number
        )
    })
}

pub fn sidebar_widget() -> impl Widget<CrabReaderState> {
    let btn = RoundedButton::dynamic(|data: &ReadingState, _env: &_| {
        if !data.sidebar_open {
            "Apri selezione capitoli".into()
        } else {
            "Chiudi selezione capitoli".into()
        }
    })
    .with_on_click(|ctx, data: &mut ReadingState, _env| {
        data.sidebar_open = !data.sidebar_open;
        ctx.request_layout();
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
        .with_child(btn)
        .with_default_spacer()
        .with_flex_child(sidebar, 1.0)
}

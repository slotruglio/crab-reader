use std::rc::Rc;

use druid::{
    widget::{Flex, Label},
    Color, Data, FontDescriptor, FontFamily, FontWeight, Lens, UnitPoint, Widget, WidgetExt,
};

use super::booklibrary::library::{Library, LibraryWidget};

#[derive(Clone, PartialEq, Lens, Data)]
pub struct CrabReaderWindowState {
    username: Rc<String>,
    num_books: String,
    pub library_state: Library,
}

fn header() -> impl Widget<CrabReaderWindowState> {
    let left_label = Label::dynamic(|data: &CrabReaderWindowState, _env: &_| {
        format!("Bentornato, {}.", data.username)
    })
    .with_text_color(Color::WHITE)
    .align_horizontal(UnitPoint::LEFT)
    .padding(5.0);

    let right_label = Label::dynamic(|data: &CrabReaderWindowState, _env: &_| {
        let nbooks = data.library_state.nbooks;
        format!("Hai {} libri disponibili.", nbooks)
    })
    .with_text_color(Color::WHITE)
    .align_horizontal(UnitPoint::RIGHT)
    .padding(5.0);

    Flex::row()
        .with_flex_child(left_label, 1.0)
        .with_flex_spacer(0.1)
        .with_flex_child(right_label, 1.0)
        .padding(10.0)
        .background(Color::GRAY)
        .rounded(7.5)
        .padding(10.0)
}

fn book_carousel(app_state: &CrabReaderWindowState) -> impl Widget<Library> {
    LibraryWidget::from(app_state.library_state.clone())
}

fn book_info_carousel() -> impl Widget<CrabReaderWindowState> {
    //todo: change this
    Flex::column()
        .with_child(
            Label::dynamic(|data: &Library, _env: &_| data.selected_book_title())
                .with_font(
                    FontDescriptor::new(FontFamily::SERIF).with_weight(FontWeight::SEMI_BOLD),
                )
                .with_text_size(24.0)
                .lens(CrabReaderWindowState::library_state),
        )
        .with_flex_child(Label::new("Other Info"), 1.0)
        .padding(5.0)
        .background(Color::GRAY)
        .rounded(7.5)
        .padding(10.0)
}

impl From<CrabReaderWindowState> for Flex<CrabReaderWindowState> {
    fn from(state: CrabReaderWindowState) -> Self {
        Flex::column().with_child(header()).with_flex_child(
            Flex::row()
                .with_flex_child(
                    book_carousel(&state)
                        .lens(CrabReaderWindowState::library_state)
                        .expand(),
                    3.0,
                )
                .with_flex_child(book_info_carousel().expand(), 1.0),
            1.0,
        )
    }
}

impl Default for CrabReaderWindowState {
    fn default() -> Self {
        Self {
            username: Rc::new("Cocco".into()),
            library_state: Library::new(),
            num_books: "".into(),
        }
    }
}

impl CrabReaderWindowState {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn build_ui(state: &CrabReaderWindowState) -> impl Widget<CrabReaderWindowState> {
    let state = state.clone();
    Flex::<CrabReaderWindowState>::from(state)
}

use std::rc::Rc;

use druid::{
    widget::{Flex, Label, Scroll},
    Color, Data, Lens, LocalizedString, UnitPoint, Widget, WidgetExt,
};

use super::booklibrary::library::{Library, LibraryWidget};

#[derive(Clone, PartialEq, Lens, Data)]
pub struct CrabReaderWindowState {
    username: Rc<String>,
    num_books: String,
    pub library_state: Library,
}

fn header() -> impl Widget<CrabReaderWindowState> {
    let left_string = LocalizedString::new("Hello").with_placeholder("Bentornato, Matteo.");
    let right_string =
        LocalizedString::new("Library").with_placeholder("Hai 420 libri disponibili");

    let left_label = Label::new(left_string).align_horizontal(UnitPoint::LEFT);
    let right_label = Label::new(right_string).align_horizontal(UnitPoint::RIGHT);

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
    let child = LibraryWidget::from(app_state.library_state.clone())
        .align_vertical(UnitPoint::TOP)
        .align_horizontal(UnitPoint::LEFT)
        .expand();

    let root = Flex::column()
        .with_flex_child(child, 1.0)
        .background(Color::GRAY)
        .rounded(7.5)
        .padding(10.0);

    root
}

fn book_info_carousel() -> impl Widget<CrabReaderWindowState> {
    //todo: change this
    Flex::column()
        .with_child(Label::new("Book Title"))
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
            username: Rc::new("".into()),
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

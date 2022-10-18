use std::rc::Rc;

use components::book::Book;
use components::book_details::BookDetails;
use components::cover_library::CoverLibrary;
use components::display_mode_button::{DisplayMode, DisplayModeButton};
use components::library::GUILibrary;
use components::listing_library::ListLibrary;
use components::mockup::{self, MockupBook, MockupLibrary};
use druid::widget::{Either, Flex, Scroll};
use druid::{AppLauncher, Color, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};
mod components;
mod utils;

type Library = MockupLibrary<MockupBook>;
use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;
use utils::envmanager::MyEnv;

//Create a global ENV variable
static MYENV: Lazy<Mutex<MyEnv>> = Lazy::new(|| Mutex::new(MyEnv::new()));

#[derive(Clone, Data, Lens)]
struct AppState {
    single_view: bool,
    is_editing: bool,
    book: Rc<Book>,
    text: Rc<String>,
}

#[derive(Clone, Data, Lens)]
struct CrabReaderState {
    user: UserState,
    library: Library,
    display_mode: DisplayMode,
}

impl Default for CrabReaderState {
    fn default() -> Self {
        Self {
            user: UserState::new(),
            library: Library::new(),
            display_mode: DisplayMode::Cover,
        }
    }
}

#[derive(Clone, Data)]
struct UserState {
    username: Rc<String>,
}

impl UserState {
    pub fn new() -> Self {
        Self {
            username: "Username".to_string(),
        }
    }
}

fn book_details_panel() -> impl Widget<CrabReaderState> {
    BookDetails::new()
        .background(Color::GRAY)
        .rounded(10.0)
        .lens(CrabReaderState::library)
}

fn build_ui() -> impl Widget<CrabReaderState> {
    let library_cover = CoverLibrary::new().lens(CrabReaderState::library);
    let library_list = ListLibrary::new().lens(CrabReaderState::library);

    let view_either = Either::new(
        |data: &CrabReaderState, _env| data.display_mode == DisplayMode::List,
        library_list.padding(5.0),
        library_cover,
    )
    .background(Color::GRAY)
    .rounded(10.0)
    .padding(10.0);

    let scroll = Scroll::new(view_either).vertical();

    let right_panel = Scroll::new(book_details_panel()).vertical().padding(5.0);
    let right_col = Flex::column()
        .with_child(
            DisplayModeButton
                .padding(10.0)
                .expand_width()
                .lens(CrabReaderState::display_mode),
        )
        .with_flex_child(right_panel, 1.0);

    let inner = Flex::row()
        .with_flex_child(scroll, 2.0)
        .with_flex_child(right_col, 1.0);

    Flex::column().with_flex_child(inner, 1.0)
}

fn main() -> Result<(), PlatformError> {
    let mut crab_state = CrabReaderState::default();

    mockup::get_mockup_book_vec().iter().for_each(|book| {
        crab_state.library.add_book(book);
    });

    AppLauncher::with_window(WindowDesc::new(build_ui).title("CrabReader")).launch(crab_state)?;
    Ok(())
}

use components::library::Library;
use druid::im::Vector;
use druid::widget::Flex;
use druid::{AppLauncher, Color, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};

mod components;
use components::book::Book;

#[derive(Clone, Data, Lens)]
struct CrabReaderState {
    user: UserState,
    books: Vector<Book>,
}

impl Default for CrabReaderState {
    fn default() -> Self {
        Self {
            user: UserState::new(),
            books: Vector::new(),
        }
    }
}

#[derive(Clone, Data)]
struct UserState {
    username: String,
}

impl UserState {
    pub fn new() -> Self {
        Self {
            username: "Username".to_string(),
        }
    }
}

fn build_ui() -> impl Widget<CrabReaderState> {
    let library = Library::new();
    Flex::row()
        .with_child(library.lens(CrabReaderState::books))
        .background(Color::GRAY)
        .rounded(5.0)
        .padding(10.0)
}

fn main() -> Result<(), PlatformError> {
    let mut crab_state = CrabReaderState::default();
    ["The Lord of the Rings", "The Hobbit", "The Silmarillion"]
        .into_iter()
        .map(|title| Book::new().with_title(title))
        .for_each(|book| crab_state.books.push_back(book));
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(crab_state)?;
    Ok(())
}

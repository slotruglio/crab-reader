use components::library::Library;
use druid::im::Vector;
use druid::widget::{Flex, Scroll};
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
    let inner = Flex::column()
        .with_child(library.lens(CrabReaderState::books))
        .background(Color::GRAY)
        .rounded(5.0)
        .padding(10.0);

    Scroll::new(inner).vertical()
}

fn main() -> Result<(), PlatformError> {
    let mut crab_state = CrabReaderState::default();
    let covers_path_names = [
        "lotr.jpg",
        "sotto-lo-stesso-cielo.jpg",
        "california-la-fine-del-sogno.jpg",
        "1984.jpg",
        "451.jpg",
        "saggio-erotico-sulla-fine-del-mondo.jpg",
    ]
    .into_iter()
    .map(|x| x.to_string());
    [
        "Il Signore degli Anelli",
        "Sotto lo stesso cielo",
        "California: La Fine del Sogno",
        "1984",
        "Farenheit 451",
        "Saggio erotico sulla fine del mondo",
    ]
    .into_iter()
    .zip(covers_path_names)
    .map(|(title, path)| Book::new().with_title(title).with_cover_path(path))
    .for_each(|book| crab_state.books.push_back(book));
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(crab_state)?;
    Ok(())
}

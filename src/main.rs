use components::library::{CoverLibrary, Library, ListLibrary};
use components::view_switcher::{SwitcherButton, ViewMode};
use druid::widget::{Either, Flex, Scroll};
use druid::{AppLauncher, Color, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};

mod components;
use components::book::Book;

#[derive(Clone, Data, Lens)]
struct CrabReaderState {
    user: UserState,
    library: Library,
    display_mode: ViewMode,
}

impl Default for CrabReaderState {
    fn default() -> Self {
        Self {
            user: UserState::new(),
            library: Library::new(),
            display_mode: ViewMode::Cover,
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
    let library_cover = CoverLibrary::new().lens(CrabReaderState::library);
    let library_list = ListLibrary::new().lens(CrabReaderState::library);

    let cover_view = Flex::column()
        .with_child(library_cover)
        .background(Color::GRAY)
        .rounded(5.0)
        .padding(10.0);

    let list_view = Flex::column()
        .with_child(library_list)
        .padding(10.0)
        .background(Color::GRAY)
        .rounded(5.0)
        .padding(10.0);

    let view_either = Either::new(
        |data: &CrabReaderState, _env| data.display_mode == ViewMode::List,
        list_view,
        cover_view,
    );

    let inner = Flex::column()
        .with_child(
            SwitcherButton
                .padding(10.0)
                .align_right()
                .lens(CrabReaderState::display_mode),
        )
        .with_child(view_either)
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
        "sugma.jpg",
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
        "Libro senza cover :)",
    ]
    .into_iter()
    .zip(covers_path_names)
    .map(|(title, path)| Book::new().with_title(title).with_cover_path(&path))
    .for_each(|book| crab_state.library.add_book(book));
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(crab_state)?;
    Ok(())
}

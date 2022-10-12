use components::library::{Library, LibraryList};
use druid::im::Vector;
use druid::widget::{Either, Flex, Label, Scroll};
use druid::{AppLauncher, Color, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};

mod components;
use components::book::Book;

#[derive(Clone, Data, PartialEq)]
enum ViewMode {
    List,
    Cover,
}

#[derive(Clone, Data, Lens)]
struct CrabReaderState {
    user: UserState,
    books: Vector<Book>,
    display_mode: ViewMode,
}

impl Default for CrabReaderState {
    fn default() -> Self {
        Self {
            user: UserState::new(),
            books: Vector::new(),
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

fn view_mode_toggle_button() -> impl Widget<CrabReaderState> {
    Label::dynamic(|data: &CrabReaderState, _env: &_| match data.display_mode {
        ViewMode::List => "Passa a Cover".into(),
        ViewMode::Cover => "Passa a Lista".into(),
    })
    .padding(10.0)
    .background(Color::GRAY)
    .rounded(10.0)
    .on_click(|ctx, data: &mut CrabReaderState, _env| {
        match data.display_mode {
            ViewMode::List => data.display_mode = ViewMode::Cover,
            ViewMode::Cover => data.display_mode = ViewMode::List,
        }
        ctx.request_layout();
    })
}

fn build_ui() -> impl Widget<CrabReaderState> {
    let library = Library::new();
    let library_list = LibraryList::new();

    let switcher = view_mode_toggle_button().padding(10.0).align_right();

    let cover_view = Flex::column()
        .with_child(library.lens(CrabReaderState::books))
        .background(Color::GRAY)
        .rounded(5.0)
        .padding(10.0);

    let list_view = Flex::column()
        .with_child(library_list.lens(CrabReaderState::books).expand())
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
        .with_child(switcher)
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
    .map(|(title, path)| Book::new().with_title(title).with_cover_path(path))
    .for_each(|book| crab_state.books.push_back(book));
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(crab_state)?;
    Ok(())
}

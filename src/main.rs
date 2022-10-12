use components::library::{Library, LibraryList};
use druid::im::Vector;
use druid::widget::{Either, Flex, Label, Scroll};
use druid::{AppLauncher, Color, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};

mod components;
use components::book::Book;

#[derive(Clone, Data, Lens)]
struct CrabReaderState {
    user: UserState,
    books: Vector<Book>,
    display_mode: bool,
}

impl Default for CrabReaderState {
    fn default() -> Self {
        Self {
            user: UserState::new(),
            books: Vector::new(),
            display_mode: true,
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

    let activate_cover_btn = Flex::row()
        .with_child(Label::new("Passa a Lista"))
        .padding(5.0)
        .background(Color::GRAY)
        .rounded(10.0)
        .on_click(|ctx, data: &mut CrabReaderState, _env| {
            data.display_mode = !data.display_mode;
        })
        .padding(10.0);

    let activate_list_btn = Flex::row()
        .with_child(Label::new("Passa a Cover"))
        .padding(5.0)
        .background(Color::GRAY)
        .rounded(10.0)
        .on_click(|ctx, data: &mut CrabReaderState, _env| {
            data.display_mode = !data.display_mode;
        })
        .padding(10.0);

    let btn_either = Either::new(
        |data: &CrabReaderState, _env| data.display_mode,
        activate_cover_btn,
        activate_list_btn,
    )
    .align_right();

    let inner = Flex::column()
        .with_child(library.lens(CrabReaderState::books))
        .background(Color::GRAY)
        .rounded(5.0)
        .padding(10.0);

    let library_list = LibraryList::new();

    let right = Flex::column()
        .with_child(library_list.lens(CrabReaderState::books).expand())
        .padding(10.0)
        .background(Color::GRAY)
        .rounded(5.0)
        .padding(10.0);

    let scroll_either = Either::new(
        |data: &CrabReaderState, _env| data.display_mode,
        inner,
        right,
    );

    Scroll::new(
        Flex::column()
            .with_child(btn_either)
            .with_spacer(5.0)
            .with_child(scroll_either),
    )
    .vertical()
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

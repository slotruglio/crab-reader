mod components;

use components::mainwindow::{
    booklibrary::{book::BookState, library::BookLibraryState},
    header::HeaderState,
    menu,
    window::CrabReaderWindowState,
};
use druid::{AppLauncher, Data, Lens, PlatformError, WindowDesc};

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    surname: String,
}

fn main() -> Result<(), PlatformError> {
    let books = vec!["One Piece", "Naruto", "Bleach"];
    let books_str = books.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let books_state = books_str
        .into_iter()
        .map(|x| BookState::new().with_title(x).get())
        .collect::<Vec<BookState>>();
    let im_books_state = druid::im::Vector::from(books_state);
    let state = CrabReaderWindowState {
        header_state: HeaderState {
            username: "Cocco".into(),
            nbooks: "69".into(),
        },
        library_state: BookLibraryState::new().with_books(im_books_state).get(),
    };
    let w = state.widget();
    let menu = menu::make_menu();
    let win = WindowDesc::new(|| w)
        .title("Crab Reader")
        .menu(menu)
        .window_size((1280., 720.));
    AppLauncher::with_window(win).launch(state)?;
    Ok(())
}

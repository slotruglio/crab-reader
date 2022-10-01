mod components;

use components::mainwindow::{
    booklibrary::{book::BookState, library::BookLibraryState},
    menu,
    window::CrabReaderWindowState,
};
use druid::{im::Vector, AppLauncher, PlatformError, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let books = vec!["One Piece", "Naruto", "Bleach"];
    let books_str = books.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let books_state: Vector<BookState> = books_str
        .into_iter()
        .map(|x| BookState::new().with_title(x).get())
        .collect();
    let state = CrabReaderWindowState {
        library_state: BookLibraryState::new().with_books(books_state),
        username: "Cocco".into(),
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

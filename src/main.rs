mod components;

use components::mainwindow::{
    booklibrary::{book::BookState, library::BookLibraryState},
    menu,
    window::CrabReaderWindowState,
};
use druid::{im::Vector, AppLauncher, PlatformError, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let books: Vector<BookState> = vec!["One Piece", "Naruto", "Bleach"]
        .iter()
        .map(|x| x.to_string())
        .map(|x| BookState::new().with_title(x))
        .collect();

    let state = CrabReaderWindowState {
        library_state: BookLibraryState::new().with_books(books),
        username: "Cocco".into(),
    };

    let root = state.widget();
    let menu = menu::make_menu();

    let window = WindowDesc::new(|| root)
        .title("Crab Reader")
        .menu(menu)
        .window_size((1280., 720.));

    AppLauncher::with_window(window).launch(state)?;
    Ok(())
}

mod components;

use components::mainwindow::{
    booklibrary::library::BookLibraryState,
    header::HeaderState,
    menu,
    window::{CrabReaderWindow, CrabReaderWindowState},
};
use druid::{AppLauncher, Data, Lens, PlatformError, WindowDesc};

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    surname: String,
}

fn main() -> Result<(), PlatformError> {
    let state = CrabReaderWindowState {
        header_state: HeaderState {
            username: "Cocco".into(),
            nbooks: "69".into(),
        },
        library_state: BookLibraryState {},
    };
    let menu = menu::make_menu();
    let win = WindowDesc::new(CrabReaderWindow::build)
        .title("Crab Reader")
        .menu(menu);
    AppLauncher::with_window(win).launch(state)?;
    Ok(())
}

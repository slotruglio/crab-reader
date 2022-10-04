mod components;

use components::mainwindow::{
    menu::make_menu,
    window::{build_ui, CrabReaderWindowState},
};
use druid::{AppLauncher, PlatformError, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let mut app_state = CrabReaderWindowState::new();
    vec!["The Hobbit", "The Lord of the Rings", "The Silmarillion"]
        .iter()
        .for_each(|book| {
            app_state.library_state.add_book(book.to_string(), 420);
        });

    let menu = make_menu();
    let root = build_ui(&app_state);

    let window = WindowDesc::new(|| root)
        .title("Crab Reader")
        .window_size((1280., 720.))
        .menu(menu);

    AppLauncher::with_window(window).launch(app_state)?;
    Ok(())
}

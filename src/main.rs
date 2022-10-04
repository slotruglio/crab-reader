mod components;

use components::mainwindow::{
    menu::make_menu,
    window::{build_ui, CrabReaderWindowState},
};
use druid::{AppLauncher, PlatformError, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let menu = make_menu();

    let window = WindowDesc::new(build_ui)
        .title("Crab Reader")
        .window_size((1280., 720.))
        .menu(menu);

    AppLauncher::with_window(window).launch(CrabReaderWindowState::default())?;
    Ok(())
}

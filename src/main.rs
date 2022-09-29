use druid::{AppLauncher, Data, Lens, PlatformError, WindowDesc};
mod window;
use window::{CrabReaderWindow, CrabReaderWindowState};

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    surname: String,
}

fn main() -> Result<(), PlatformError> {
    let state = CrabReaderWindowState {
        header_state: window::HeaderState {
            username: "Cocco".into(),
            nbooks: 69,
        },
        library_state: window::BookLibraryState {},
    };
    let win = WindowDesc::new(CrabReaderWindow::build).title("Crab Reader");
    AppLauncher::with_window(win).launch(state)?;
    Ok(())
}

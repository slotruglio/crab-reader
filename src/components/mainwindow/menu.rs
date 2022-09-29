use druid::{Command, LocalizedString, MenuDesc, MenuItem, Selector, Target};

use crate::components::mainwindow::window::CrabReaderWindowState;

pub fn make_menu() -> MenuDesc<CrabReaderWindowState> {
    type Button = MenuItem<CrabReaderWindowState>;

    let file_selector = Selector::new("main-window-menu-file");
    let file_command = Command::new(file_selector, (), Target::Auto);
    let file_button: Button = MenuItem::new(LocalizedString::new("Files"), file_command);

    let edit_selector = Selector::new("main-window-menu-edit");
    let edit_command = Command::new(edit_selector, (), Target::Auto);
    let edit_button: Button = MenuItem::new(LocalizedString::new("Edit"), edit_command);

    let view_selector = Selector::new("main-window-menu-edit");
    let view_command = Command::new(view_selector, (), Target::Auto);
    let view_button: Button = MenuItem::new(LocalizedString::new("View"), view_command);

    let about_selector = Selector::new("main-window-menu-about");
    let about_command = Command::new(about_selector, (), Target::Auto);
    let about_button: Button = MenuItem::new(LocalizedString::new("About"), about_command);

    let checked_selector = Selector::new("submenu-checked");
    let checked_command = Command::new(checked_selector, (), Target::Auto);
    let checked_button: Button =
        MenuItem::new(LocalizedString::new("Checked"), checked_command).selected();

    let disabled_selector = Selector::new("submenu-disabled");
    let disabled_command = Command::new(disabled_selector, (), Target::Auto);
    let disabled_button: Button =
        MenuItem::new(LocalizedString::new("Disabled"), disabled_command).disabled();

    // Da testare con lo stato per vedere come interagisce...

    let submenu = MenuDesc::new(LocalizedString::new("Submenu"))
        .append(checked_button)
        .append(disabled_button);

    MenuDesc::new(LocalizedString::new("Menu"))
        .append(file_button)
        .append(edit_button)
        .append(view_button)
        .append(about_button)
        .append(submenu)
}

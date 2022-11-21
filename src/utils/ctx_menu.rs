use druid::{Command, LocalizedString, MenuDesc, MenuItem, Selector, Target};

use crate::CrabReaderState;

const DUMB_SELECTOR: Selector<()> = Selector::new("dumb.selector");

#[inline]
fn loc_str(s: impl Into<String>) -> LocalizedString<CrabReaderState> {
    LocalizedString::new("---").with_placeholder(s.into())
}

#[inline]
fn entry(s: impl Into<String>) -> MenuItem<CrabReaderState> {
    MenuItem::new(
        loc_str(s),
        Command::new(DUMB_SELECTOR.into(), (), Target::Auto),
    )
}

fn file() -> MenuDesc<CrabReaderState> {
    let add_file = entry("Aggiungi un eBook");
    let rm_file = entry("Rimuovi un eBook");
    let del_cache = entry("Svuota cache");
    MenuDesc::new(loc_str("File"))
        .append(add_file)
        .append(rm_file)
        .append(del_cache)
}

fn options() -> MenuDesc<CrabReaderState> {
    MenuDesc::new(loc_str("Preferenze"))
        .append(theme())
        .append(shadows())
        .append(text())
        .append(lang())
}

fn text() -> MenuDesc<CrabReaderState> {
    let sz = text_sz();
    let font = font();
    MenuDesc::new(loc_str("Testo")).append(sz).append(font)
}

/// Returns the context menu for the main window
pub fn main_window() -> MenuDesc<CrabReaderState> {
    MenuDesc::new(LocalizedString::new("---").with_placeholder("Nome del menù, ndo se vede???"))
        .append(file())
        .append(options())
}

/*





*/

fn theme() -> MenuDesc<CrabReaderState> {
    let light = entry("Chiaro");
    let dark = entry("Scuro").selected();
    let sepia = entry("Seppia");
    MenuDesc::new(loc_str("Tema"))
        .append(light)
        .append(dark)
        .append(sepia)
}

fn shadows() -> MenuDesc<CrabReaderState> {
    let shadows_on = entry("Abilita ombre").selected();
    MenuDesc::new(loc_str("Ombre")).append(shadows_on)
}

fn text_sz() -> MenuDesc<CrabReaderState> {
    let small = entry("Piccolo");
    let medium = entry("Medio").selected();
    let large = entry("Grande");
    MenuDesc::new(loc_str("Dimensione testo"))
        .append(small)
        .append(medium)
        .append(large)
}

fn font() -> MenuDesc<CrabReaderState> {
    let font1 = entry("Default di sistema");
    let font2 = entry("Arial");
    let font3 = entry("Helvetica");
    let font4 = entry("Noto Sans");
    MenuDesc::new(loc_str("Caratteri"))
        .append(font1)
        .append(font2)
        .append(font3)
        .append(font4)
}

fn lang() -> MenuDesc<CrabReaderState> {
    let lang1 = entry("Italiano");
    let lang2 = entry("Inglese");
    let lang3 = entry("Francese");
    let lang4 = entry("Spagnolo");
    MenuDesc::new(loc_str("Lingua"))
        .append(lang1)
        .append(lang2)
        .append(lang3)
        .append(lang4)
}

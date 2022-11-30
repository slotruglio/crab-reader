use crate::{CrabReaderState, utils::fonts::FONT};
use druid::{Menu, MenuItem, Command, Target, Env, FontFamily};

use super::{colors::CrabTheme, fonts::{SET_FONT_SYSTEM_UI, SET_FONT_MONOSPACE, SET_FONT_SERIF, SET_FONT_SANS_SERIF}};

fn file() -> Menu<CrabReaderState> {
    let add_file = MenuItem::new("Aggiungi un eBook");
    let rm_file = MenuItem::new("Rimuovi un eBook");
    let del_cache = MenuItem::new("Svuota cache");
    Menu::new("File")
        .entry(add_file)
        .entry(rm_file)
        .entry(del_cache)
}

fn options() -> Menu<CrabReaderState> {
    Menu::new("Preferenze")
        .entry(theme())
        .entry(shadows())
        .entry(text())
        .entry(lang())
}

fn text() -> Menu<CrabReaderState> {
    let sz = text_sz();
    let font = font();
    Menu::new("Testo").entry(sz).entry(font)
}

/// Returns the context menu for the main window
pub fn main_window() -> Menu<CrabReaderState> {
    Menu::new("CrabMenù").entry(file()).entry(options())
}

fn theme() -> Menu<CrabReaderState> {
    let light = MenuItem::new("Chiaro")
        .on_activate(|_, data: &mut CrabReaderState, _| {
            data.theme = CrabTheme::Light;
        })
        .selected_if(|data, _| data.theme == CrabTheme::Light);
    let sepia = MenuItem::new("Sepia")
        .on_activate(|_, data: &mut CrabReaderState, _| {
            data.theme = CrabTheme::Sepia;
        })
        .selected_if(|data, _| data.theme == CrabTheme::Sepia);
    let dark = MenuItem::new("Scuro")
        .on_activate(|_, data: &mut CrabReaderState, _| {
            data.theme = CrabTheme::Dark;
        })
        .selected_if(|data, _| data.theme == CrabTheme::Dark);
    Menu::new("Tema").entry(light).entry(dark).entry(sepia)
}

fn shadows() -> Menu<CrabReaderState> {
    let shadows_on = MenuItem::new("Ombre copertine")
        .selected_if(|data: &CrabReaderState, _| data.paint_shadows)
        .on_activate(|_, data: &mut CrabReaderState, _| {
            println!("{} -> {}", data.paint_shadows, !data.paint_shadows);
            data.paint_shadows = !data.paint_shadows;
        });
    Menu::new("Ombre").entry(shadows_on)
}

fn text_sz() -> Menu<CrabReaderState> {
    let small = MenuItem::new("Piccolo");
    let medium = MenuItem::new("Medio").selected_if(|_, _| true);
    let large = MenuItem::new("Grande");
    Menu::new("Dimensione testo")
        .entry(small)
        .entry(medium)
        .entry(large)
}

fn font() -> Menu<CrabReaderState> {

    fn selected_if_default(data: &CrabReaderState, _: &Env) -> bool {
        let font = &data.font;
        font.family == FontFamily::SYSTEM_UI
    }

    fn selected_if_mono(data: &CrabReaderState, _: &Env) -> bool {
        let font = &data.font;
        font.family == FontFamily::MONOSPACE
    }

    fn selected_if_serif(data: &CrabReaderState, _: &Env) -> bool {
        let font = &data.font;
        font.family == FontFamily::SERIF
    }

    fn selected_if_sans(data: &CrabReaderState, _: &Env) -> bool {
        let font = &data.font;
        font.family == FontFamily::SANS_SERIF
    }

    let font1 = MenuItem::new("Default di sistema").selected_if(selected_if_default)
        .command(Command::new(SET_FONT_SYSTEM_UI, (), Target::Auto));
    let font2 = MenuItem::new("Monospace").selected_if(selected_if_mono)
        .command(Command::new(SET_FONT_MONOSPACE, (), Target::Auto));
    let font3 = MenuItem::new("Serif").selected_if(selected_if_serif)
        .command(Command::new(SET_FONT_SERIF, (), Target::Auto));
    let font4 = MenuItem::new("Sans Serif").selected_if(selected_if_sans)
        .command(Command::new(SET_FONT_SANS_SERIF, (), Target::Auto));
    Menu::new("Caratteri")
        .entry(font1)
        .entry(font2)
        .entry(font3)
        .entry(font4)
}

fn lang() -> Menu<CrabReaderState> {
    let lang1 = MenuItem::new("Italiano");
    let lang2 = MenuItem::new("Inglese");
    let lang3 = MenuItem::new("Francese");
    let lang4 = MenuItem::new("Spagnolo");
    Menu::new("Lingua")
        .entry(lang1)
        .entry(lang2)
        .entry(lang3)
        .entry(lang4)
}

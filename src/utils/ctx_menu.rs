use crate::CrabReaderState;
use druid::{LocalizedString, Menu, MenuItem};

use super::colors::CrabTheme;

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
        .on_activate(|ctx, data: &mut CrabReaderState, env| {
            data.theme = CrabTheme::Light;
        })
        .selected_if(|data, theme| data.theme == CrabTheme::Light);
    let sepia = MenuItem::new("Sepia")
        .on_activate(|ctx, data: &mut CrabReaderState, env| {
            data.theme = CrabTheme::Sepia;
        })
        .selected_if(|data, env| data.theme == CrabTheme::Sepia);
    let dark = MenuItem::new("Scuro")
        .on_activate(|ctx, data: &mut CrabReaderState, env| {
            data.theme = CrabTheme::Dark;
        })
        .selected_if(|data, env| data.theme == CrabTheme::Dark);
    Menu::new("Tema").entry(light).entry(dark).entry(sepia)
}

fn shadows() -> Menu<CrabReaderState> {
    let shadows_on = MenuItem::new("Ombre copertine")
        .selected_if(|data: &CrabReaderState, env| data.paint_shadows)
        .on_activate(|ctx, data: &mut CrabReaderState, env| {
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
    let font1 = MenuItem::new("Default di sistema");
    let font2 = MenuItem::new("Arial");
    let font3 = MenuItem::new("Helvetica");
    let font4 = MenuItem::new("Noto Sans");
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

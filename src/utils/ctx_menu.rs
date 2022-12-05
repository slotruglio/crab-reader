use crate::{CrabReaderState, utils::fonts::{FONT, self, SET_FONT_SMALL, SET_FONT_MEDIUM, SET_FONT_LARGE}, MYENV};
use druid::{Menu, MenuItem, Command, Target, Env, FontFamily, FontDescriptor};

use super::{colors::CrabTheme};

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
            // update env
            let mut my_env = MYENV.lock().unwrap();
            my_env.set_property(
                "theme".to_string(), "light".to_string());
            my_env.save_to_env();
        })
        .selected_if(|data, _| data.theme == CrabTheme::Light);
    let sepia = MenuItem::new("Sepia")
        .on_activate(|_, data: &mut CrabReaderState, _| {
            data.theme = CrabTheme::Sepia;
            // update env
            let mut my_env = MYENV.lock().unwrap();
            my_env.set_property(
                "theme".to_string(), "sepia".to_string());
            my_env.save_to_env();
        })
        .selected_if(|data, _| data.theme == CrabTheme::Sepia);
    let dark = MenuItem::new("Scuro")
        .on_activate(|_, data: &mut CrabReaderState, _| {
            data.theme = CrabTheme::Dark;
            // update env
            let mut my_env = MYENV.lock().unwrap();
            my_env.set_property(
                "theme".to_string(), "dark".to_string());
            my_env.save_to_env();
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
            // update env
            let mut my_env = MYENV.lock().unwrap();
            my_env.set_property(
                "shadows".to_string(), data.paint_shadows.to_string());
            my_env.save_to_env();
        });
    Menu::new("Ombre").entry(shadows_on)
}

fn text_sz() -> Menu<CrabReaderState> {

    fn selected_if_small(data: &CrabReaderState, _: &Env) -> bool {
        data.font.size == fonts::small.size
    }

    fn selected_if_medium(data: &CrabReaderState, _: &Env) -> bool {
        data.font.size == fonts::medium.size
    }

    fn selected_if_large(data: &CrabReaderState, _: &Env) -> bool {
        data.font.size == fonts::large.size
    }

    let small = MenuItem::new("Piccolo").selected_if(selected_if_small)
        .command(Command::new(SET_FONT_SMALL, (), Target::Auto));
    let medium = MenuItem::new("Medio").selected_if(selected_if_medium)
        .command(Command::new(SET_FONT_MEDIUM, (), Target::Auto));
    let large = MenuItem::new("Grande").selected_if(selected_if_large)
        .command(Command::new(SET_FONT_LARGE, (), Target::Auto));
        
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
        .on_activate(|_, data: &mut CrabReaderState, _| {
            let mut my_env = MYENV.lock().unwrap();
            data.font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(my_env.font.size);
            my_env.set_property(
                "font_family".to_string(), 
                "\"SYSTEM_UI\"".to_string()
            );
            my_env.save_to_env();
        });
    let font2 = MenuItem::new("Monospace").selected_if(selected_if_mono)
        .on_activate(|_, data: &mut CrabReaderState, _| {
            let mut my_env = MYENV.lock().unwrap();
            data.font = FontDescriptor::new(FontFamily::MONOSPACE).with_size(my_env.font.size);
            my_env.set_property(
                "font_family".to_string(), 
                "\"MONOSPACE\"".to_string()
            );
            my_env.save_to_env();
        });
    let font3 = MenuItem::new("Serif").selected_if(selected_if_serif)
    .on_activate(|_, data: &mut CrabReaderState, _| {
        let mut my_env = MYENV.lock().unwrap();
        data.font = FontDescriptor::new(FontFamily::SERIF).with_size(my_env.font.size);
        my_env.set_property(
            "font_family".to_string(), 
            "\"SERIF\"".to_string()
        );
        my_env.save_to_env();
    });
    let font4 = MenuItem::new("Sans Serif").selected_if(selected_if_sans)
    .on_activate(|_, data: &mut CrabReaderState, _| {
        let mut my_env = MYENV.lock().unwrap();
        data.font = FontDescriptor::new(FontFamily::SANS_SERIF).with_size(my_env.font.size);
        my_env.set_property(
            "font_family".to_string(), 
            "\"SANS_SERIF\"".to_string()
        );
        my_env.save_to_env();
    });
    Menu::new("Caratteri")
        .entry(font1)
        .entry(font2)
        .entry(font3)
        .entry(font4)
}

fn lang() -> Menu<CrabReaderState> {
    let lang1 = MenuItem::new("Italiano").selected_if(|_, _| true);
    let lang2 = MenuItem::new("Inglese");
    let lang3 = MenuItem::new("Francese");
    let lang4 = MenuItem::new("Spagnolo");
    Menu::new("Lingua")
        .entry(lang1)
        .entry(lang2)
        .entry(lang3)
        .entry(lang4)
}

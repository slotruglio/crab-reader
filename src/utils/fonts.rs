#![allow(unused, non_upper_case_globals)]
use druid::{FontDescriptor, FontFamily, Selector, Key, Env};

use crate::CrabReaderState;

pub const SET_FONT_MONOSPACE:   Selector<()> = Selector::new("font.monospace");
pub const SET_FONT_SYSTEM_UI:   Selector<()> = Selector::new("font.system_ui");
pub const SET_FONT_SERIF:       Selector<()> = Selector::new("font.serif");
pub const SET_FONT_SANS_SERIF:  Selector<()> = Selector::new("font.sans_serif");

pub const FONT: Key<FontDescriptor> = Key::new("crab.reader.font.family");

pub fn update_font_family(env: &mut Env, data: &CrabReaderState) {
    let env_font = env.get(FONT);
    let data_font = data.font.family.clone();
    env.set(FONT, FontDescriptor::new(data_font));
}

mod font_sizes {
    pub const xxsmall: f64 = 6.0;
    pub const xsmall: f64 = 10.0;
    pub const small: f64 = 14.0;
    pub const medium: f64 = 18.0;
    pub const large: f64 = 22.0;
    pub const xlarge: f64 = 26.0;
    pub const xxlarge: f64 = 30.0;
}

pub mod bold {
    use druid::{FontDescriptor, FontFamily, FontWeight};

    use super::font_sizes;

    pub const xxsmall: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::xxsmall)
        .with_weight(FontWeight::BOLD);
    pub const xsmall: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::xsmall)
        .with_weight(FontWeight::BOLD);
    pub const small: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::small)
        .with_weight(FontWeight::BOLD);
    pub const medium: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::medium)
        .with_weight(FontWeight::BOLD);
    pub const large: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::large)
        .with_weight(FontWeight::BOLD);
    pub const xlarge: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::xlarge)
        .with_weight(FontWeight::BOLD);
}

pub mod italic {
    use druid::{FontDescriptor, FontFamily, FontStyle};

    use super::font_sizes;

    pub const xxsmall: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::xxsmall)
        .with_style(FontStyle::Italic);
    pub const xsmall: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::xsmall)
        .with_style(FontStyle::Italic);
    pub const small: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::small)
        .with_style(FontStyle::Italic);
    pub const medium: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::medium)
        .with_style(FontStyle::Italic);
    pub const large: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::large)
        .with_style(FontStyle::Italic);
    pub const xlarge: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(font_sizes::xlarge)
        .with_style(FontStyle::Italic);
}

pub const H1: FontDescriptor = bold::xlarge;
pub const H2: FontDescriptor = bold::large;
pub const H3: FontDescriptor = bold::medium;
pub const H4: FontDescriptor = bold::small;

pub const xxsmall: FontDescriptor =
    FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(font_sizes::xxsmall);
pub const xsmall: FontDescriptor =
    FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(font_sizes::xsmall);
pub const small: FontDescriptor =
    FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(font_sizes::small);
pub const medium: FontDescriptor =
    FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(font_sizes::medium);
pub const large: FontDescriptor =
    FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(font_sizes::large);
pub const xlarge: FontDescriptor =
    FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(font_sizes::xlarge);

pub const HEART_EMOJI: &str = "❤️";

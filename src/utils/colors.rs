use std::ops::Deref;

use druid::{
    theme::{
        BACKGROUND_DARK, BACKGROUND_LIGHT, PLACEHOLDER_COLOR, SCROLLBAR_BORDER_COLOR,
        SCROLLBAR_COLOR, SCROLLBAR_EDGE_WIDTH,
    },
    Color, Data, Env, Key, Selector,
};

use crate::{components::library::cover_library::DO_PAINT_SHADOWS, CrabReaderState};

pub struct ColorSetInner {
    primary: Color,
    primary_variant: Color,
    primary_text: Color,
    primary_accent: Color,
    secondary: Color,
    secondary_accent: Color,
    secondary_variant: Color,
    secondary_text: Color,
    background: Color,
    background_variant: Color,
    background_text: Color,
}

const LIGHT_COLOR_SET: ColorSetInner = ColorSetInner {
    primary: Color::rgb8(0xff, 0x88, 0x22),
    primary_variant: Color::rgb8(0xff, 0x66, 0x22),
    primary_accent: Color::rgb8(0xff, 0xaa, 0x55),
    primary_text: Color::rgb8(0xff, 0xff, 0xff),
    secondary: Color::rgb8(0x33, 0x99, 0xff),
    secondary_accent: Color::rgb8(0x66, 0xbb, 0xff),
    secondary_variant: Color::rgb8(0x33, 0x55, 0xff),
    secondary_text: Color::rgb8(0xff, 0xff, 0xff),
    background_variant: Color::rgb8(0xf0, 0xf0, 0xf0),
    background: Color::rgb8(0xd0, 0xd0, 0xd0),
    background_text: Color::rgb8(0x00, 0x00, 0x00),
};

const SEPIA_COLOR_SET: ColorSetInner = ColorSetInner {
    primary: Color::rgb8(0xd0, 0xd0, 0xd0),
    primary_variant: Color::rgb8(0xa0, 0xa0, 0xa0),
    primary_text: Color::rgb8(0x00, 0x00, 0x00),
    primary_accent: Color::rgb8(0xf6, 0xf6, 0xf6),
    secondary: Color::rgb8(0xd6, 0xd6, 0x20),
    secondary_accent: Color::rgb8(0xf8, 0xf8, 0x55),
    secondary_variant: Color::rgb8(0xfe, 0xdd, 0x22),
    secondary_text: Color::rgb8(0x00, 0x00, 0x00),
    background: Color::rgb8(0xb1, 0x6d, 0x1f),
    background_variant: Color::rgb8(0x6b, 0x44, 0x12),
    background_text: Color::rgb8(0xd7, 0xd9, 0x8b),
};

const DARK_COLOR_SET: ColorSetInner = ColorSetInner {
    primary_text: Color::rgb8(0x00, 0x00, 0x00),
    background_variant: Color::rgb8(0x30, 0x30, 0x30),
    background: Color::rgb8(0x50, 0x50, 0x50),
    background_text: Color::rgb8(0xff, 0xff, 0xff),
    ..LIGHT_COLOR_SET
};

#[derive(Clone, PartialEq, Data)]
pub enum CrabTheme {
    Light,
    Sepia,
    Dark,
}

impl Deref for CrabTheme {
    type Target = ColorSetInner;

    fn deref(&self) -> &Self::Target {
        match self {
            CrabTheme::Light => &LIGHT_COLOR_SET,
            CrabTheme::Sepia => &SEPIA_COLOR_SET,
            CrabTheme::Dark => &DARK_COLOR_SET,
        }
    }
}

pub const PRIMARY: Key<Color> = Key::new("crabreader.primary");
pub const PRIMARY_VARIANT: Key<Color> = Key::new("crabreader.primary_variant");
pub const PRIMARY_ACCENT: Key<Color> = Key::new("crabreader.primary_accent");
pub const SECONDARY: Key<Color> = Key::new("crabreader.secondary");
pub const SECONDARY_VARIANT: Key<Color> = Key::new("crabreader.secondary_variant");
pub const SECONDARY_ACCENT: Key<Color> = Key::new("crabreader.secondary_accent");
pub const BACKGROUND: Key<Color> = Key::new("crabreader.background");
pub const BACKGROUND_VARIANT: Key<Color> = Key::new("crabreader.background_variant");
pub const ON_PRIMARY: Key<Color> = Key::new("crabreader.on_primary");
pub const ON_SECONDARY: Key<Color> = Key::new("crabreader.on_secondary");
pub const ON_BACKGROUND: Key<Color> = Key::new("crabreader.on_background");
pub const SWITCH_THEME: Selector<CrabTheme> = Selector::new("crabreader.switch_theme");

pub fn update_theme(env: &mut Env, data: &CrabReaderState) {
    let theme = &data.theme;
    env.set(PRIMARY, theme.primary.clone());
    env.set(PRIMARY_VARIANT, theme.primary_variant.clone());
    env.set(PRIMARY_ACCENT, theme.primary_accent.clone());
    env.set(SECONDARY, theme.secondary.clone());
    env.set(SECONDARY_VARIANT, theme.secondary_variant.clone());
    env.set(SECONDARY_ACCENT, theme.secondary_accent.clone());
    env.set(BACKGROUND, theme.background.clone());
    env.set(BACKGROUND_VARIANT, theme.background_variant.clone());
    env.set(ON_PRIMARY, theme.primary_text.clone());
    env.set(ON_SECONDARY, theme.secondary_text.clone());
    env.set(ON_BACKGROUND, theme.background_text.clone());
    env.set(BACKGROUND_DARK, theme.background.clone());
    env.set(BACKGROUND_LIGHT, theme.background.clone());
    env.set(
        PLACEHOLDER_COLOR,
        theme.background_text.clone().with_alpha(0.6),
    );
    env.set(SCROLLBAR_COLOR, theme.primary.clone().with_alpha(0.6));
    env.set(
        SCROLLBAR_BORDER_COLOR,
        theme.primary_variant.clone().with_alpha(0.6),
    );
    env.set(SCROLLBAR_EDGE_WIDTH, 2.0);
    env.set(DO_PAINT_SHADOWS, data.paint_shadows)
}

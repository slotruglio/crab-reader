use druid::{Widget, FontDescriptor, FontFamily, FontWeight, FontStyle};

const H1: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
    .with_size(24.0)
    .with_weight(FontWeight::BOLD);

const H2: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
    .with_size(18.0)
    .with_weight(FontWeight::BOLD);

const H3: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
    .with_size(14.0)
    .with_weight(FontWeight::BOLD);

const P: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
    .with_size(12.0)
    .with_weight(FontWeight::NORMAL);

const BOLD: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
    .with_size(12.0)
    .with_weight(FontWeight::BOLD);

const ITALIC: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
    .with_size(12.0)
    .with_weight(FontWeight::NORMAL)
    .with_style(FontStyle::Italic);


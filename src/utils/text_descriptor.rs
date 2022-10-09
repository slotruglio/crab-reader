use druid::{Widget, FontDescriptor, FontFamily, FontWeight, FontStyle, ArcStr, text::{RichText, Attribute}};

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

pub fn get_rich_text(text: String, tags: Vec<(usize, usize, String)>) -> RichText {
    let buffer = ArcStr::from(text);
    let mut rich_text = RichText::new(buffer);
    for tag in tags {
        let start = tag.0;
        let end = tag.1;
        let tag = tag.2;
        
        let attribute = match tag.as_str() {
            "p" => P,
            "strong" => BOLD,
            "em" => ITALIC,
            _ => P,
        };
        let val = Attribute::font_descriptor(attribute);
        rich_text.add_attribute((start..end), val);

    }
    rich_text
}
use druid::{
    text::{Attribute, RichText},
    ArcStr, FontDescriptor, FontFamily, FontStyle, FontWeight, Widget,
};
use epub::doc::EpubDoc;

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

pub fn get_rich_text(text: impl Into<String>, tags: Vec<(usize, usize, String)>) -> RichText {
    let buffer = ArcStr::from(text.into());
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

pub fn get_chapter_rich_text(path: &str, chapter_number: usize, lines_per_chapter: usize) -> Vec<RichText> {
    let mut book = EpubDoc::new(path).unwrap();
    book.set_current_page(chapter_number).unwrap();
    let content = book.get_current_str().unwrap();
    let vec_tagged = html2text::from_read_rich(content.as_bytes(), 50);
    
    
    let mut pages = Vec::new();
    let mut text = String::new();
    let mut tags = Vec::<(usize, usize, String)>::new();
    let mut counter_page = 0 as usize;
    for (i, line) in vec_tagged.into_iter().enumerate() {
        //println!("line: {:?}", line);
        
        for item in line.into_tagged_strings() {
            let starting_index = text.len();
            if item.s.starts_with("*") & item.s.ends_with("*") {
                text.push_str(item.s.replace("*", "").as_str());
            }else{
                text.push_str(item.s.as_str());
            }
            let ending_index = text.len()-1;

            if let Some(tag) = item.tag.get(0) {
                let tag = match tag {
                    html2text::render::text_renderer::RichAnnotation::Default => "default",
                    //html2text::render::text_renderer::RichAnnotation::Link(link) => "a",
                    html2text::render::text_renderer::RichAnnotation::Link(link) => "em",
                    //html2text::render::text_renderer::RichAnnotation::Image(link) => "img",
                    html2text::render::text_renderer::RichAnnotation::Emphasis => "em",
                    html2text::render::text_renderer::RichAnnotation::Strong => "strong",
                    //html2text::render::text_renderer::RichAnnotation::Strikeout => "strike",
                    //html2text::render::text_renderer::RichAnnotation::Code => "code",
                    //html2text::render::text_renderer::RichAnnotation::Preformat(boolean) => "preformat",
                    _ => "none"
                };
                tags.push((starting_index, ending_index, tag.to_string()));
            }
            

        }
        text.push_str("\n");

        if i % lines_per_chapter == lines_per_chapter-1 {
            println!("i: {}, lines: {}", i, lines_per_chapter);
            println!("page: {}", counter_page);
            println!("text: {}", text);
            pages.push(get_rich_text(text.clone(), tags.clone()));
            text.clear();
            tags.clear();
        }
    }
    println!("pages: {}", pages.len());
    pages

}
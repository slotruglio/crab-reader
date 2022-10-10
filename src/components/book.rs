use druid::image::io::Reader as ImageReader;
use druid::piet::{CairoText, Text};
use druid::{FontDescriptor, FontFamily, FontWeight, TextLayout};

use druid::{
    piet::{ImageFormat, InterpolationMode},
    BoxConstraints, Color,
    Cursor::{self, OpenHand},
    Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Rect, RenderContext,
    Size, UpdateCtx, Widget,
};
use std::ops::Deref;
use std::rc::Rc;

pub const BOOK_WIDGET_SIZE: Size = Size::new(150.0, 250.0);

#[derive(Clone, Data)]
pub struct Book {
    title: Rc<String>,
    npages: u16,
    cover_path: Rc<String>,
    #[data(ignore)]
    cover_rgb8: Box<[u8]>,
    selected: bool,
}

impl Book {
    pub fn new() -> Self {
        Self {
            title: Rc::new("".to_string()),
            npages: 0,
            cover_path: Rc::new("".to_string()),
            cover_rgb8: Box::from([]),
            selected: false,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Rc::from(title.into());
        self
    }

    pub fn get_title(&self) -> String {
        (*self.title).clone()
    }

    pub fn with_npages(mut self, npages: u16) -> Self {
        self.npages = npages;
        self
    }

    pub fn get_npages(&self) -> u16 {
        self.npages
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn with_cover_path(mut self, path: impl Into<String>) -> Self {
        self.cover_path = Rc::from(path.into());
        // Move this elesewhere later
        // ... and write better
        let path = std::env::current_dir()
            .unwrap()
            .join("src")
            .join("covers")
            .join(self.cover_path.deref());

        let image_reader = ImageReader::open(&path);
        if let Ok(image_reader) = image_reader {
            if let Ok(image) = image_reader.decode() {
                let h = BOOK_WIDGET_SIZE.height as u32;
                let w = BOOK_WIDGET_SIZE.width as u32;
                let resized = image.thumbnail_exact(w, h).to_rgb8().into_raw();
                self.cover_rgb8 = Box::from(resized.clone());
            }
        }
        self
    }

    // Widget utilities

    fn paint_shadow(&self, ctx: &mut PaintCtx) {
        let blur_radius = 20.0;
        let size = ctx.size();
        let shadow_offset = 20.0; // How much the shadow is offset from the book

        // v Can be optimized? Does it matter? v
        let shadow_rect = Rect::new(
            shadow_offset,
            shadow_offset,
            size.width + shadow_offset,
            size.height + shadow_offset,
        );
        let shadow_color = Color::rgba(0., 0., 0., 0.7); // Black shadow, last value is opacity
        ctx.paint_with_z_index(1, move |ctx| {
            ctx.blurred_rect(shadow_rect, blur_radius, &shadow_color);
        });
    }

    fn paint_book_title(&self, ctx: &mut PaintCtx, env: &Env) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::NORMAL);

        let mut layout = TextLayout::new();
        layout.set_text(self.title.deref().clone());
        layout.set_text_color(Color::WHITE);
        layout.set_font(font);
        layout.set_wrap_width(ctx.size().width - 2.5);
        layout.rebuild_if_needed(ctx.text(), env);

        let pos = ctx.size().to_rect().center() - layout.size().to_vec2() / 2.0;

        ctx.paint_with_z_index(3, move |ctx| {
            if let Some(layout) = layout.layout() {
                ctx.draw_text(layout, pos);
            }
        });
    }

    fn paint_default_cover(&self, ctx: &mut PaintCtx) {
        let round_factr = 20.0;
        let color = Color::rgb8(50, 50, 50);
        let rect = ctx.size().to_rounded_rect(round_factr);

        ctx.paint_with_z_index(2, move |ctx| {
            ctx.fill(rect, &color);
        });
    }

    fn paint_cover(&self, ctx: &mut PaintCtx, env: &Env) {
        if self.cover_rgb8.len() == 0 {
            self.paint_default_cover(ctx);
            self.paint_book_title(ctx, env);
            return;
        }

        let round_factr = 20.0;
        let image_buffer = self.cover_rgb8.clone();
        let paint_rect = ctx.size().to_rect();
        let paint_rounded = paint_rect.clone().to_rounded_rect(round_factr);
        let w = BOOK_WIDGET_SIZE.width as usize;
        let h = BOOK_WIDGET_SIZE.height as usize;
        let image = ctx.make_image(w, h, &image_buffer, ImageFormat::Rgb);
        if let Ok(image) = image {
            ctx.paint_with_z_index(2, move |ctx| {
                ctx.with_save(|ctx| {
                    ctx.clip(paint_rounded);
                    ctx.draw_image(&image, paint_rect, InterpolationMode::Bilinear);
                });
            });
        }
    }
}

// ???
impl Widget<Book> for Book {
    fn event(&mut self, ctx: &mut EventCtx, _event: &Event, _data: &mut Book, _env: &Env) {
        if ctx.is_hot() {
            ctx.set_cursor(&OpenHand);
        } else {
            ctx.set_cursor(&Cursor::Arrow);
        }
        ctx.request_paint();
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &Book, _env: &Env) {
        _ctx.request_paint();
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Book, _data: &Book, _env: &Env) {
        _ctx.request_paint();
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &Book,
        _env: &Env,
    ) -> Size {
        BOOK_WIDGET_SIZE
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &Book, env: &Env) {
        self.paint_shadow(ctx);
        self.paint_cover(ctx, env);
    }
}

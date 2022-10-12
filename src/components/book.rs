use druid::image::io::Reader as ImageReader;
use druid::piet::{CairoText, Text};
use druid::{FontDescriptor, FontFamily, FontWeight, Point, TextLayout};

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
    bg_color: Color,
}

impl Book {
    pub fn new() -> Self {
        Self {
            title: Rc::new("".to_string()),
            npages: 0,
            cover_path: Rc::new("".to_string()),
            cover_rgb8: Box::from([]),
            selected: false,
            bg_color: Color::rgb8(50, 50, 50),
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
        let shadow_offset = 15.0; // How much the shadow is offset from the book

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
        let color = self.bg_color.clone();
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

pub struct BookCoverItem(Book);

impl From<Book> for BookCoverItem {
    fn from(book: Book) -> Self {
        Self(book)
    }
}

impl Widget<Book> for BookCoverItem {
    fn event(&mut self, ctx: &mut EventCtx, _event: &Event, _data: &mut Book, _env: &Env) {
        if ctx.is_hot() {
            ctx.set_cursor(&OpenHand);
        } else {
            ctx.set_cursor(&Cursor::Arrow);
        }
        ctx.request_paint();
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &Book, _env: &Env) {
        ctx.request_layout();
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
        self.0.paint_shadow(ctx);
        self.0.paint_cover(ctx, env);
    }
}

pub struct ListBookItem(Book);

impl From<Book> for ListBookItem {
    fn from(book: Book) -> Self {
        ListBookItem(book)
    }
}

impl ListBookItem {
    fn get_bg_color(&self) -> Color {
        self.0.bg_color.clone()
    }

    fn paint_bg_rect(&self, ctx: &mut PaintCtx, _: &Env) {
        let color = self.get_bg_color();
        let rect = ctx.size().to_rect().to_rounded_rect(10.0);
        ctx.paint_with_z_index(0, move |ctx| {
            ctx.fill(rect, &color);
        });
    }

    fn paint_num_pages(&self, ctx: &mut PaintCtx, env: &Env) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::NORMAL);

        let mut layout = TextLayout::new();
        layout.set_text(format!("0/{} pagine lette", self.0.npages));
        layout.set_text_color(Color::WHITE);
        layout.set_font(font);
        layout.set_wrap_width(ctx.size().width / 4.0);
        layout.rebuild_if_needed(ctx.text(), env);

        let pos = (
            ctx.size().to_rect().width() - layout.size().width - 10.0,
            ctx.size().height / 2.0 - layout.size().height / 2.0,
        );

        ctx.paint_with_z_index(3, move |ctx| {
            if let Some(layout) = layout.layout() {
                ctx.draw_text(layout, pos);
            }
        });
    }

    fn paint_title(&self, ctx: &mut PaintCtx, env: &Env) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::NORMAL);

        let mut layout = TextLayout::new();
        layout.set_text(self.0.title.deref().clone());
        layout.set_text_color(Color::WHITE);
        layout.set_font(font.clone());
        layout.set_wrap_width(ctx.size().width * 3.0 / 4.0);
        layout.rebuild_if_needed(ctx.text(), env);

        let pos: Point = (10.0, ctx.size().height / 2.0 - layout.size().height / 2.0).into();

        ctx.paint_with_z_index(3, move |ctx| {
            if let Some(layout) = layout.layout() {
                ctx.draw_text(layout, pos);
            }
        });
    }

    fn set_hovered_bg_color(&mut self, is_hovered: bool) {
        match is_hovered {
            true => self.0.bg_color = Color::rgb8(100, 100, 100),
            false => self.0.bg_color = Color::rgb8(50, 50, 50),
        }
    }
}

impl Widget<Book> for ListBookItem {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut Book, _: &Env) {
        ()
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _: &Book, _: &Env) {
        match event {
            LifeCycle::HotChanged(hot) => {
                self.set_hovered_bg_color(*hot);
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn update(&mut self, _: &mut UpdateCtx, _: &Book, _: &Book, _: &Env) {
        ()
    }

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &Book, _: &Env) -> Size {
        // Father sets the size for this child
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _: &Book, env: &Env) {
        self.paint_bg_rect(ctx, env);
        self.paint_title(ctx, env);
        self.paint_num_pages(ctx, env);
    }
}

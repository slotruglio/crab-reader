use druid::image::io::Reader as ImageReader;
use druid::piet::{CairoText, Text};
use druid::{Command, FontDescriptor, FontFamily, FontWeight, Target, TextLayout};

use druid::{
    piet::{ImageFormat, InterpolationMode},
    BoxConstraints, Color,
    Cursor::{self, OpenHand},
    Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Rect, RenderContext,
    Size, UpdateCtx, Widget,
};
use std::rc::Rc;

use super::library::SELECTED_BOOK_SELECTOR;

pub const BOOK_WIDGET_SIZE: Size = Size::new(150.0, 250.0);
pub const SELECTED_BG_COLOR: Color = Color::rgb8(20, 20, 20);
pub const HOT_BG_COLOR: Color = Color::rgb8(70, 70, 70);
pub const NORMAL_BG_COLOR: Color = Color::rgb8(40, 40, 40);

#[derive(Clone, Data, PartialEq)]
pub struct Book {
    title: Rc<String>,
    npages: u16,
    idx: u16,
    cover_path: Rc<String>,
    selected: bool,
}

impl Book {
    pub fn new() -> Self {
        Self {
            title: Rc::new("".to_string()),
            npages: 0,
            idx: 0, // Represents the book position in the array... is there a better way?
            cover_path: Rc::new("".to_string()),
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

    pub fn unselect(&mut self) {
        self.selected = false;
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn with_cover_path(mut self, filename: impl Into<String>) -> Self {
        if let Ok(cwd) = std::env::current_dir() {
            let path = cwd.join("src").join("covers").join(filename.into());
            if path.exists() {
                self.cover_path = Rc::from(path.to_str().unwrap().to_string());
            }
        }
        self
    }

    pub fn get_cover_path(&self) -> String {
        (*self.cover_path).clone()
    }

    pub fn with_idx(mut self, idx: u16) -> Book {
        self.set_idx(idx);
        self
    }

    pub fn set_idx(&mut self, idx: u16) {
        self.idx = idx;
    }

    pub fn get_idx(&self) -> u16 {
        self.idx
    }

    // Widget utilities
}

pub struct CoverBook {
    cover_img: Box<[u8]>,
    cover_img_path: Rc<String>,
    is_hot: bool,
}

impl CoverBook {
    pub fn new() -> Self {
        Self {
            cover_img: Box::new([]),
            cover_img_path: Rc::new("".to_string()),
            is_hot: false,
        }
    }

    pub fn with_cover_image_path(mut self, path: impl Into<String>) -> Self {
        let path: String = path.into();
        self.set_cover_image_path(path);
        self
    }

    pub fn set_cover_image_path(&mut self, path: impl Into<String>) {
        let path: String = path.into();
        self.cover_img_path = Rc::from(path);
        self.load_cover_image();
    }

    fn load_cover_image(&mut self) {
        let path = (*self.cover_img_path).clone();
        if let Ok(image) = ImageReader::open(path) {
            if let Ok(image) = image.decode() {
                let h = BOOK_WIDGET_SIZE.height as u32;
                let w = BOOK_WIDGET_SIZE.width as u32;
                let resized = image.thumbnail_exact(w, h).to_rgb8().into_raw();
                self.cover_img = Box::from(resized.clone());
            }
        }
    }

    fn paint_shadow(&self, ctx: &mut PaintCtx) {
        let blur_radius = 20.0;
        let size = ctx.size();
        let shadow_offset = 15.0; // How much the shadow is offset from the book
        let shadow_color = Color::rgba(0., 0., 0., 0.7); // Black shadow, last value is opacity

        let shadow_rect = Rect::new(
            shadow_offset,
            shadow_offset,
            size.width + shadow_offset,
            size.height + shadow_offset,
        );

        ctx.paint_with_z_index(1, move |ctx| {
            ctx.blurred_rect(shadow_rect, blur_radius, &shadow_color);
        });
    }

    fn paint_cover(&self, ctx: &mut PaintCtx, env: &Env, data: &Book) {
        if self.cover_img.len() == 0 {
            self.paint_default_cover(ctx, data);
            self.paint_book_title(ctx, env, data);
            return;
        }

        let round_factr = 20.0;
        let image_buffer = &self.cover_img;
        let paint_rect = ctx.size().to_rect();
        let paint_rounded = paint_rect.clone().to_rounded_rect(round_factr);
        let w = BOOK_WIDGET_SIZE.width as usize;
        let h = BOOK_WIDGET_SIZE.height as usize;

        if let Ok(image) = ctx.make_image(w, h, &image_buffer, ImageFormat::Rgb) {
            ctx.paint_with_z_index(2, move |ctx| {
                ctx.with_save(|ctx| {
                    ctx.clip(paint_rounded);
                    ctx.draw_image(&image, paint_rect, InterpolationMode::Bilinear);
                });
            });
        }
    }

    fn paint_default_cover(&self, ctx: &mut PaintCtx, data: &Book) {
        let round_factr = 20.0;
        let color = self.get_bg_color(data);
        let rect = ctx.size().to_rounded_rect(round_factr);

        ctx.paint_with_z_index(2, move |ctx| {
            ctx.fill(rect, &color);
        });
    }

    fn paint_book_title(&self, ctx: &mut PaintCtx, env: &Env, data: &Book) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::NORMAL);

        let mut layout = TextLayout::new();
        layout.set_text(data.get_title());
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

    fn set_hot(&mut self, is_hot: bool) {
        self.is_hot = is_hot;
    }

    fn get_bg_color(&self, data: &Book) -> Color {
        if data.is_selected() {
            SELECTED_BG_COLOR
        } else if self.is_hot {
            HOT_BG_COLOR
        } else {
            NORMAL_BG_COLOR
        }
    }
}

impl Widget<Book> for CoverBook {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Book, _: &Env) {
        if ctx.is_hot() {
            ctx.set_cursor(&OpenHand);
        } else {
            ctx.set_cursor(&Cursor::Arrow);
        }

        match event {
            Event::MouseDown(_) => {
                data.select();
                ctx.set_handled();
                let cmd = Command::new(
                    SELECTED_BOOK_SELECTOR.into(),
                    Some(data.get_idx()),
                    Target::Auto,
                );
                ctx.submit_notification(cmd);
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _: &Book, _: &Env) {
        match event {
            LifeCycle::HotChanged(hot) => {
                self.set_hot(*hot);
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Book, data: &Book, _: &Env) {
        if data != old_data {
            ctx.request_layout();
        }
    }

    fn layout(&mut self, _: &mut LayoutCtx, _: &BoxConstraints, _: &Book, _: &Env) -> Size {
        BOOK_WIDGET_SIZE
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Book, env: &Env) {
        self.paint_shadow(ctx);
        self.paint_cover(ctx, env, data);
    }
}

#[derive(Clone)]
pub struct BookListing {
    is_hot: bool,
}

impl BookListing {
    pub fn new() -> Self {
        Self { is_hot: false }
    }

    fn paint_bg_rect(&self, ctx: &mut PaintCtx, _: &Env, data: &Book) {
        let color = self.get_bg_color(data);
        let rect = ctx.size().to_rect().to_rounded_rect(10.0);
        ctx.paint_with_z_index(0, move |ctx| {
            ctx.fill(rect, &color);
        });
    }

    fn paint_num_pages(&self, ctx: &mut PaintCtx, env: &Env, data: &Book) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::NORMAL);

        let mut layout = TextLayout::new();
        layout.set_text(format!("0/{} pagine lette", data.get_npages()));
        layout.set_text_color(Color::WHITE);
        layout.set_font(font);
        layout.set_wrap_width(ctx.size().width / 4.0);
        layout.rebuild_if_needed(ctx.text(), env);

        let pos = (
            ctx.size().width - layout.size().width - 10.0,
            ctx.size().height / 2.0 - layout.size().height / 2.5, // ???
        );

        ctx.paint_with_z_index(3, move |ctx| {
            if let Some(layout) = layout.layout() {
                ctx.draw_text(layout, pos);
            }
        });
    }

    fn paint_title(&self, ctx: &mut PaintCtx, env: &Env, data: &Book) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::NORMAL);

        let mut layout = TextLayout::new();
        layout.set_text(data.get_title());
        layout.set_text_color(Color::WHITE);
        layout.set_font(font.clone());
        layout.set_wrap_width(ctx.size().width * 3.0 / 4.0);
        layout.rebuild_if_needed(ctx.text(), env);

        let pos = (10.0, ctx.size().height / 2.0 - layout.size().height / 4.0);

        ctx.paint_with_z_index(3, move |ctx| {
            if let Some(layout) = layout.layout() {
                ctx.draw_text(layout, pos);
            }
        });
    }

    fn set_hot(&mut self, is_hot: bool) {
        self.is_hot = is_hot;
    }

    fn get_bg_color(&self, data: &Book) -> Color {
        if data.is_selected() {
            SELECTED_BG_COLOR
        } else if self.is_hot {
            HOT_BG_COLOR
        } else {
            NORMAL_BG_COLOR
        }
    }
}

impl Widget<Book> for BookListing {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Book, _: &Env) {
        match event {
            Event::MouseDown(_) => {
                data.select();
                ctx.set_handled();
                let cmd = Command::new(
                    SELECTED_BOOK_SELECTOR.into(),
                    Some(data.get_idx()),
                    Target::Auto,
                );
                ctx.submit_notification(cmd);
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _: &Book, _: &Env) {
        match event {
            LifeCycle::HotChanged(hot) => {
                self.set_hot(*hot);
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Book, data: &Book, _: &Env) {
        if old_data != data {
            ctx.request_paint();
        }
    }

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &Book, _: &Env) -> Size {
        // Father sets the size for this child
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Book, env: &Env) {
        self.paint_bg_rect(ctx, env, data);
        self.paint_title(ctx, env, data);
        self.paint_num_pages(ctx, env, data);
    }
}

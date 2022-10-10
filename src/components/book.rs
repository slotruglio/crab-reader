use druid::image::io::Reader as ImageReader;

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
    cover_rgb8: Rc<Vec<u8>>, // No way to use `Vector`? Should use Rc? Edit: used Rc, maybe slightly faster
    selected: bool,
}

impl Book {
    pub fn new() -> Self {
        Self {
            title: Rc::new("".to_string()),
            npages: 0,
            cover_path: Rc::new("".to_string()),
            cover_rgb8: Rc::from(Vec::new()),
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
        match image_reader {
            Ok(reader) => match reader.decode() {
                Ok(image) => {
                    let h = 250u32;
                    let w = 150u32;
                    let resized = image.thumbnail_exact(w, h);

                    // IMPORTANT TO DO
                    // ADD OPTION INSTEAD
                    // OTHERWISE IT PANICS IF NOT FOUND
                    self.cover_rgb8 = Rc::from(resized.to_rgb8().to_vec());
                }
                Err(e) => {
                    println!("Error decoding: {}", e);
                }
            },
            Err(err) => {
                println!("path: {:?}", path);
                println!("Error image {}: {}", err, self.cover_path);
            }
        }
        self
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

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &Book, _env: &Env) {
        // Rewrite this ugly ass function
        // Paint shadow
        let size = ctx.size();
        ctx.paint_with_z_index(1, move |ctx| {
            let offset = 20.0;
            let shadow = Rect::new(offset, offset, size.width + offset, size.height + offset);
            let shadow_color = &Color::rgba8(20, 20, 20, 160);
            ctx.render_ctx.blurred_rect(shadow, 10.0, shadow_color);
        });

        // Paint cover image
        let buf = self.cover_rgb8.clone();
        let size = ctx.size();
        ctx.paint_with_z_index(2, move |ctx| {
            let rect = size.to_rect();
            let rrect = size.to_rounded_rect(20.0);
            ctx.clip(rrect);
            let image = ctx.make_image(150, 250, &buf, ImageFormat::Rgb);
            if let Ok(image) = image {
                ctx.draw_image(&image, rect, InterpolationMode::Bilinear);
            } else {
                println!("Error creating image.");
            }
            let _ = ctx.restore();
        });

        // Text -- Book Title
        // Disable for now, maybe for ever
        // let mut tl: TextLayout<String> = TextLayout::new();
        // tl.set_text((*self.title).clone());
        // tl.set_text_color(Color::WHITE);
        // tl.set_text_alignment(druid::piet::TextAlignment::Justified);
        // tl.set_text_size(24.0);
        // tl.set_wrap_width(ctx.size().width - 10.0);
        // tl.rebuild_if_needed(ctx.text(), _env);

        // let x = 10.0;
        // let y = (ctx.size().height / 2.0) - (tl.size().height / 2.0);
        // let pos = Point::new(x, y);

        // if let Some(layout) = tl.layout() {
        // ctx.render_ctx.draw_text(layout, pos);
        // }
    }
}

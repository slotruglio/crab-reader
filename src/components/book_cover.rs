use druid::{
    piet::{ImageFormat, InterpolationMode, Text},
    BoxConstraints, Color, Command,
    Cursor::OpenHand,
    Data, Env, Event, EventCtx, FontDescriptor, FontFamily, FontWeight, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Rect, RenderContext, Size, Target, TextLayout, UpdateCtx, Widget,
};

use super::{book::GUIBook, library::SELECTED_BOOK_SELECTOR};

const SELECTED_BG_COLOR: Color = Color::rgb8(20, 20, 20);
const HOT_BG_COLOR: Color = Color::rgb8(70, 70, 70);
const NORMAL_BG_COLOR: Color = Color::rgb8(40, 40, 40);
pub const BOOK_WIDGET_SIZE: Size = Size::new(150.0, 250.0);

/// This structure contains the data relative to a Book when it is rendered as a cover, i.e. a rounded
/// rect with smoothed edges and the book cover as a picture
pub struct BookCover {
    cover_img: Option<Box<[u8]>>,
    is_hot: bool,
}

impl BookCover {
    pub fn new() -> Self {
        Self {
            cover_img: None,
            is_hot: false,
        }
    }

    pub fn with_cover_image(mut self, cover_img: Option<Box<[u8]>>) -> Self {
        self.cover_img = cover_img;
        self
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

    fn paint_cover(&self, ctx: &mut PaintCtx, env: &Env, data: &impl GUIBook) {
        if self.cover_img.is_none() {
            self.paint_default_cover(ctx, data);
            self.paint_book_title(ctx, env, data);
            return;
        }

        let round_factr = 20.0;
        let image_buffer = self.cover_img.clone().unwrap();
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

    fn paint_default_cover(&self, ctx: &mut PaintCtx, data: &impl GUIBook) {
        let round_factr = 20.0;
        let color = self.get_bg_color(data);
        let rect = ctx.size().to_rounded_rect(round_factr);

        ctx.paint_with_z_index(2, move |ctx| {
            ctx.fill(rect, &color);
        });
    }

    fn paint_book_title(&self, ctx: &mut PaintCtx, env: &Env, data: &impl GUIBook) {
        let font_family = ctx
            .text()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::NORMAL);

        let mut layout = TextLayout::new();
        layout.set_text(data.get_title().to_string());
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

    fn get_bg_color(&self, data: &impl GUIBook) -> Color {
        if data.is_selected() {
            SELECTED_BG_COLOR
        } else if self.is_hot {
            HOT_BG_COLOR
        } else {
            NORMAL_BG_COLOR
        }
    }
}

impl<BookData: GUIBook + Data> Widget<BookData> for BookCover {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut BookData, _: &Env) {
        if ctx.is_hot() {
            ctx.set_cursor(&OpenHand);
        } else {
            ctx.clear_cursor();
        }

        match event {
            Event::MouseDown(_) => {
                data.select();
                ctx.set_handled();
                let cmd = Command::new(
                    SELECTED_BOOK_SELECTOR.into(),
                    Some(data.get_index()),
                    Target::Auto,
                );
                ctx.submit_notification(cmd);
                ctx.request_layout();
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _: &BookData, _: &Env) {
        match event {
            LifeCycle::HotChanged(hot) => {
                self.set_hot(*hot);
                ctx.request_layout();
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &BookData, data: &BookData, _: &Env) {
        if data.same(old_data) {
            ctx.request_layout();
        }
    }

    fn layout(&mut self, _: &mut LayoutCtx, _: &BoxConstraints, _: &BookData, _: &Env) -> Size {
        BOOK_WIDGET_SIZE
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &BookData, env: &Env) {
        self.paint_shadow(ctx);
        self.paint_cover(ctx, env, data);
    }
}

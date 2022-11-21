use druid::{
    piet::{ImageFormat, InterpolationMode, PietImage},
    widget::Label,
    BoxConstraints, Color, Command,
    Cursor::OpenHand,
    Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Rect, RenderContext,
    Size, Target, TextLayout, UpdateCtx, Widget, WidgetPod,
};

use crate::{utils::fonts, PAINT_BOOK_COVERS_SHADOWS};

use super::{book::GUIBook, colors, library::SELECTED_BOOK_SELECTOR};

pub const BOOK_WIDGET_SIZE: Size = Size::new(150.0, 250.0);

/// This structure contains the data relative to a Book when it is rendered as a cover, i.e. a rounded
/// rect with smoothed edges and the book cover as a picture
pub struct BookCover<B: GUIBook> {
    is_hot: bool,
    star: WidgetPod<B, Label<B>>,
    image: Option<PietImage>,
}

impl<B: GUIBook> BookCover<B> {
    pub fn new() -> Self {
        let star = Label::dynamic(|data: &B, _| {
            if data.is_favorite() {
                "🌟".into()
            } else {
                "".into()
            }
        })
        .with_font(fonts::Font::default().lg().emoji().get());

        Self {
            is_hot: false,
            star: WidgetPod::new(star),
            image: None,
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

        ctx.blurred_rect(shadow_rect, blur_radius, &shadow_color);
    }

    fn paint_cover(&mut self, ctx: &mut PaintCtx, env: &Env, data: &impl GUIBook) {
        let cover_data = data.get_cover_image();
        if cover_data.len() == 0 {
            self.paint_default_cover(ctx, data);
            self.paint_book_title(ctx, env, data);
            return;
        }

        let round_factr = 20.0;
        let paint_rect = ctx.size().to_rect();
        let paint_rounded = paint_rect.clone().to_rounded_rect(round_factr);
        let w = BOOK_WIDGET_SIZE.width as usize;
        let h = BOOK_WIDGET_SIZE.height as usize;

        if let Some(ref image) = self.image {
            ctx.with_save(|ctx| {
                ctx.clip(paint_rounded);
                ctx.draw_image(&image, paint_rect, InterpolationMode::Bilinear);
            });
        } else if let Ok(image) = ctx.make_image(w, h, &cover_data, ImageFormat::Rgb) {
            ctx.with_save(|ctx| {
                ctx.clip(paint_rounded);
                ctx.draw_image(&image, paint_rect, InterpolationMode::Bilinear);
            });
            self.image = Some(image);
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
        let font = fonts::Font::default().sm().get();
        let mut layout = TextLayout::new();
        layout.set_text(data.get_title().to_string());
        layout.set_text_color(colors::TEXT_WHITE);
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
            colors::ACTIVE_GRAY
        } else if self.is_hot {
            colors::BG_GRAY
        } else {
            colors::NORMAL_GRAY
        }
    }
}

impl<B: GUIBook + Data> Widget<B> for BookCover<B> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut B, env: &Env) {
        self.star.event(ctx, event, data, env);

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
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &B, env: &Env) {
        self.star.lifecycle(ctx, event, data, env);
        match event {
            LifeCycle::HotChanged(hot) => {
                self.set_hot(*hot);
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &B, data: &B, env: &Env) {
        if !data.same(old_data) {
            self.star.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &B, env: &Env) -> Size {
        if data.is_filtered_out() {
            return Size::ZERO;
        }
        let sl = self.star.layout(ctx, bc, data, env);
        let origin = (10.0, BOOK_WIDGET_SIZE.height - sl.height - 10.0).into();
        self.star.set_origin(ctx, data, env, origin);
        BOOK_WIDGET_SIZE
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &B, env: &Env) {
        if env.get(PAINT_BOOK_COVERS_SHADOWS) {
            self.paint_shadow(ctx);
        }
        self.paint_cover(ctx, env, data);
        self.star.paint(ctx, data, env);
    }
}

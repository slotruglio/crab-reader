use druid::{
    piet::InterpolationMode,
    widget::{Label, LineBreaking},
    BoxConstraints, Color, Command,
    Cursor::Pointer,
    Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Rect, RenderContext,
    Size, Target, UpdateCtx, Widget, WidgetPod,
};

use crate::{
    components::library::cover_library::DO_PAINT_SHADOWS,
    models::library::SELECTED_BOOK_SELECTOR,
    traits::gui::GUIBook,
    utils::{colors, fonts},
};

pub const BOOK_WIDGET_SIZE: Size = Size::new(150.0, 250.0);

/// This structure contains the data relative to a Book when it is rendered as a cover, i.e. a rounded
/// rect with smoothed edges and the book cover as a picture
pub struct BookCover<B: GUIBook> {
    is_hot: bool,
    star: WidgetPod<B, Label<B>>,
    label: WidgetPod<B, Label<B>>,
}

impl<B: GUIBook> BookCover<B> {
    pub fn new() -> Self {
        let star = Label::dynamic(|data: &B, _| {
            if data.is_favorite() {
                fonts::HEART_EMOJI.into()
            } else {
                "".into()
            }
        })
        .with_font(fonts::medium);

        let label = Label::dynamic(|data: &B, _| data.get_title().to_string())
            .with_line_break_mode(LineBreaking::WordWrap)
            .with_font(fonts::medium)
            .with_text_color(colors::ON_PRIMARY);

        Self {
            is_hot: false,
            star: WidgetPod::new(star),
            label: WidgetPod::new(label),
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

    fn paint_cover(&mut self, ctx: &mut PaintCtx, data: &B, env: &Env) {
        let rect = ctx.size().to_rect();
        let rrect = rect.clone().to_rounded_rect(10.0);

        if data.get_cover_image().is_none() && data.get_cover_buffer().is_empty() {
            self.paint_default_cover(ctx, data, env);
            return;
        }

        if data.get_cover_image().is_none() && !data.get_cover_buffer().is_empty() {
            let Ok(_) = data.set_cover_image(ctx) else { return };
        }

        if data.get_cover_image().is_none() {
            self.paint_default_cover(ctx, data, env);
            return;
        }

        if let Some(image) = data.get_cover_image().as_ref() {
            ctx.with_save(|ctx| {
                ctx.clip(rrect);
                ctx.draw_image(&image, rect, InterpolationMode::Bilinear);
            });
        }
    }

    fn paint_default_cover(&mut self, ctx: &mut PaintCtx, data: &B, env: &Env) {
        let round_factr = 20.0;
        let color = self.get_bg_color(data, env);
        let rect = ctx.size().to_rounded_rect(round_factr);

        ctx.fill(rect, &color);
        self.label.paint(ctx, data, env);
    }

    fn get_bg_color(&self, data: &impl GUIBook, env: &Env) -> Color {
        if data.is_selected() {
            env.get(colors::PRIMARY_VARIANT)
        } else if self.is_hot {
            env.get(colors::PRIMARY_ACCENT)
        } else {
            env.get(colors::PRIMARY)
        }
    }
}

impl<B: GUIBook + Data> Widget<B> for BookCover<B> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut B, env: &Env) {
        self.star.event(ctx, event, data, env);
        self.label.event(ctx, event, data, env);

        if ctx.is_hot() {
            ctx.set_cursor(&Pointer);
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
        self.label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &B, data: &B, env: &Env) {
        if !data.same(old_data) || ctx.env_changed() {
            self.star.update(ctx, data, env);
            self.label.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &B, env: &Env) -> Size {
        if data.is_filtered_out() {
            return Size::ZERO;
        }
        let sl = self.star.layout(ctx, bc, data, env);
        let origin = (10.0, BOOK_WIDGET_SIZE.height - sl.height - 10.0).into();
        self.star.set_origin(ctx, data, env, origin);
        let lbc = BoxConstraints::tight(bc.constrain(BOOK_WIDGET_SIZE))
            .loosen()
            .shrink((10.0, 10.0));
        let ls = self.label.layout(ctx, &lbc, data, env);
        let origin_x = (BOOK_WIDGET_SIZE.width - ls.width) / 2.0;
        let origin_y = (BOOK_WIDGET_SIZE.height - ls.height) / 2.0;
        let origin = (origin_x, origin_y).into();
        self.label.set_origin(ctx, data, env, origin);
        BOOK_WIDGET_SIZE
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &B, env: &Env) {
        if env.get(DO_PAINT_SHADOWS) {
            self.paint_shadow(ctx);
        }
        self.paint_cover(ctx, data, env);
        self.star.paint(ctx, data, env);
    }
}

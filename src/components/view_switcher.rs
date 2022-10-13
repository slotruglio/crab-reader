use druid::{
    piet::{CairoText, Text},
    BoxConstraints, Color,
    Cursor::{self},
    Data, Env, Event, EventCtx, FontDescriptor, FontFamily, FontWeight, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, RenderContext, Size, TextLayout, UpdateCtx, Widget,
};

#[derive(Clone, Data, PartialEq)]
pub enum ViewMode {
    List,
    Cover,
}

pub struct SwitcherButton;

impl SwitcherButton {
    fn get_bg_color(&self, ctx: &PaintCtx) -> Color {
        if ctx.is_hot() {
            Color::rgb8(20, 20, 20)
        } else {
            Color::GRAY
        }
    }

    fn paint_bg_rect(&self, ctx: &mut PaintCtx, _: &ViewMode) {
        let bg_color = self.get_bg_color(ctx);
        let bg = ctx.size().to_rounded_rect(10.0);
        ctx.paint_with_z_index(0, move |ctx| {
            ctx.fill(bg, &bg_color);
        });
    }

    fn paint_text(&self, ctx: &mut PaintCtx, data: &ViewMode, env: &Env) {
        let text = match data {
            ViewMode::List => "Passa a Cover",
            ViewMode::Cover => "Passa a Lista",
        };

        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::NORMAL);

        let mut layout: TextLayout<String> = TextLayout::new();
        layout.set_text(text.into());
        layout.set_text_color(Color::WHITE);
        layout.set_font(font);
        layout.set_wrap_width(ctx.size().width - 2.5);
        layout.rebuild_if_needed(ctx.text(), env);

        let pos = (
            ctx.size().width / 2.0 - layout.size().width / 2.0,
            ctx.size().height / 2.0 - layout.size().height / 4.0,
        );

        ctx.paint_with_z_index(3, move |ctx| {
            if let Some(layout) = layout.layout() {
                ctx.draw_text(layout, pos);
            }
        });
    }
}

impl Widget<ViewMode> for SwitcherButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut ViewMode, _: &Env) {
        match event {
            Event::MouseDown(_) => {
                *data = match *data {
                    ViewMode::List => ViewMode::Cover,
                    ViewMode::Cover => ViewMode::List,
                };
                ctx.request_paint();
            }
            Event::MouseMove(_) => {
                if ctx.is_hot() {
                    ctx.set_cursor(&Cursor::OpenHand);
                } else {
                    ctx.clear_cursor();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _: &ViewMode, _: &Env) {
        match event {
            LifeCycle::HotChanged(_) => {
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &ViewMode, data: &ViewMode, _: &Env) {
        if old_data != data {
            ctx.request_paint();
        }
    }

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &ViewMode, _: &Env) -> Size {
        let w = bc.max().width.min(150.0);
        let h = bc.max().height.min(35.0);
        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &ViewMode, env: &Env) {
        self.paint_bg_rect(ctx, data);
        self.paint_text(ctx, data, env);
    }
}

use druid::{
    piet::{CairoText, Text},
    BoxConstraints, Color, Command, Data, Env, Event, EventCtx, FontDescriptor, FontFamily,
    FontWeight, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, Target,
    TextLayout, UpdateCtx, Widget,
};

use super::{book::GUIBook, library::SELECTED_BOOK_SELECTOR};

const SELECTED_BG_COLOR: Color = Color::rgb8(20, 20, 20);
const HOT_BG_COLOR: Color = Color::rgb8(70, 70, 70);
const NORMAL_BG_COLOR: Color = Color::rgb8(40, 40, 40);

#[derive(Clone)]
pub struct BookListing {
    is_hot: bool,
}

impl BookListing {
    pub fn new() -> Self {
        Self { is_hot: false }
    }

    fn paint_bg_rect(&self, ctx: &mut PaintCtx, _: &Env, data: &impl GUIBook) {
        let color = self.get_bg_color(data);
        let rect = ctx.size().to_rect().to_rounded_rect(10.0);
        ctx.paint_with_z_index(0, move |ctx| {
            ctx.fill(rect, &color);
        });
    }

    fn paint_num_pages(&self, ctx: &mut PaintCtx, env: &Env, data: &impl GUIBook) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::NORMAL);

        let mut layout = TextLayout::new();

        layout.set_text(format!(
            "{}/{} pagine lette",
            data.get_number_of_read_pages(),
            data.get_number_of_pages()
        ));

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

    fn paint_title(&self, ctx: &mut PaintCtx, env: &Env, data: &impl GUIBook) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::NORMAL);

        let mut layout = TextLayout::new();
        layout.set_text(data.get_title().to_string());
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

impl<BookState: GUIBook + Data> Widget<BookState> for BookListing {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut BookState, _: &Env) {
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
            Event::MouseMove(_) => {
                self.set_hot(ctx.is_hot());
                ctx.request_paint();
            }
            Event::Wheel(_) => {
                self.set_hot(false);
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &BookState, _: &Env) {
        ()
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &BookState, data: &BookState, _: &Env) {
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &BookState, _: &Env) -> Size {
        // Father sets the size for this child
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &BookState, env: &Env) {
        self.paint_bg_rect(ctx, env, data);
        self.paint_title(ctx, env, data);
        self.paint_num_pages(ctx, env, data);
    }
}

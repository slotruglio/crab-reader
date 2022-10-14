use druid::{
    piet::{CairoText, Text},
    BoxConstraints, Color, Env, Event, EventCtx, FontDescriptor, FontFamily, FontWeight, LayoutCtx,
    LifeCycle, LifeCycleCtx, PaintCtx, Point, UpdateCtx, Widget,
};
use druid::{RenderContext, TextLayout};

use super::{book::Book, library::Library};

pub struct BookDetails {
    title_offset_y: f64,
    author_offset_y: f64,
    description_offset_y: f64,
}

impl Widget<Library> for BookDetails {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut Library, _: &Env) {
        ()
    }

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &Library, _: &Env) {
        ()
    }

    fn update(&mut self, _: &mut UpdateCtx, _: &Library, _: &Library, _: &Env) {
        ()
    }

    fn layout(
        &mut self,
        _: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Library,
        _: &Env,
    ) -> druid::Size {
        if data.get_selected_book().is_some() {
            let w = bc.max().width;
            let h = self.description_offset_y + 10.0;
            (w, h).into()
        } else {
            bc.min()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Library, env: &Env) {
        if let Some(idx) = data.get_selected_book() {
            if let Some(book) = data.get_book(idx) {
                self.make_title(ctx, book, env);
                self.make_author(ctx, book, env);
                self.make_description(ctx, book, env)
            }
        }
    }
}

impl BookDetails {
    pub fn new() -> Self {
        Self {
            title_offset_y: 0.,
            author_offset_y: 0.,
            description_offset_y: 0.,
        }
    }

    fn make_title(&mut self, ctx: &mut PaintCtx, data: &Book, env: &Env) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::SEMI_BOLD);

        let lmargin = 7.5;

        let mut layout: TextLayout<String> = TextLayout::new();
        layout.set_text(data.get_title());
        layout.set_text_color(Color::WHITE);
        layout.set_font(font);
        layout.set_wrap_width(ctx.size().width - 2. * lmargin - 10.0);
        layout.rebuild_if_needed(ctx.text(), env);

        let pos = Point::new(
            (ctx.size().width / 2.0 - layout.size().width / 2.0).max(lmargin) - 5.0,
            layout.size().height + 5.0,
        );

        self.title_offset_y = pos.y + layout.size().height;

        if let Some(layout) = layout.layout() {
            ctx.draw_text(layout, pos);
        }
    }

    fn make_author(&mut self, ctx: &mut PaintCtx, data: &Book, env: &Env) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(14.0)
            .with_weight(FontWeight::SEMI_BOLD);

        let lmargin = 7.5;

        let mut layout: TextLayout<String> = TextLayout::new();
        layout.set_text(data.get_author());
        layout.set_text_color(Color::WHITE);
        layout.set_font(font);
        layout.set_wrap_width(ctx.size().width - 2. * lmargin - 10.0);
        layout.rebuild_if_needed(ctx.text(), env);

        let pos = Point::new(
            (ctx.size().width / 2.0 - layout.size().width / 2.0).max(lmargin) - 5.0,
            self.title_offset_y + 5.0,
        );

        self.author_offset_y = pos.y + layout.size().height;

        if let Some(layout) = layout.layout() {
            ctx.draw_text(layout, pos);
        }
    }

    fn make_description(&mut self, ctx: &mut PaintCtx, data: &Book, env: &Env) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::THIN);

        let mut layout: TextLayout<String> = TextLayout::new();

        let lmargin = 15.0;

        layout.set_text(data.get_description());
        layout.set_text_color(Color::WHITE);
        layout.set_font(font);
        layout.set_wrap_width(ctx.size().width - 2. * lmargin - 5.0);
        layout.rebuild_if_needed(ctx.text(), env);

        let pos = Point::new(
            (ctx.size().width / 2.0 - layout.size().width / 2.0).max(lmargin) - 5.0,
            self.author_offset_y + 5.0,
        );

        self.description_offset_y = pos.y + layout.size().height;

        if let Some(layout) = layout.layout() {
            ctx.draw_text(layout, pos);
        }
    }
}

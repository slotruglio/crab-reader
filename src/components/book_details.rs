use druid::{
    piet::{CairoText, Text},
    widget::Label,
    BoxConstraints, Color, Env, FontDescriptor, FontFamily, FontWeight, PaintCtx, Point, Widget,
};
use druid::{RenderContext, TextLayout};

use super::{book::Book, library::Library};

pub struct BookDetails {
    title_offset_y: f64,
    description_offset_y: f64,
}

impl Widget<Library> for BookDetails {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Library,
        env: &Env,
    ) {
        ctx.request_paint();
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Library,
        env: &Env,
    ) {
        ctx.request_layout();
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &Library,
        data: &Library,
        env: &Env,
    ) {
        ctx.request_layout();
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Library,
        env: &Env,
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
                self.make_description(ctx, book, env)
            }
        }
    }
}

impl BookDetails {
    pub fn new() -> Self {
        Self {
            title_offset_y: 0.,
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

    fn make_description(&mut self, ctx: &mut PaintCtx, _: &Book, env: &Env) {
        let font_family = CairoText::new()
            .font_family("URW Bookman")
            .unwrap_or(FontFamily::SYSTEM_UI);

        let font = FontDescriptor::new(font_family)
            .with_size(18.0)
            .with_weight(FontWeight::THIN);

        let description = "Questa è una spiegazione lunga. Davvero tanto lunga. Così lunga che mi chiedso se non ci sia un \
        modo migiliore in Rust di memorizzare informazioni testuali lunghe se non usanto delle stringhe. \
        Spero che il compilatore si accorga che deve ottimizzare via questa scritta così lunga e insensata, e non la \
        inserisca davvero come dato statico nel codice. Sarebbe abbastanza imabrazzante. \
        Comunque credo che questa sia una lunghezza abbastanza accetabile. Dovrei essere in grado di vedere che succede quando \
        una stringa così lunga viene visualizzata tramite layout. Speriamo non debba sistemare troppi bug, mi darebbe molto fastidio.";

        let mut layout: TextLayout<String> = TextLayout::new();

        let lmargin = 15.0;

        layout.set_text(description.into());
        layout.set_text_color(Color::WHITE);
        layout.set_font(font);
        layout.set_wrap_width(ctx.size().width - 2. * lmargin - 5.0);
        layout.rebuild_if_needed(ctx.text(), env);

        let pos = Point::new(
            (ctx.size().width / 2.0 - layout.size().width / 2.0).max(lmargin) - 5.0,
            self.title_offset_y + 5.0,
        );

        self.description_offset_y = pos.y + layout.size().height;

        if let Some(layout) = layout.layout() {
            ctx.draw_text(layout, pos);
        }
    }
}

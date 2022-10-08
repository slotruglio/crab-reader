use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Point, RenderContext, Size, TextLayout, UpdateCtx, Widget,
};
use std::rc::Rc;

pub const BOOK_WIDGET_SIZE: Size = Size::new(150.0, 250.0);

#[derive(Clone, Data)]
pub struct Book {
    title: Rc<String>,
    npages: u16,
    cover_path: Rc<String>,
    selected: bool,
}

impl Book {
    pub fn new() -> Self {
        Self {
            title: Rc::new("".to_string()),
            npages: 0,
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

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

// ???
impl Widget<Book> for Book {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut Book, _env: &Env) {
        ()
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &Book, _env: &Env) {
        ()
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Book, _data: &Book, _env: &Env) {
        ()
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
        let rect = ctx.size().to_rounded_rect(7.5);
        let brush_color = Color::BLACK;
        ctx.render_ctx.fill(rect, &brush_color);

        // Text -- Book Title
        let mut tl: TextLayout<String> = TextLayout::new();
        tl.set_text((*self.title).clone());
        tl.set_text_color(Color::WHITE);
        tl.set_text_alignment(druid::piet::TextAlignment::Justified);
        tl.set_text_size(24.0);
        tl.set_wrap_width(ctx.size().width - 10.0);
        tl.rebuild_if_needed(ctx.text(), _env);

        let x = 10.0;
        let y = (ctx.size().height / 2.0) - (tl.size().height / 2.0);
        let pos = Point::new(x, y);

        if let Some(layout) = tl.layout() {
            ctx.render_ctx.draw_text(layout, pos);
        }
    }
}

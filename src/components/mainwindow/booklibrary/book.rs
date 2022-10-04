use druid::{
    piet::{Brush, CairoText, PietText},
    widget::{BackgroundBrush, Container, Flex, Label, LineBreaking, Painter},
    BoxConstraints, Color, Cursor, Data, Event, FontDescriptor, FontFamily, Lens, LifeCycle, Point,
    RenderContext, TextLayout, Widget, WidgetExt, WidgetPod,
};

use crate::components::mainwindow::booklibrary::library::SELECTED_BOOK;

// Book Colors

const DEFAULT_COVER_COLOR: Color = Color::rgb8(30, 30, 30);
const HOVERED_COVER_COLOR: Color = Color::rgb8(60, 60, 60);
const SELECTED_COVER_COLOR: Color = Color::rgb8(90, 90, 90);

const DEFAULT_TEXT_COLOR: Color = Color::rgb8(220, 220, 220);
const HOVERED_TEXT_COLOR: Color = Color::rgb8(190, 190, 190);
const SELECTED_TEXT_COLOR: Color = Color::rgb8(160, 160, 160);

#[derive(Clone, PartialEq, Lens, Data)]
pub struct Book {
    title: String,
    npages: u16,
    idx: u16,
    selected: bool,
}

impl Default for Book {
    fn default() -> Self {
        Self {
            title: "Book Title".into(),
            npages: 0,
            idx: 0,
            selected: false,
        }
    }
}

impl Book {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_npages(mut self, npages: u16) -> Self {
        self.npages = npages;
        self
    }

    pub fn with_idx(mut self, idx: u16) -> Self {
        self.idx = idx;
        self
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn unselect(&mut self) {
        self.selected = false;
    }

    pub fn widget(self) -> impl Widget<Book> {
        BookWidget::from(self)
    }
}

#[derive(Lens)]
pub struct BookWidget {
    inner: WidgetPod<Book, Flex<Book>>,
    state: Book,
}

impl From<Book> for BookWidget {
    fn from(state: Book) -> Self {
        let label = Label::new("");
        let child = Flex::row().with_child(label);
        let inner = WidgetPod::new(child);
        Self { inner, state }
    }
}

impl Widget<Book> for BookWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Book,
        env: &druid::Env,
    ) {
        match event {
            Event::MouseDown(e) => {
                println!(
                    "User clicked on book with idx {} -- {}",
                    data.idx,
                    data.title.clone()
                );
                data.select();
                ctx.submit_notification(SELECTED_BOOK.with(self.state.idx));
                ctx.request_update();
                ctx.request_paint();
            }
            _ => (),
        }
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Book,
        env: &druid::Env,
    ) {
        match event {
            LifeCycle::HotChanged(_) => {
                ctx.request_paint();
            }
            _ => (),
        }
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &Book,
        data: &Book,
        env: &druid::Env,
    ) {
        self.inner.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Book,
        env: &druid::Env,
    ) -> druid::Size {
        let size = bc.constrain((150.0, 250.0));
        let bc = BoxConstraints::tight(size);
        self.inner.layout(ctx, &bc, data, env);
        size
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Book, env: &druid::Env) {
        self.inner.paint(ctx, data, env);
        let size = ctx.size();
        let rect = size.to_rect().to_rounded_rect(5.0);

        let bg_color = if data.selected {
            SELECTED_COVER_COLOR
        } else if ctx.is_hot() {
            HOVERED_COVER_COLOR
        } else {
            DEFAULT_COVER_COLOR
        };

        let text_color = if data.selected {
            SELECTED_TEXT_COLOR
        } else if ctx.is_hot() {
            HOVERED_TEXT_COLOR
        } else {
            DEFAULT_TEXT_COLOR
        };

        ctx.render_ctx.fill(rect, &bg_color);

        let mut tl = TextLayout::new();
        tl.set_text(data.title.clone());
        tl.set_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(18.0));
        tl.set_text_color(text_color);
        tl.set_wrap_width(ctx.size().width - 10.0);
        tl.rebuild_if_needed(ctx.text(), env);

        let x = ctx.size().width / 2.0 - tl.size().width / 2.0;
        let y = (size / 2.0).to_vec2().to_point().y;
        let point = Point::new(x, y);
        tl.draw(ctx, point);
    }
}

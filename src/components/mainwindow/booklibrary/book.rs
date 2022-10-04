use druid::{
    widget::{Container, Flex, Label, LineBreaking},
    BoxConstraints, Color, Cursor, Data, Event, Lens, Widget, WidgetExt, WidgetPod,
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
        let selected = state.selected;
        let label = Label::dynamic(|data: &Book, _env: &_| data.title.clone())
            .with_text_color(if selected {
                SELECTED_TEXT_COLOR
            } else {
                DEFAULT_TEXT_COLOR
            })
            .with_text_size(18.0)
            .with_line_break_mode(LineBreaking::WordWrap)
            .center()
            .background(if selected {
                SELECTED_COVER_COLOR
            } else {
                DEFAULT_COVER_COLOR
            })
            .rounded(7.5)
            .padding(5.0)
            .expand();
        let child = Flex::row().with_flex_child(label, 1.0);
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
            Event::MouseMove(_) => {
                if ctx.is_hot() {
                    ctx.set_cursor(&Cursor::OpenHand);
                } else {
                    ctx.set_cursor(&Cursor::Arrow);
                }
            }
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
    }
}

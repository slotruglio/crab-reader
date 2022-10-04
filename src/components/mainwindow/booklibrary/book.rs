use druid::{
    widget::{Container, Flex, Label, LineBreaking},
    BoxConstraints, Color, Data, Lens, Widget, WidgetExt, WidgetPod,
};

#[derive(Clone, PartialEq, Lens, Data)]
pub struct Book {
    title: String,
    npages: u16,
}

impl Default for Book {
    fn default() -> Self {
        Self {
            title: "Book Title".into(),
            npages: 0,
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
        let label = Label::dynamic(|data: &Book, _env: &_| data.title.clone())
            .with_text_color(Color::WHITE)
            .with_text_size(18.0)
            .with_line_break_mode(LineBreaking::WordWrap)
            .center()
            .padding(5.0)
            .background(Color::BLACK)
            .rounded(5.0)
            .padding(5.0);
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
        dbg!(size);
        size
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Book, env: &druid::Env) {
        self.inner.paint(ctx, data, env);
    }
}

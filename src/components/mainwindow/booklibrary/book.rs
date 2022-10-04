use druid::{
    widget::{Flex, Label},
    Color, Data, Lens, Widget, WidgetExt, WidgetPod,
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
        let inner = Flex::row().with_flex_child(
            Label::dynamic(|data: &Book, _env: &_| data.title.clone())
                .center()
                .border(Color::BLACK, 2.0)
                .padding(20.0)
                .background(Color::GREEN)
                .rounded(5.0)
                .padding(5.0)
                .on_click(|_, data, _env| {
                    dbg!("Clicked {}", data.title.clone());
                }),
            1.0,
        );
        Self {
            inner: WidgetPod::new(inner),
            state,
        }
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
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Book, env: &druid::Env) {
        self.inner.paint(ctx, data, env);
    }
}

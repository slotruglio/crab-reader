use druid::im::Vector;
use druid::{widget::Flex, WidgetPod};
use druid::{Data, Lens, LensExt, Widget, WidgetExt};

use super::book::BookState;

#[derive(Clone, PartialEq, Data, Lens)]
pub struct BookLibraryState {
    books: Vector<BookState>,
}

impl BookLibraryState {
    pub fn new() -> Self {
        Self {
            books: Vector::new(),
        }
    }

    pub fn with_books(&mut self, books: Vector<BookState>) -> &mut Self {
        self.books = books;
        self
    }

    pub fn get(&mut self) -> Self {
        self.clone()
    }

    pub fn build(&self) -> BookLibraryWidget {
        let mut row = Flex::row();
        for (idx, book) in self.books.iter().enumerate() {
            row.add_flex_child(
                book.clone()
                    .build()
                    .padding(5.0)
                    .lens(BookLibraryState::books.index(idx)),
                1.0,
            );
        }
        BookLibraryWidget {
            inner: WidgetPod::new(row),
        }
    }
}

#[derive(Lens)]
pub struct BookLibraryWidget {
    inner: WidgetPod<BookLibraryState, Flex<BookLibraryState>>,
}

impl Widget<BookLibraryState> for BookLibraryWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut BookLibraryState,
        env: &druid::Env,
    ) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &BookLibraryState,
        env: &druid::Env,
    ) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &BookLibraryState,
        data: &BookLibraryState,
        env: &druid::Env,
    ) {
        if old_data != data {
            self.inner.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &BookLibraryState,
        env: &druid::Env,
    ) -> druid::Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &BookLibraryState, env: &druid::Env) {
        self.inner.paint(ctx, data, env);
    }
}

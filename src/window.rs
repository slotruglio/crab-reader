use druid::{
    widget::{Flex, Label},
    Data, Lens, UnitPoint, Widget, WidgetExt, WidgetPod,
};

pub struct Header {
    pub inner: WidgetPod<HeaderState, Flex<HeaderState>>,
} // Placeholder
#[derive(Clone, Data, Lens, PartialEq)]
pub struct HeaderState {
    pub username: String,
    pub nbooks: u32,
} // Placeholder

pub struct BookLibrary {
    pub inner: WidgetPod<BookLibraryState, Flex<BookLibraryState>>,
} // Placeholder
#[derive(Clone, Data, Lens, PartialEq)]
pub struct BookLibraryState {} // Placeholder

#[derive(Clone, Data, Lens, PartialEq)]
pub struct CrabReaderWindowState {
    pub header_state: HeaderState,
    pub library_state: BookLibraryState,
} // Placeholder
pub struct CrabReaderWindow {
    pub header: WidgetPod<HeaderState, Header>,
    pub library: WidgetPod<BookLibraryState, BookLibrary>,
}

impl Widget<CrabReaderWindowState> for CrabReaderWindow {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut CrabReaderWindowState,
        env: &druid::Env,
    ) {
        // Propagate any evenet down
        self.library.event(ctx, event, &mut data.library_state, env);
        self.header.event(ctx, event, &mut data.header_state, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &CrabReaderWindowState,
        env: &druid::Env,
    ) {
        // This function should call `lifecycle` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // I.E: if we're reading a book, we need not to call these two, but the lifecycle of the reader
        self.header.lifecycle(ctx, event, &data.header_state, env);
        self.library.lifecycle(ctx, event, &data.library_state, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &CrabReaderWindowState,
        data: &CrabReaderWindowState,
        env: &druid::Env,
    ) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // I.E: if we're reading a book, we need not to call these two, but the lifecycle of the reader
        if old_data != data {
            self.header.update(ctx, &data.header_state, env);
            self.library.update(ctx, &data.library_state, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &CrabReaderWindowState,
        env: &druid::Env,
    ) -> druid::Size {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // Can't be empty
        self.header.layout(ctx, bc, &data.header_state, env);
        self.library.layout(ctx, bc, &data.library_state, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &CrabReaderWindowState, env: &druid::Env) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // Can't be empty
        self.header.paint(ctx, &data.header_state, env);
        self.library.paint(ctx, &data.library_state, env);
    }
}

impl Widget<BookLibraryState> for BookLibrary {
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
        // This function should call `lifecycle` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &BookLibraryState,
        data: &BookLibraryState,
        env: &druid::Env,
    ) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // For now, do nothing
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
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // Can't be empty
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &BookLibraryState, env: &druid::Env) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // Can't be empty
        self.inner.paint(ctx, data, env);
    }
}

impl Widget<HeaderState> for Header {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut HeaderState,
        env: &druid::Env,
    ) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &HeaderState,
        env: &druid::Env,
    ) {
        // This function should call `lifecycle` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &HeaderState,
        data: &HeaderState,
        env: &druid::Env,
    ) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        if old_data != data {
            self.inner.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &HeaderState,
        env: &druid::Env,
    ) -> druid::Size {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // Can't be empty
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &HeaderState, env: &druid::Env) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // Can't be empty
        self.inner.paint(ctx, data, env);
    }
}

impl CrabReaderWindow {
    pub fn build() -> Self {
        CrabReaderWindow {
            header: WidgetPod::new(Header::build()),
            library: WidgetPod::new(BookLibrary::build()),
        }
    }
}

impl Header {
    pub fn build() -> Self {
        let left_label = Label::new("Static Left String")
            .padding(10.0)
            .align_horizontal(UnitPoint::LEFT)
            .align_vertical(UnitPoint::CENTER);
        let right_label = Label::new("Static Right String")
            .padding(10.0)
            .align_horizontal(UnitPoint::RIGHT)
            .align_vertical(UnitPoint::CENTER);
        Header {
            inner: WidgetPod::new(
                Flex::row()
                    .with_flex_child(left_label, 1.0)
                    .with_flex_child(right_label, 1.0),
            ),
        }
    }
}

impl BookLibrary {
    pub fn build() -> Self {
        let mut library = Flex::column();
        for i in 0..10 {
            let label = Label::new(format!("Book {}", i))
                .padding(10.0)
                .align_horizontal(UnitPoint::LEFT)
                .align_vertical(UnitPoint::CENTER);
            library.add_child(label);
        }
        BookLibrary {
            inner: WidgetPod::new(library),
        }
    }
}

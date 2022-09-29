use druid::{
    widget::{Container, Flex, Label},
    Color, Data, Lens, Widget, WidgetExt, WidgetPod,
};

pub struct BookLibrary {
    pub inner: WidgetPod<BookLibraryState, Flex<BookLibraryState>>,
} // Placeholder
#[derive(Clone, Data, Lens, PartialEq)]
pub struct BookLibraryState {} // Placeholder

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
        _old_data: &BookLibraryState,
        data: &BookLibraryState,
        env: &druid::Env,
    ) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        self.inner.update(ctx, data, env);
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
        let size = self.inner.layout(ctx, bc, data, env);

        // ??
        if bc.is_height_bounded() && bc.is_width_bounded() {
            bc.max()
        } else {
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &BookLibraryState, env: &druid::Env) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        self.inner.paint(ctx, data, env);
    }
}

impl BookLibrary {
    pub fn build() -> Flex<BookLibraryState> {
        let mut library = Flex::column();
        for i in 0..10 {
            let label = Label::new(format!("Book {}", i)).center();
            let adjusted = label
                .padding(2.0)
                .background(Color::BLUE)
                .rounded(5.0)
                .center();
            library.add_flex_child(adjusted, 2.0);
            library.add_flex_spacer(0.2);
        }

        let container_library = Container::new(library.padding(10.0));
        Flex::row().with_flex_child(container_library, 1.0)
    }
}

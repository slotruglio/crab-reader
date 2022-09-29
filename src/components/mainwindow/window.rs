use super::{
    booklibrary::{library::BookLibrary, library::BookLibraryState},
    header::{Header, HeaderState},
};

use druid::{
    widget::{Container, Flex, TextBox},
    Color, Data, Lens, LensExt, Widget, WidgetExt, WidgetPod,
};

#[derive(Clone, Data, Lens, PartialEq)]
pub struct CrabReaderWindowState {
    pub header_state: HeaderState,
    pub library_state: BookLibraryState,
} // Placeholder
pub struct CrabReaderWindow {
    pub header: WidgetPod<HeaderState, Flex<HeaderState>>,
    pub library: WidgetPod<BookLibraryState, Flex<BookLibraryState>>,
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
        _old_data: &CrabReaderWindowState,
        data: &CrabReaderWindowState,
        env: &druid::Env,
    ) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // I.E: if we're reading a book, we need not to call these two, but the lifecycle of the reader
        self.header.update(ctx, &data.header_state, env);
        self.library.update(ctx, &data.library_state, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &CrabReaderWindowState,
        env: &druid::Env,
    ) -> druid::Size {
        // This function should call `layout` on all children widgets
        // The key point is that we can decide wheter to or not for a single child

        let size_one = self.header.layout(ctx, bc, &data.header_state, env);
        let size_two = self.library.layout(ctx, bc, &data.library_state, env);
        let size = size_one + size_two;

        if bc.is_height_bounded() && bc.is_width_bounded() {
            bc.max()
        } else {
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &CrabReaderWindowState, env: &druid::Env) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // Can't be empty
        self.header.paint(ctx, &data.header_state, env);
        self.library.paint(ctx, &data.library_state, env);
    }
}

impl CrabReaderWindow {
    pub fn user_input() -> impl Widget<CrabReaderWindowState> {
        let username_input = TextBox::new()
            .with_placeholder("Username")
            .lens(CrabReaderWindowState::header_state.then(HeaderState::username));

        let nbooks_input = TextBox::new()
            .with_placeholder("Number of books")
            .lens(CrabReaderWindowState::header_state.then(HeaderState::nbooks));

        let row = Flex::row()
            .with_child(username_input)
            .with_child(nbooks_input)
            .padding(5.0);

        row
    }

    pub fn build() -> Flex<CrabReaderWindowState> {
        let header = Header::build().padding(5.0);
        let container_header = Container::new(header)
            .background(Color::RED)
            .rounded(10.0)
            .lens(CrabReaderWindowState::header_state);
        let inner = Flex::column()
            .with_child(Self::user_input())
            .with_child(container_header)
            .with_flex_child(
                BookLibrary::build().lens(CrabReaderWindowState::library_state),
                1.0,
            )
            .padding(10.0);
        Flex::row().with_flex_child(inner, 1.0)
    }
}

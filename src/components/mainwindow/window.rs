use super::{
    booklibrary::library::BookLibraryState,
    header::{Header, HeaderState},
};

use druid::{
    widget::{Container, Flex, Label, LineBreaking, TextBox},
    Color, Data, FontDescriptor, FontFamily, FontWeight, Lens, LensExt, TextAlignment, Widget,
    WidgetExt, WidgetPod,
};

#[derive(Clone, Data, Lens, PartialEq)]
pub struct CrabReaderWindowState {
    pub header_state: HeaderState,
    pub library_state: BookLibraryState,
} // Placeholde
pub struct CrabReaderWindow {
    pub header: WidgetPod<HeaderState, Flex<HeaderState>>,
    pub library: WidgetPod<BookLibraryState, Flex<BookLibraryState>>,
}

impl CrabReaderWindowState {
    pub fn widget(&self) -> Flex<CrabReaderWindowState> {
        let bg_color = Color::rgb8(122, 122, 122);
        let round_factor = 5.0;
        let pad_factor = 10.0;

        let lib = self.library_state.build();
        let header = Header::build().padding(pad_factor);

        let right_label = Label::new("Select a book to see more details.")
            .with_text_color(Color::rgb8(230, 230, 230))
            .with_text_size(22.0)
            .with_line_break_mode(LineBreaking::WordWrap)
            .with_font(
                FontDescriptor::new(FontFamily::SYSTEM_UI)
                    .with_weight(FontWeight::MEDIUM)
                    .with_size(24.0),
            )
            .with_text_alignment(TextAlignment::End);

        let container_header = Container::new(header)
            .background(bg_color.clone())
            .rounded(10.0)
            .padding(pad_factor)
            .lens(CrabReaderWindowState::header_state);

        let inner_left = Flex::column()
            .with_flex_child(
                Container::new(lib)
                    .background(bg_color.clone())
                    .rounded(pad_factor)
                    .lens(CrabReaderWindowState::library_state),
                1.0,
            )
            .background(bg_color.clone())
            .rounded(round_factor)
            .padding(pad_factor);

        let inner_right = Flex::column()
            .with_child(right_label)
            .center()
            .background(bg_color.clone())
            .rounded(round_factor)
            .padding(pad_factor)
            .expand();

        let inner = Flex::row()
            .with_flex_child(inner_left, 3.0)
            .with_flex_child(inner_right, 1.0);

        Flex::column()
            .with_child(container_header)
            .with_spacer(5.0)
            .with_flex_child(inner, 1.0)
    }
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
    #[allow(dead_code)]
    pub fn user_input() -> impl Widget<CrabReaderWindowState> {
        let username_input = TextBox::new()
            .with_placeholder("Username")
            .lens(CrabReaderWindowState::header_state.then(HeaderState::username));

        let nbooks_input = TextBox::new()
            .with_placeholder("Number of books")
            .lens(CrabReaderWindowState::header_state.then(HeaderState::nbooks));

        let row = Flex::row()
            .with_child(username_input)
            .with_flex_child(nbooks_input, 1.0)
            .padding(5.0);

        row
    }
}

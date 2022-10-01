use super::{booklibrary::library::BookLibraryState, header::Header};

use druid::{
    widget::{Container, Flex, Label, LineBreaking},
    Color, Data, FontDescriptor, FontFamily, FontWeight, Lens, TextAlignment, Widget, WidgetExt,
    WidgetPod,
};

#[derive(Clone, Data, Lens, PartialEq)]
pub struct CrabReaderWindowState {
    pub library_state: BookLibraryState,
    pub username: String,
}

pub struct CrabReaderWindow {
    pub header: WidgetPod<CrabReaderWindowState, Flex<CrabReaderWindowState>>,
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
            .padding(pad_factor);

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
            .with_spacer(1.0)
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
        self.header.event(ctx, event, data, env);
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
        self.header.lifecycle(ctx, event, &data, env);
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
        self.header.update(ctx, &data, env);
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

        let size_one = self.header.layout(ctx, bc, &data, env);
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
        self.header.paint(ctx, data, env);
        self.library.paint(ctx, &data.library_state, env);
    }
}

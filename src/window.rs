use druid::{
    widget::{Container, Flex, Label, LineBreaking, TextBox},
    Color, Command, Data, Env, Lens, LensExt, LocalizedString, MenuDesc, MenuItem, Selector,
    Target, UnitPoint, Widget, WidgetExt, WidgetPod,
};

pub struct Header {
    pub inner: WidgetPod<HeaderState, Flex<HeaderState>>,
} // Placeholder
#[derive(Clone, Data, Lens, PartialEq)]
pub struct HeaderState {
    pub username: String,
    pub nbooks: String,
} // Placeholder

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
        _old_data: &HeaderState,
        data: &HeaderState,
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
        data: &HeaderState,
        env: &druid::Env,
    ) -> druid::Size {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // Can't be empty
        self.inner.layout(ctx, bc, data, env);

        if bc.is_height_bounded() && bc.is_width_bounded() {
            bc.max()
        } else {
            bc.constrain((100.0, 100.0))
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &HeaderState, env: &druid::Env) {
        // This function should call `update` on all children widgets
        // The key point is that we can decide wheter to or not for a single child
        // Can't be empty
        self.inner.paint(ctx, data, env);
    }
}

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

impl Header {
    pub fn build() -> Flex<HeaderState> {
        let mut left_label_inner = Label::dynamic(|data: &HeaderState, _: &Env| {
            format!("Bentornato, {}", data.username).into()
        });
        left_label_inner.set_line_break_mode(LineBreaking::WordWrap);

        let mut right_label_inner = Label::dynamic(|data: &HeaderState, _: &Env| {
            format!("Hai {} libri da leggere.", data.nbooks).into()
        });
        right_label_inner.set_line_break_mode(LineBreaking::WordWrap);

        let left_label = left_label_inner
            .padding(10.0)
            .align_horizontal(UnitPoint::LEFT)
            .align_vertical(UnitPoint::TOP);
        let right_label = right_label_inner
            .padding(10.0)
            .align_horizontal(UnitPoint::RIGHT)
            .align_vertical(UnitPoint::TOP);

        Flex::row()
            .with_flex_child(left_label, 1.0)
            .with_flex_child(right_label, 1.0)
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

pub fn make_menu() -> MenuDesc<CrabReaderWindowState> {
    type Button = MenuItem<CrabReaderWindowState>;

    let file_selector = Selector::new("main-window-menu-file");
    let file_command = Command::new(file_selector, (), Target::Auto);
    let file_button: Button = MenuItem::new(LocalizedString::new("Files"), file_command);

    let edit_selector = Selector::new("main-window-menu-edit");
    let edit_command = Command::new(edit_selector, (), Target::Auto);
    let edit_button: Button = MenuItem::new(LocalizedString::new("Edit"), edit_command);

    let view_selector = Selector::new("main-window-menu-edit");
    let view_command = Command::new(view_selector, (), Target::Auto);
    let view_button: Button = MenuItem::new(LocalizedString::new("View"), view_command);

    let about_selector = Selector::new("main-window-menu-about");
    let about_command = Command::new(about_selector, (), Target::Auto);
    let about_button: Button = MenuItem::new(LocalizedString::new("About"), about_command);

    let checked_selector = Selector::new("submenu-checked");
    let checked_command = Command::new(checked_selector, (), Target::Auto);
    let checked_button: Button =
        MenuItem::new(LocalizedString::new("Checked"), checked_command).selected();

    let disabled_selector = Selector::new("submenu-disabled");
    let disabled_command = Command::new(disabled_selector, (), Target::Auto);
    let disabled_button: Button =
        MenuItem::new(LocalizedString::new("Disabled"), disabled_command).disabled();

    // Da testare con lo stato per vedere come interagisce...

    let submenu = MenuDesc::new(LocalizedString::new("Submenu"))
        .append(checked_button)
        .append(disabled_button);

    MenuDesc::new(LocalizedString::new("Menu"))
        .append(file_button)
        .append(edit_button)
        .append(view_button)
        .append(about_button)
        .append(submenu)
}

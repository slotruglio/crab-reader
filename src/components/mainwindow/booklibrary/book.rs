use druid::Color;
use druid::{widget::Flex, Data, Lens, Widget, WidgetPod};

pub const DEFAULT_BOOK_COLOR: Color = Color::GREEN;
pub const HOVER_BOOK_COLOR: Color = Color::PURPLE;

pub struct BookItem {
    pub inner: WidgetPod<BookItemState, Flex<BookItem>>,
}

#[derive(Clone, PartialEq, Data, Lens)]
pub struct BookItemState {
    title: String,
    npages: String,
    color: druid::Color,
}

impl Widget<BookItemState> for BookItem {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut BookItemState,
        _env: &druid::Env,
    ) {
        ()
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &BookItemState,
        _env: &druid::Env,
    ) {
        ()
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &BookItemState,
        _data: &BookItemState,
        _env: &druid::Env,
    ) {
        todo!()
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &BookItemState,
        _env: &druid::Env,
    ) -> druid::Size {
        if bc.is_height_bounded() && bc.is_width_bounded() {
            () // ??
        }
        druid::Size::new(100.0, 100.0)
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &BookItemState, _env: &druid::Env) {
        ()
    }
}

impl BookItemState {
    pub fn new() -> Self {
        Self {
            npages: "".into(),
            title: "".into(),
            color: DEFAULT_BOOK_COLOR,
        }
    }

    pub fn with_title(&mut self, title: String) -> &mut BookItemState {
        self.title = title;
        self
    }

    pub fn with_npages(&mut self, n: String) -> &mut BookItemState {
        self.npages = n;
        self
    }
}

impl BookItem {
    pub fn build() -> Self {
        // OBJECTIVE: Change book color on hover
        todo!()
    }
}

use druid::{
    piet::PaintBrush,
    widget::{Container, Flex, Label, Painter},
    Color, Data, Env, Event, Lens, RenderContext, TextLayout, Widget, WidgetExt, WidgetPod,
};

pub const STRCLR_DEFAULT: Color = Color::rgb8(70, 70, 70);
pub const STRCLR_HOVER: Color = Color::rgb8(110, 110, 110);
pub const STRCLR_PRESSED: Color = Color::rgb8(200, 200, 200);

pub const BGCOLOR_DEFAULT: Color = Color::rgb8(50, 50, 50);
pub const BGCOLOR_HOVER: Color = Color::rgb8(70, 70, 70);
pub const BGCOLOR_PRESSED: Color = Color::rgb8(20, 20, 20);

#[derive(Clone, PartialEq, Data, Lens)]
pub struct BookState {
    title: String,
    bg_color: Color,
    str_color: Color,
    active: bool,
    hovered: bool,
}

impl From<BookState> for BookWidget {
    fn from(state: BookState) -> Self {
        state.build()
    }
}

pub struct BookWidget {
    inner: WidgetPod<BookState, Flex<BookState>>,
}

impl BookState {
    pub fn new() -> Self {
        Self {
            title: "book_title".to_string(),
            bg_color: BGCOLOR_DEFAULT,
            str_color: STRCLR_DEFAULT,
            active: false,
            hovered: false,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn build(&self) -> BookWidget {
        let label = Label::new(self.title.clone()).padding(5.0);
        let container = Container::new(label);
        let row = Flex::row().with_flex_child(container, 1.0);
        BookWidget {
            inner: WidgetPod::new(row),
        }
    }
}

impl Widget<BookState> for BookWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut BookState,
        env: &Env,
    ) {
        self.inner.event(ctx, event, data, env);
        data.hovered = ctx.is_hot();
        data.active = match event {
            Event::MouseDown(_) => true,
            Event::MouseUp(_) => false,
            _ => data.active,
        };

        (data.bg_color, data.str_color) = match (data.hovered, data.active) {
            (true, true) => (BGCOLOR_PRESSED, STRCLR_PRESSED),
            (true, false) => (BGCOLOR_HOVER, STRCLR_HOVER),
            (false, _) => (BGCOLOR_DEFAULT, STRCLR_DEFAULT),
        };

        ctx.request_update();
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &BookState,
        env: &Env,
    ) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &BookState,
        data: &BookState,
        env: &Env,
    ) {
        self.inner.update(ctx, data, env);
        if old_data != data {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &BookState,
        env: &Env,
    ) -> druid::Size {
        let max_w = 150.0;
        let max_h = 250.0;
        let mut size = self.inner.layout(ctx, bc, data, env);
        if size.width > max_w {
            size.width = max_w;
        }
        size.height = max_h;
        size
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &BookState, env: &Env) {
        self.inner.paint(ctx, data, env);
        Painter::new(|ctx: &mut druid::PaintCtx, data: &BookState, env| {
            // Paint the background first
            let rect = ctx.size().to_rect().to_rounded_rect(5.0);
            let brush: PaintBrush = data.bg_color.clone().into();
            ctx.fill(rect, &brush);

            // Draw the text on top
            let mut layout = TextLayout::<String>::from_text(data.title.clone());
            layout.rebuild_if_needed(ctx.text(), env);
            let center = ctx.size().to_rect().center() - layout.size().to_vec2() / 2.0;
            ctx.draw_text(layout.layout().unwrap(), center);
        })
        .paint(ctx, data, env);
    }
}

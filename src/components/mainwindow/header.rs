use druid::{
    widget::{Flex, Label, LineBreaking},
    Data, Env, Lens, UnitPoint, Widget, WidgetExt, WidgetPod,
};

pub struct Header {
    pub inner: WidgetPod<HeaderState, Flex<HeaderState>>,
} // Placeholder
#[derive(Clone, Data, Lens, PartialEq)]
pub struct HeaderState {
    pub username: String,
    pub nbooks: String,
} // Placeholder

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

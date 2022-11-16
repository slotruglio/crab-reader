use druid::{
    widget::Label, Affine, Color, Cursor::NotAllowed, Data, Env, Event, EventCtx,
    LifeCycle::HotChanged, RenderContext, Size, Widget,
};

use super::colors;

pub struct RoundedButton<T> {
    label: Label<T>,
    label_size: Size,
    color: Color,
    hot_color: Color,
    active_color: Color,
    status: ButtonStatus,
    on_click: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
    disable_condition: Box<dyn Fn(&T, &Env) -> bool>,
}

#[derive(PartialEq)]
enum ButtonStatus {
    Normal,
    Hot,
    Active,
    Disabled,
}

impl<T: Data> RoundedButton<T> {
    fn new(label: Label<T>) -> Self {
        Self {
            label: label.with_line_break_mode(druid::widget::LineBreaking::WordWrap),
            label_size: Size::ZERO,
            color: colors::NORMAL_GRAY,
            hot_color: colors::HOT_GRAY,
            active_color: colors::ACTIVE_GRAY,
            status: ButtonStatus::Normal,
            on_click: Box::new(|_, _, _| {}),
            disable_condition: Box::new(|_, _| false),
        }
        .with_text_color(colors::TEXT_WHITE)
    }

    pub fn dynamic(closure: impl Fn(&T, &Env) -> String + 'static) -> Self {
        let label = Label::dynamic(closure);
        RoundedButton::new(label)
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        let label = Label::new(text.into());
        RoundedButton::new(label)
    }

    pub fn with_text_size(mut self, size: f64) -> Self {
        self.label.set_text_size(size);
        self
    }

    /// The hot color is the color for when the button is hovered
    pub fn with_hot_color(mut self, color: impl Into<Color>) -> Self {
        self.hot_color = color.into();
        self
    }

    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }

    pub fn with_active_color(mut self, color: impl Into<Color>) -> Self {
        self.active_color = color.into();
        self
    }

    fn is_disabled(&self) -> bool {
        self.status == ButtonStatus::Disabled
    }

    pub fn with_on_click(
        mut self,
        on_click: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> Self {
        self.on_click = Box::new(on_click);
        self
    }

    pub fn with_text_color(mut self, color: impl Into<Color>) -> Self {
        self.label.set_text_color(color.into());
        self
    }

    pub fn disabled_if(mut self, closure: impl Fn(&T, &Env) -> bool + 'static) -> Self {
        self.disable_condition = Box::new(closure);
        self
    }
}

impl<T: Data> Widget<T> for RoundedButton<T> {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut T, env: &Env) {
        self.label.event(ctx, event, data, env);
        let disable = (self.disable_condition)(data, env);
        if disable {
            self.status = ButtonStatus::Disabled;
        } else {
            self.status = ButtonStatus::Normal;
        };

        if self.is_disabled() {
            ctx.set_cursor(&NotAllowed);
        }

        match event {
            Event::MouseDown(_) => {
                if self.is_disabled() {
                    return;
                }
                self.status = ButtonStatus::Active;
                ctx.request_paint();
                (self.on_click)(ctx, data, env);
            }
            Event::MouseUp(_) => {
                if self.is_disabled() {
                    return;
                }
                self.status = ButtonStatus::Hot;
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &Env,
    ) {
        self.label.lifecycle(ctx, event, data, env);

        match event {
            HotChanged(_) => {
                if self.is_disabled() {
                    return;
                }
                if ctx.is_hot() {
                    self.status = ButtonStatus::Hot;
                    ctx.request_paint();
                } else {
                    self.status = ButtonStatus::Normal;
                    ctx.request_paint();
                }
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &Env,
    ) -> druid::Size {
        let label_size = self.label.layout(ctx, bc, data, env);
        let mut w = label_size.width + 10.0;
        let mut h = label_size.height + 10.0;

        if bc.is_width_bounded() {
            w = bc.max().width;
        }

        if bc.is_height_bounded() {
            h = bc.max().height;
        }

        self.label_size = label_size;
        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &Env) {
        let rrect = ctx.size().to_rect().to_rounded_rect(5.0);

        let color = match self.status {
            ButtonStatus::Normal => &self.color,
            ButtonStatus::Hot => &self.hot_color,
            ButtonStatus::Active => &self.active_color,
            _ => &self.color,
        };
        ctx.fill(rrect, color);

        let label_origin = ctx.size().to_rect().center() - self.label_size.to_vec2() / 2.0;
        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_origin.to_vec2()));
            self.label.paint(ctx, data, env);
        });

        if self.is_disabled() {
            ctx.fill(rrect, &Color::rgba8(0, 0, 0, 80));
        }
    }
}

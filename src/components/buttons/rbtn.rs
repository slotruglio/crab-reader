use druid::{
    theme, widget::Label, Affine, Color, Cursor::NotAllowed, Data, Env, Event, EventCtx,
    KeyOrValue, LifeCycle::HotChanged, Point, RenderContext, Size, Widget, WidgetPod,
};

use crate::utils::colors::{self};

pub struct RoundedButton<T> {
    label: WidgetPod<T, Label<T>>,
    label_size: Size,
    status: ButtonStatus,
    on_click: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
    disable_condition: Box<dyn Fn(&T, &Env) -> bool>,
    toggle_condition: Box<dyn Fn(&T, &Env) -> bool>,
    primary: bool,
}

#[derive(PartialEq, Clone)]
enum ButtonStatus {
    Normal,
    Hot,
    Active,
    Disabled,
}

impl<T: Data> RoundedButton<T> {
    fn new(label: Label<T>) -> Self {
        Self {
            label: WidgetPod::new(
                label
                    .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
                    .with_text_color(colors::ON_PRIMARY),
            ),
            label_size: Size::ZERO,
            status: ButtonStatus::Normal,
            on_click: Box::new(|_, _, _| {}),
            disable_condition: Box::new(|_, _| false),
            toggle_condition: Box::new(|_, _| false),
            primary: true,
        }
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
        self.label.widget_mut().set_text_size(size);
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

    pub fn with_text_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.label.widget_mut().set_text_color(color.into());
        self
    }

    pub fn disabled_if(mut self, closure: impl Fn(&T, &Env) -> bool + 'static) -> Self {
        self.disable_condition = Box::new(closure);
        self
    }

    pub fn with_font(mut self, font: impl Into<druid::FontDescriptor>) -> Self {
        self.label.widget_mut().set_font(font.into());
        self
    }

    pub fn with_toggle(mut self, closure: impl Fn(&T, &Env) -> bool + 'static) -> Self {
        self.toggle_condition = Box::new(closure);
        self
    }

    pub fn secondary(mut self) -> Self {
        self.primary = false;
        self.label.widget_mut().set_text_color(colors::ON_SECONDARY);
        self
    }

    fn get_fill_color(&self, ctx: &mut druid::PaintCtx, data: &T, env: &Env) -> Color {
        if self.primary {
            self.get_fill_color_primary(ctx, data, env)
        } else {
            self.get_fill_color_secondary(ctx, data, env)
        }
    }

    fn get_fill_color_primary(&self, _: &mut druid::PaintCtx, data: &T, env: &Env) -> Color {
        if (self.toggle_condition)(data, env) {
            return env.get(colors::PRIMARY_VARIANT);
        }

        match self.status {
            ButtonStatus::Active => env.get(colors::PRIMARY_VARIANT),
            ButtonStatus::Hot => env.get(colors::PRIMARY_ACCENT),
            ButtonStatus::Normal => env.get(colors::PRIMARY),
            ButtonStatus::Disabled => env.get(colors::PRIMARY_VARIANT).with_alpha(0.5),
        }
    }

    fn get_fill_color_secondary(&self, _: &mut druid::PaintCtx, data: &T, env: &Env) -> Color {
        if (self.toggle_condition)(data, env) {
            return env.get(colors::SECONDARY_VARIANT);
        }

        match self.status {
            ButtonStatus::Active => env.get(colors::SECONDARY_VARIANT),
            ButtonStatus::Hot => env.get(colors::SECONDARY_ACCENT),
            ButtonStatus::Normal => env.get(colors::SECONDARY),
            ButtonStatus::Disabled => env.get(colors::SECONDARY_VARIANT).with_alpha(0.7),
        }
    }
}

impl<T: Data> Widget<T> for RoundedButton<T> {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut T, env: &Env) {
        self.label.event(ctx, event, data, env);

        match event {
            Event::MouseDown(_) => {
                if self.is_disabled() {
                    return;
                }
                self.status = ButtonStatus::Active;
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

        if self.status == ButtonStatus::Active && !ctx.is_hot() {
            self.status = ButtonStatus::Normal;
            ctx.request_paint();
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
                if self.is_disabled() || &self.status == &ButtonStatus::Active {
                    return;
                }

                if ctx.is_hot() {
                    self.status = ButtonStatus::Hot;
                } else {
                    self.status = ButtonStatus::Normal;
                }
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _: &T, data: &T, env: &Env) {
        self.label.update(ctx, data, env);

        let disable = (self.disable_condition)(data, env);
        if disable {
            self.status = ButtonStatus::Disabled;
            ctx.set_cursor(&NotAllowed);
        } else if &self.status == &ButtonStatus::Active {
            // nop
        } else if ctx.is_hot() {
            self.status = ButtonStatus::Hot;
        } else {
            self.status = ButtonStatus::Normal;
            ctx.clear_cursor();
        };
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &Env,
    ) -> druid::Size {
        let xpad = env.get(theme::WIDGET_PADDING_HORIZONTAL);
        let ypad = env.get(theme::WIDGET_PADDING_VERTICAL);
        let lbc = bc.shrink((xpad, ypad)).loosen();
        let label_size = self.label.layout(ctx, &lbc, data, env);

        let w = if bc.is_width_bounded() {
            bc.max().width
        } else {
            label_size.width + xpad
        };

        let h = if bc.is_height_bounded() {
            bc.max().height
        } else {
            label_size.height + ypad
        };

        // Non ho idea di perché quel -1 sia necessario...
        let x_origin = label_size.width / 2.0 * -1.;
        let y_origin = label_size.height / 2.0 * -1.;
        let origin = Point::new(x_origin, y_origin);
        self.label.set_origin(ctx, data, env, origin);
        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &Env) {
        let rrect = ctx.size().to_rect().to_rounded_rect(5.0);

        let color = self.get_fill_color(ctx, data, env);
        ctx.fill(rrect, &color);

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

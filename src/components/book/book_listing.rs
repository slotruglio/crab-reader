use druid::{
    widget::{Label, LineBreaking},
    BoxConstraints, Command, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, Size, Target, UpdateCtx, Widget, WidgetPod,
};

use crate::{
    traits::gui::GUIBook,
    utils::{colors, fonts, library::SELECTED_BOOK_SELECTOR},
};

pub struct BookListing<T> {
    is_hot: bool,
    title_label: WidgetPod<T, Label<T>>,
    page_cnt_label: WidgetPod<T, Label<T>>,
    star: WidgetPod<T, Label<T>>,
}

impl<T: GUIBook> BookListing<T> {
    pub fn new() -> Self {
        let title_label = Label::dynamic(|data: &T, _| format!("{}", data.get_title().to_string()))
            .with_font(fonts::Font::default().md().bold().get())
            .with_line_break_mode(LineBreaking::WordWrap)
            .with_text_color(colors::ON_PRIMARY);

        let page_cnt_label = Label::dynamic(|data: &T, _| {
            format!(
                "{}/{} pagine lette",
                data.get_number_of_read_pages(),
                data.get_number_of_pages()
            )
        })
        .with_font(fonts::Font::default().sm().get())
        .with_line_break_mode(LineBreaking::WordWrap)
        .with_text_color(colors::ON_PRIMARY);

        let star_label =
            Label::dynamic(|data: &T, _| if data.is_favorite() { "🌟" } else { "" }.into())
                .with_font(fonts::Font::default().md().emoji().get());

        Self {
            is_hot: false,
            title_label: WidgetPod::new(title_label),
            page_cnt_label: WidgetPod::new(page_cnt_label),
            star: WidgetPod::new(star_label),
        }
    }
}

impl<B: GUIBook + Data> Widget<B> for BookListing<B> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut B, env: &Env) {
        self.title_label.event(ctx, event, data, env);
        self.page_cnt_label.event(ctx, event, data, env);
        self.star.event(ctx, event, data, env);

        match event {
            Event::MouseDown(_) => {
                data.select();
                ctx.set_handled();
                let cmd = Command::new(
                    SELECTED_BOOK_SELECTOR.into(),
                    Some(data.get_index()),
                    Target::Auto,
                );
                ctx.submit_notification(cmd);
                ctx.request_paint();
            }
            Event::MouseMove(_) => {
                self.is_hot = ctx.is_hot();
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &B, env: &Env) {
        self.title_label.lifecycle(ctx, event, data, env);
        self.page_cnt_label.lifecycle(ctx, event, data, env);
        self.star.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &B, data: &B, env: &Env) {
        if !old_data.same(data) || ctx.env_changed() {
            self.title_label.update(ctx, data, env);
            self.page_cnt_label.update(ctx, data, env);
            self.star.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &B, env: &Env) -> Size {
        let maxh = 1000.0; // ?
        let minh = 10.0; // ?

        let w = if bc.is_width_bounded() {
            bc.max().width
        } else {
            bc.min().width.max(10.)
        };

        let title_w = w * 0.65;
        let star_w = w * 0.1;
        let page_cnt_w = w * 0.25;

        let title_bc = BoxConstraints::new((title_w, minh).into(), (title_w, maxh).into());
        let page_cnt_bc = BoxConstraints::new((minh, minh).into(), (page_cnt_w, maxh).into());
        let star_bc = BoxConstraints::new((star_w, minh).into(), (star_w, maxh).into());

        let title_label_size = self.title_label.layout(ctx, &title_bc, data, env);
        let page_cnt_label_size = self.page_cnt_label.layout(ctx, &page_cnt_bc, data, env);
        let star_size = self.star.layout(ctx, &star_bc, data, env);

        let h = title_label_size.height.max(page_cnt_label_size.height) + 20.0;

        let title_origin = (10.0, h / 2. - title_label_size.height / 2.).into();
        self.title_label.set_origin(ctx, data, env, title_origin);

        let star_origin = (
            title_origin.x + title_label_size.width + 10.0,
            h / 2. - star_size.height / 2.,
        )
            .into();
        self.star.set_origin(ctx, data, env, star_origin);

        let page_cnt_origin = (
            w - page_cnt_label_size.width - 10.,
            h / 2. - page_cnt_label_size.height / 2.,
        )
            .into();
        self.page_cnt_label
            .set_origin(ctx, data, env, page_cnt_origin);

        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &B, env: &Env) {
        let color = if data.is_selected() {
            env.get(colors::PRIMARY_VARIANT)
        } else if self.is_hot {
            env.get(colors::PRIMARY_ACCENT)
        } else {
            env.get(colors::PRIMARY)
        };

        let rect = ctx.size().to_rect().to_rounded_rect(10.0);
        ctx.fill(rect, &color);

        self.title_label.paint(ctx, data, env);
        self.page_cnt_label.paint(ctx, data, env);
        self.star.paint(ctx, data, env);
    }
}

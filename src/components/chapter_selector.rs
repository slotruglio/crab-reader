use druid::{widget::Label, Color, LifeCycleCtx, RenderContext, Widget, WidgetExt, WidgetPod};

use crate::Library;

use super::{book::BookReading, library::GUILibrary};

pub struct ChapterSelector {
    children: Vec<ChapterSelectorItem>,
}

struct ChapterSelectorItem {
    idx: usize,
    inner: WidgetPod<Library, Box<dyn Widget<Library>>>,
    pod_size: druid::Size,
}

impl ChapterSelector {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    fn add_child(&mut self, idx: usize) {
        let pod = ChapterSelectorItem::new(idx);
        self.children.push(pod);
    }
}

impl ChapterSelectorItem {
    pub fn new(idx: usize) -> Self {
        let label = Label::dynamic(move |_: &Library, _env: &_| format!("Capitolo {}", idx + 1))
            .with_text_color(Color::rgb8(255, 255, 255));
        let clickable = label.on_click(move |ctx, data: &mut Library, _env| {
            data.get_selected_book_mut()
                .unwrap()
                .set_chapter_number(idx, false);
            ctx.request_layout();
        });
        let boxed = Box::new(clickable);

        Self {
            idx,
            inner: WidgetPod::new(boxed),
            pod_size: druid::Size::ZERO,
        }
    }
}

impl Widget<Library> for ChapterSelector {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Library,
        env: &druid::Env,
    ) {
        for child in self.children.iter_mut() {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Library,
        env: &druid::Env,
    ) {
        let book = data.get_selected_book().unwrap();
        while self.children.len() < book.get_number_of_chapters() {
            let idx = self.children.len();
            self.add_child(idx);
            ctx.children_changed();
        }

        for child in self.children.iter_mut() {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &Library,
        data: &Library,
        env: &druid::Env,
    ) {
        for child in self.children.iter_mut() {
            child.update(ctx, old_data, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Library,
        env: &druid::Env,
    ) -> druid::Size {
        let mut h = 0.0;
        for child in self.children.iter_mut() {
            let pos = druid::Point::new(0.0, h);
            let size = child.layout(ctx, bc, data, env);
            h += size.height;
            child.inner.set_origin(ctx, data, env, pos);
            child.pod_size = size;
        }

        let w = if bc.is_width_bounded() {
            bc.max().width
        } else {
            bc.min().width
        };

        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Library, env: &druid::Env) {
        for child in self.children.iter_mut() {
            child.paint(ctx, data, env);
        }
    }
}

impl Widget<Library> for ChapterSelectorItem {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Library,
        env: &druid::Env,
    ) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Library,
        env: &druid::Env,
    ) {
        match event {
            druid::LifeCycle::HotChanged(_) => {
                ctx.request_paint();
            }
            _ => {}
        }
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _: &Library,
        data: &Library,
        env: &druid::Env,
    ) {
        self.inner.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Library,
        env: &druid::Env,
    ) -> druid::Size {
        let size = self.inner.layout(ctx, bc, data, env);
        let w = if bc.is_width_bounded() {
            bc.max().width
        } else {
            size.width
        };
        (w, size.height).into()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Library, env: &druid::Env) {
        // todo: chiedre a Sam un metodo per avere il chapter attualmete in lettura
        // ( è possibile solo tramite library )
        let opacity = if self.idx == 0 {
            200
        } else if ctx.is_hot() {
            100
        } else {
            10
        };

        let rect = self.pod_size.to_rect();
        ctx.fill(rect, &Color::rgba8(0, 0, 0, opacity));
        self.inner.paint(ctx, data, env);
    }
}

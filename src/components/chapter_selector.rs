use druid::{
    widget::Label, Affine, Color, MouseEvent, RenderContext, Size, Widget, WidgetExt, WidgetPod,
};

use crate::Library;

use super::{book::BookReading, colors, library::GUILibrary};

pub struct ChapterSelector {
    children: Vec<ChapterSelectorItem>,
}

struct ChapterSelectorItem {
    idx: usize,
    inner: WidgetPod<Library, Box<dyn Widget<Library>>>,
    pod_size: Size,
    hot: bool,
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
            .with_text_color(colors::TEXT_WHITE);
        let boxed = Box::new(label);

        Self {
            idx,
            inner: WidgetPod::new(boxed),
            pod_size: Size::ZERO,
            hot: false,
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
        ctx.request_paint();
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
            let pod_h = child.layout(ctx, bc, data, env).height;
            let pod_w = if bc.is_width_bounded() {
                bc.max().width
            } else {
                400.0
            };
            h += pod_h;
            child.inner.set_origin(ctx, data, env, pos);
            child.pod_size = (pod_w, pod_h).into();
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
        match event {
            druid::Event::MouseMove(mouse) => {
                let pos = mouse.pos;
                let y = pos.y;
                let h0 = self.pod_size.height * self.idx as f64;
                let h1 = h0 + self.pod_size.height;
                self.hot = y >= h0 && y <= h1;
            }
            druid::Event::MouseDown(_) => {
                if self.hot {
                    println!("Current index: {}", self.idx);
                    data.get_selected_book_mut()
                        .unwrap()
                        .set_chapter_number(self.idx, true);
                    ctx.request_paint();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Library,
        env: &druid::Env,
    ) {
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
        let rect = self.pod_size.to_rect();
        let h = rect.height();
        let dh = self.idx as f64 * h;

        let color = if self.idx == data.get_selected_book().unwrap().get_chapter_number() {
            colors::ACTIVE_GRAY
        } else if self.hot {
            colors::HOT_GRAY
        } else {
            colors::BG_GRAY
        };

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate((0.0, dh)));
            ctx.fill(rect, &color);
        });
        self.inner.paint(ctx, data, env);
    }
}

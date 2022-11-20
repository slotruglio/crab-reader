use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Size, UpdateCtx, Widget, WidgetPod,
};

use super::{
    book::GUIBook,
    book_listing::BookListing,
    library::{GUILibrary, SELECTED_BOOK_SELECTOR},
};

pub struct ListLibrary<B: GUIBook> {
    children: Vec<WidgetPod<B, BookListing<B>>>,
}

impl<B: GUIBook> ListLibrary<B> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self) {
        let child = BookListing::new();
        let pod = WidgetPod::new(child);
        self.children.push(pod);
    }

    #[allow(dead_code)]
    pub fn remove_book(&mut self, idx: usize) {
        if let Some(_) = self.children.get(idx) {
            self.children.remove(idx);
        }
    }
}

impl<L, B> Widget<L> for ListLibrary<B>
where
    L: GUILibrary + GUILibrary<B = B>,
    B: GUIBook,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut L, env: &Env) {
        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book_mut(idx) {
                inner.event(ctx, event, book, env);
            }
        }

        match event {
            Event::MouseDown(_) => {
                if !ctx.is_handled() {
                    data.unselect_current_book();
                    ctx.request_layout();
                }
            }
            Event::Notification(cmd) => {
                if let Some(idx) = cmd.get(SELECTED_BOOK_SELECTOR) {
                    if let Some(idx) = idx {
                        data.set_selected_book_idx(*idx);
                    } else {
                        data.unselect_current_book();
                    }
                    ctx.request_layout();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &L, env: &Env) {
        while data.number_of_books() > self.children.len() {
            self.add_child();
            ctx.children_changed();
        }

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                inner.lifecycle(ctx, event, book, env);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &L, data: &L, env: &Env) {
        if data.get_sort_order() != old_data.get_sort_order() {
            self.children.clear();
            ctx.children_changed();
        }

        if data.only_fav() != old_data.only_fav() {
            self.children.clear();
            ctx.children_changed();
        }

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                inner.update(ctx, book, env);
            }
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &L, env: &Env) -> Size {
        let w = bc.max().width;
        let max_child_h = 100.0;
        let child_spacing = 10.0;
        let tight_bc = BoxConstraints::tight(Size::new(w, max_child_h));
        let mut h = 0.;

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                if book.is_filtered_out() {
                    let bc = BoxConstraints::tight((0.0, 0.0).into());
                    inner.layout(ctx, &bc, book, env);
                    inner.set_origin(ctx, book, env, (0.0, 0.0).into());
                    continue;
                }

                let csize = inner.layout(ctx, &tight_bc, book, env);
                let ch = csize.height;
                let origin = Point::new(0.0, h);
                inner.set_origin(ctx, book, env, origin);
                h += ch + child_spacing;
            }
        }

        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &L, env: &Env) {
        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                inner.paint(ctx, book, env);
            }
        }
    }
}

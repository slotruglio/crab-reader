use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Size, UpdateCtx, Widget, WidgetPod,
};

use super::{
    book::Book,
    book_listing::BookListing,
    library::{GUILibrary, SELECTED_BOOK_SELECTOR},
    mockup::MockupLibrary,
};
type Library = MockupLibrary<Book>;

pub struct ListLibrary {
    children: Vec<WidgetPod<Book, BookListing>>,
}

impl ListLibrary {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add_book(&mut self, _: &Book) {
        let child: BookListing = BookListing::new();
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

impl Widget<Library> for ListLibrary {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Library, env: &Env) {
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

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Library, env: &Env) {
        while data.number_of_books() > self.children.len() {
            let idx = self.children.len();
            if let Some(book) = data.get_book(idx) {
                self.add_book(book);
                ctx.children_changed();
            };
        }

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                inner.lifecycle(ctx, event, book, env);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &Library, data: &Library, env: &Env) {
        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                inner.update(ctx, book, env);
            }
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Library,
        env: &Env,
    ) -> Size {
        let w = bc.max().width;
        let mut h = 0.0;
        let child_h = 70.0;
        let child_spacing = 10.0;

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                let size = (w, child_h).into();
                let bc = BoxConstraints::tight(size);
                inner.layout(ctx, &bc, book, env);

                let y = child_spacing + (idx as f64 * (child_h + child_spacing));

                let origin = Point::new(0.0, y);
                inner.set_origin(ctx, book, env, origin);
                h += size.height + child_spacing;
            }
        }

        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Library, env: &Env) {
        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                inner.paint(ctx, book, env);
            }
        }
    }
}

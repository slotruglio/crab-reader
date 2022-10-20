use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Size, UpdateCtx, Widget, WidgetPod,
};

use crate::Library;

use super::{
    book::{Book, GUIBook},
    book_cover::{BookCover, BOOK_WIDGET_SIZE},
    library::{GUILibrary, SELECTED_BOOK_SELECTOR},
};

pub struct CoverLibrary {
    children: Vec<WidgetPod<Book, BookCover>>,
}

impl CoverLibrary {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn add_child(&mut self, book: &Book) {
        let widget = BookCover::new(book);
        let pod = WidgetPod::new(widget);
        self.children.push(pod);
    }
}

impl Widget<Library> for CoverLibrary {
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
                    ctx.request_paint();
                }
            }
            Event::Notification(cmd) => {
                if let Some(idx) = cmd.get(SELECTED_BOOK_SELECTOR) {
                    if let Some(idx) = idx {
                        data.set_selected_book_idx(*idx);
                    } else {
                        data.unselect_current_book();
                    }
                    ctx.request_paint();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Library, env: &Env) {
        while self.children.len() < data.number_of_books() {
            let idx = self.children.len();
            if let Some(book) = data.get_book(idx) {
                self.add_child(book);
                ctx.children_changed();
            };
        }

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                inner.lifecycle(ctx, event, book, env);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Library, data: &Library, env: &Env) {
        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(_) = old_data.get_book(idx) {
                if let Some(book) = data.get_book(idx) {
                    // TIL: let Some && let Some is unstable
                    (*inner).update(ctx, book, env);
                }
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
        let book_w = BOOK_WIDGET_SIZE.width;
        let book_h = BOOK_WIDGET_SIZE.height;
        let width = bc.max().width;
        let min_spacing = 20.0;

        // To avoid many casts, one is used for floating point ops and the othe for usize ops
        let mut fbooks_per_row = (width / book_w)
            .min(data.number_of_books() as f64)
            .max(1.0)
            .floor();
        let mut ubooks_per_row = fbooks_per_row as usize;

        let mut leftover_space = width - (fbooks_per_row * book_w);
        let mut spacing = leftover_space / (fbooks_per_row + 1.0);

        if spacing <= min_spacing {
            fbooks_per_row = (fbooks_per_row - 1.0).max(1.0);
            ubooks_per_row = fbooks_per_row as usize;
            leftover_space = bc.max().width - (fbooks_per_row * book_w);
            spacing = leftover_space / (fbooks_per_row + 1.0);
        }

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                let row = idx / ubooks_per_row;
                let col = idx % ubooks_per_row;

                let x = spacing + (col as f64 * (book_w + spacing));
                let y = spacing + (row as f64 * (book_h + spacing));

                let size = Size::new(book_w, book_h);
                let origin = Point::new(x, y);
                inner.layout(ctx, &BoxConstraints::tight(size), book, env);
                inner.set_origin(ctx, book, env, origin); // TLDR: must be a WidgetPod...
            }
        }

        let nrows = (data.number_of_books() / ubooks_per_row + 1) as f64;
        let w = fbooks_per_row * book_w + (fbooks_per_row + 1.0) * spacing;
        let h = nrows * (book_h + spacing) + spacing;
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

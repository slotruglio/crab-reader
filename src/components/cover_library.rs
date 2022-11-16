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

    pub fn add_child(&mut self) {
        let widget = BookCover::new(); // .with_cover_image(book.get_path());
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
        while self.children.len() < data.number_of_books() {
            self.add_child();
            ctx.children_changed();
        }

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                inner.lifecycle(ctx, event, book, env);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Library, data: &Library, env: &Env) {
        if old_data.get_sort_order() != data.get_sort_order() {
            self.children.clear();
            ctx.children_changed();
        }

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(old_book) = old_data.get_book(idx) {
                if let Some(book) = data.get_book(idx) {
                    // TIL: let Some && let Some is unstable
                    if old_book != book {
                        (*inner).update(ctx, book, env);
                    }
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
        dbg!(data.get_number_of_visible_books(), width);
        let min_spacing = 30.0;
        let mut cnt = 0;

        let books_per_row = ((width - min_spacing) / (book_w + min_spacing)).floor() as usize;
        let rows =
            (data.get_number_of_visible_books() as f64 / books_per_row as f64).ceil() as usize;
        dbg!((width - (books_per_row as f64 * book_w)) / (books_per_row as f64 + 1.0));
        let spacing = (width - (books_per_row as f64 * book_w)) / (books_per_row as f64 + 1.0);
        let xspacing = ((width - (data.get_number_of_visible_books() as f64 * book_w))
            / (data.get_number_of_visible_books() as f64 + 1.0))
            .max(spacing);

        dbg!(spacing, xspacing);

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx) {
                if book.is_filtered_out() {
                    inner.layout(ctx, bc, book, env);
                    continue;
                }

                let row = cnt / books_per_row;
                let col = cnt % books_per_row;

                let x = xspacing + (col as f64 * (book_w + xspacing));
                let y = spacing + (row as f64 * (book_h + spacing));

                let size = Size::new(book_w, book_h);
                let origin = Point::new(x, y);
                inner.layout(ctx, &BoxConstraints::tight(size), book, env);
                inner.set_origin(ctx, book, env, origin); // TLDR: must be a WidgetPod...
                cnt += 1;
            }
        }

        let w = books_per_row as f64 * book_w + (books_per_row as f64 + 1.0) * spacing;
        let h = rows as f64 * (book_h + spacing) + spacing;
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

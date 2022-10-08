use druid::{
    im::Vector, BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, Size, UpdateCtx, Widget, WidgetPod,
};

use crate::components::book::BOOK_WIDGET_SIZE;

use super::book::Book;

pub struct Library {
    books: Vec<WidgetPod<Book, Box<dyn Widget<Book>>>>,
    #[allow(dead_code)]
    book_per_row: u16,
    #[allow(dead_code)]
    rows: u16,
}

impl Library {
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
            // Avoid divison by 0 errors
            book_per_row: 1,
            rows: 1,
        }
    }

    pub fn add_book(&mut self, book: &Book) {
        let w = Box::new(book.clone()) as Box<dyn Widget<Book>>;
        let pod = WidgetPod::new(w);
        self.books.push(pod);
    }

    #[allow(dead_code)]
    pub fn remove_book(&mut self, idx: u16) {
        let idx = idx as usize;
        if let Some(_) = self.books.get(idx) {
            self.books.remove(idx);
        }
    }
}

impl Widget<Vector<Book>> for Library {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Vector<Book>, env: &Env) {
        for (idx, inner) in self.books.iter_mut().enumerate() {
            let book = data.get_mut(idx).unwrap(); // Make safe just for fun??
            inner.event(ctx, event, book, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &Vector<Book>,
        env: &Env,
    ) {
        while data.len() > self.books.len() {
            let idx = self.books.len();
            let book = data.get(idx).unwrap();
            self.add_book(book);
        }

        for (idx, inner) in self.books.iter_mut().enumerate() {
            inner.lifecycle(ctx, event, &data[idx], env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &Vector<Book>,
        data: &Vector<Book>,
        env: &Env,
    ) {
        for (idx, inner) in self.books.iter_mut().enumerate() {
            inner.update(ctx, &data[idx], env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Vector<Book>,
        env: &Env,
    ) -> Size {
        let book_w = BOOK_WIDGET_SIZE.width;
        let book_h = BOOK_WIDGET_SIZE.height;
        let width = bc.max().width;
        let min_spacing = 20.0;

        let mut books_per_row = ((width / book_w).floor() as u16).min(data.len() as u16);
        let mut leftover_space = width - (books_per_row as f64 * book_w);
        let mut spacing = leftover_space / (books_per_row as f64 + 1.0);

        if spacing <= min_spacing {
            books_per_row -= 1;
            leftover_space = bc.max().width - (books_per_row as f64 * book_w);
            spacing = leftover_space / (books_per_row as f64 + 1.0);
        }

        let rows = (data.len() as f64 / books_per_row as f64).ceil();

        for (idx, inner) in self.books.iter_mut().enumerate() {
            let data = &data[idx];
            let row = (idx as f64 / books_per_row as f64).floor() as u16;
            let col = (idx as f64 % books_per_row as f64).floor() as u16;

            let x = spacing + (col as f64 * (book_w + spacing));
            let y = spacing + (row as f64 * (book_h + spacing));

            let size = Size::new(book_w, book_h);
            let origin = Point::new(x, y);
            let bc = BoxConstraints::tight(size);
            inner.layout(ctx, &bc, data, env);
            inner.set_origin(ctx, data, env, origin);
        }

        let w = books_per_row as f64 * book_w + (books_per_row as f64 + 1.0) * spacing;
        let h = rows * book_h + (rows + 1.0) * spacing;
        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Vector<Book>, env: &Env) {
        for (idx, inner) in self.books.iter_mut().enumerate() {
            inner.paint(ctx, &data[idx], env);
        }
    }
}

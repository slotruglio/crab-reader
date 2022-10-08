use druid::{
    im::Vector, BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Size, UpdateCtx, Widget, WidgetPod,
};

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
        dbg!(bc);
        for (idx, inner) in self.books.iter_mut().enumerate() {
            inner.layout(ctx, bc, &data[idx], env);
        }

        if bc.is_height_bounded() && bc.is_width_bounded() {
            bc.max()
        } else {
            Size::new(400.0, 400.0) // Placeholder
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Vector<Book>, env: &Env) {
        for (idx, inner) in self.books.iter_mut().enumerate() {
            inner.paint(ctx, &data[idx], env);
        }
    }
}

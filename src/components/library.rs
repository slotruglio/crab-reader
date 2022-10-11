use druid::{
    im::Vector, BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, Size, UpdateCtx, Widget, WidgetPod,
};

use crate::components::book::BOOK_WIDGET_SIZE;

use super::book::{Book, BookCoverItem, ListBookItem};

pub struct Library {
    books: Vec<WidgetPod<Book, Box<dyn Widget<Book>>>>,
}

impl Library {
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: &Book) {
        let w = Box::<BookCoverItem>::new(book.clone().into()) as Box<dyn Widget<Book>>;
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

        // To avoid many casts, one is used for floating point ops and the othe for usize ops
        let mut fbooks_per_row = (width / book_w).min(data.len() as f64).max(1.0).floor();
        let mut ubooks_per_row = fbooks_per_row as usize;

        let mut leftover_space = width - (fbooks_per_row * book_w);
        let mut spacing = leftover_space / (fbooks_per_row + 1.0);

        if spacing <= min_spacing {
            fbooks_per_row = (fbooks_per_row - 1.0).max(1.0);
            ubooks_per_row = fbooks_per_row as usize;
            leftover_space = bc.max().width - (fbooks_per_row * book_w);
            spacing = leftover_space / (fbooks_per_row + 1.0);
        }

        for (idx, inner) in self.books.iter_mut().enumerate() {
            let data = &data[idx];
            let row = (idx / ubooks_per_row) as f64;
            let col = (idx % ubooks_per_row) as f64;

            let x = spacing + (col * (book_w + spacing));
            let y = spacing + (row * (book_h + spacing));

            let size = (book_w, book_h).into();
            let origin = Point::new(x, y);
            let bc = BoxConstraints::tight(size);
            inner.layout(ctx, &bc, data, env);
            inner.set_origin(ctx, data, env, origin);
        }

        let nrows = (data.len() / ubooks_per_row + 1) as f64;
        let w = fbooks_per_row * book_w + (fbooks_per_row + 1.0) * spacing;
        let h = nrows * (book_h + spacing) + spacing;
        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Vector<Book>, env: &Env) {
        for (idx, inner) in self.books.iter_mut().enumerate() {
            inner.paint(ctx, &data[idx], env);
        }
    }
}

pub struct LibraryList {
    books: Vec<WidgetPod<Book, Box<dyn Widget<Book>>>>,
}

impl LibraryList {
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: &Book) {
        let list_book_item: ListBookItem = book.clone().into();
        let wbox = Box::new(list_book_item) as Box<dyn Widget<Book>>;
        let w = Box::new(wbox) as Box<dyn Widget<Book>>;
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

impl Widget<Vector<Book>> for LibraryList {
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

    fn update(&mut self, ctx: &mut UpdateCtx, _: &Vector<Book>, data: &Vector<Book>, env: &Env) {
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
        let w = bc.max().width;
        let mut h = 0.0;
        let child_h = 70.0;
        let child_spacing = 10.0;

        for (idx, inner) in self.books.iter_mut().enumerate() {
            let data = &data[idx];
            let size = (w, child_h).into();
            let bc = BoxConstraints::tight(size);
            inner.layout(ctx, &bc, data, env);

            let y = child_spacing + (idx as f64 * (child_h + child_spacing));

            let origin = Point::new(0.0, y);
            inner.set_origin(ctx, data, env, origin);
            h += size.height + child_spacing;
        }

        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Vector<Book>, env: &Env) {
        for (idx, inner) in self.books.iter_mut().enumerate() {
            inner.paint(ctx, &data[idx], env);
        }
    }
}

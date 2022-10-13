use druid::{
    im::Vector, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, Selector, Size, UpdateCtx, Widget, WidgetPod,
};

use crate::components::book::BOOK_WIDGET_SIZE;

use super::book::{Book, BookListing, CoverBook};

pub const SELECTED_BOOK_SELECTOR: Selector<Option<u16>> = Selector::new("selected-book");

#[derive(Clone, Lens, PartialEq, Data)]
pub struct Library {
    books: Vector<Book>,
    selected_book: Option<u16>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            books: Vector::new(),
            selected_book: None,
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push_back(book.with_idx(self.books.len() as u16));
    }

    pub fn remove_book(&mut self, idx: u16) {
        let idx = idx as usize;
        if let Some(_) = self.books.get(idx) {
            self.books.remove(idx);
        }
    }

    pub fn get_book_mut(&mut self, idx: u16) -> Option<&mut Book> {
        let idx = idx as usize;
        self.books.get_mut(idx)
    }

    pub fn get_book(&self, idx: u16) -> Option<&Book> {
        let idx = idx as usize;
        self.books.get(idx)
    }

    pub fn library_len(&self) -> usize {
        self.books.len()
    }

    fn set_selcted_book(&mut self, selected: &Option<u16>) {
        if let Some(idx) = self.selected_book {
            if let Some(old_selected) = self.get_book_mut(idx) {
                old_selected.unselect();
            }
        }
        self.selected_book = selected.clone();
    }
}

pub struct CoverLibrary {
    children: Vec<WidgetPod<Book, CoverBook>>,
}

impl CoverLibrary {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn add_child(&mut self, book: &Book) {
        let widget = CoverBook::new().with_cover_image_path(book.get_cover_path());
        let pod = WidgetPod::new(widget);
        self.children.push(pod);
    }
}

impl Widget<Library> for CoverLibrary {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Library, env: &Env) {
        for (idx, inner) in self.children.iter_mut().enumerate() {
            let idx = idx as u16;
            if let Some(book) = data.get_book_mut(idx) {
                inner.event(ctx, event, book, env);
            }
        }

        match event {
            Event::MouseDown(_) => {
                if !ctx.is_handled() {
                    data.set_selcted_book(&None);
                    ctx.request_paint();
                }
            }
            Event::Notification(cmd) => {
                if let Some(selected) = cmd.get(SELECTED_BOOK_SELECTOR) {
                    data.set_selcted_book(selected);
                    ctx.request_paint();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Library, env: &Env) {
        while self.children.len() < data.library_len() {
            let idx = self.children.len() as u16;
            if let Some(book) = data.get_book(idx) {
                self.add_child(book);
                ctx.children_changed();
            };
        }

        for (idx, inner) in self.children.iter_mut().enumerate() {
            if let Some(book) = data.get_book(idx as u16) {
                inner.lifecycle(ctx, event, book, env);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Library, data: &Library, env: &Env) {
        for (idx, inner) in self.children.iter_mut().enumerate() {
            let idx = idx as u16;
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
            .min(data.library_len() as f64)
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
            let idx = idx as u16;
            if let Some(book) = data.get_book(idx) {
                let row = idx / ubooks_per_row as u16;
                let col = idx % ubooks_per_row as u16;

                let x = spacing + (col as f64 * (book_w + spacing));
                let y = spacing + (row as f64 * (book_h + spacing));

                let size = Size::new(book_w, book_h);
                let origin = Point::new(x, y);
                inner.layout(ctx, &BoxConstraints::tight(size), book, env);
                inner.set_origin(ctx, book, env, origin); // TLDR: must be a WidgetPod...
            }
        }

        let nrows = (data.library_len() / ubooks_per_row + 1) as f64;
        let w = fbooks_per_row * book_w + (fbooks_per_row + 1.0) * spacing;
        let h = nrows * (book_h + spacing) + spacing;
        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Library, env: &Env) {
        for (idx, inner) in self.children.iter_mut().enumerate() {
            let idx = idx as u16;
            if let Some(book) = data.get_book(idx) {
                inner.paint(ctx, book, env);
            }
        }
    }
}

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
    pub fn remove_book(&mut self, idx: u16) {
        let idx = idx as usize;
        if let Some(_) = self.children.get(idx) {
            self.children.remove(idx);
        }
    }
}

impl Widget<Library> for ListLibrary {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Library, env: &Env) {
        for (idx, inner) in self.children.iter_mut().enumerate() {
            let idx = idx as u16;
            let book = data.get_book_mut(idx).unwrap(); // Make safe just for fun??
            inner.event(ctx, event, book, env);
        }

        match event {
            Event::MouseDown(_) => {
                if !ctx.is_handled() {
                    data.set_selcted_book(&None);
                }
            }
            Event::Notification(cmd) => {
                if let Some(selected) = cmd.get(SELECTED_BOOK_SELECTOR) {
                    data.set_selcted_book(selected);
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Library, env: &Env) {
        while data.library_len() > self.children.len() {
            let idx = self.children.len() as u16;
            if let Some(book) = data.get_book(idx) {
                self.add_book(book);
                ctx.children_changed();
            };
        }

        for (idx, inner) in self.children.iter_mut().enumerate() {
            let idx = idx as u16;
            if let Some(book) = data.get_book(idx) {
                inner.lifecycle(ctx, event, book, env);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &Library, data: &Library, env: &Env) {
        for (idx, inner) in self.children.iter_mut().enumerate() {
            let idx = idx as u16;
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
            let idx = idx as u16;
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
            if let Some(book) = data.get_book(idx as u16) {
                inner.paint(ctx, book, env);
            }
        }
    }
}

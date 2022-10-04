use druid::im::Vector;
use druid::widget::Flex;
use druid::WidgetExt;
use druid::{Data, Env, Event, Lens, LensExt, Widget, WidgetPod};

use super::book::Book;

#[derive(Clone, Data, Lens, PartialEq)]
pub struct Library {
    //todo: change this
    pub nbooks: u16,
    selected: Option<u16>,
    books: Vector<Book>,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            nbooks: 0,
            selected: None,
            books: Vector::new(),
        }
    }
}

impl From<Library> for LibraryWidget {
    fn from(val: Library) -> Self {
        let nbooks = val.nbooks as usize;
        let mut row = Flex::column();
        dbg!(val.books.len());
        for idx in 0..nbooks {
            let book = val.books.get(idx as usize);
            if let Some(book) = book {
                let book = book.clone();
                let widget = book.widget().lens(Library::books.index(idx));
                row.add_flex_child(widget, 1.0);
            }
        }
        LibraryWidget {
            inner: WidgetPod::new(row),
            state: val,
        }
    }
}

impl Library {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_book(&mut self, title: String, npages: u16) {
        let book = Book::new().with_title(title).with_npages(npages);
        self.books.push_front(book);
        self.nbooks += 1;
    }

    pub fn remove_book(&mut self, idx: u16) {
        let len = self.books.len();
        let idx = idx as usize;

        if len == 0 {
            dbg!("Tried to remove books on an empty library!");
            return;
        }

        if idx >= len {
            dbg!("Tried to remove a book that doesn't exist!");
            return;
        }

        self.books.remove(idx);
    }

    pub fn set_selected(&mut self, idx: u16) {
        let len = self.books.len();
        let idx = idx as usize;

        if len == 0 {
            dbg!("Tried to select a book on an empty library!");
            return;
        }

        if idx >= len {
            dbg!("Tried to select a book that doesn't exist!");
            return;
        }

        self.selected = Some(idx as u16);
    }

    pub fn unselect_current(&mut self) {
        self.selected = None;
    }
}

#[derive(Lens)]
pub struct LibraryWidget {
    inner: WidgetPod<Library, Flex<Library>>,
    state: Library,
}

impl Widget<Library> for LibraryWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &Event, data: &mut Library, env: &Env) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Library,
        env: &Env,
    ) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &Library,
        data: &Library,
        env: &Env,
    ) {
        self.inner.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Library,
        env: &Env,
    ) -> druid::Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Library, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}

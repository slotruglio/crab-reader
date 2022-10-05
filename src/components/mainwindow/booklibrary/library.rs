use druid::im::Vector;
use druid::widget::Flex;
use druid::widget::MainAxisAlignment::SpaceEvenly;
use druid::{
    BoxConstraints, Color, Command, Data, Env, Event, EventCtx, LayoutCtx, Lens, LensExt,
    LifeCycle, LifeCycleCtx, PaintCtx, Size, Target, UpdateCtx, Widget, WidgetPod,
};
use druid::{Selector, WidgetExt};

use super::book::{self, Book};

pub const SELECTED_BOOK: Selector<u16> = Selector::<u16>::new("library.selectedbook.idx");
pub const UPDATE_NCHILDREN: Selector<()> = Selector::<()>::new("library.update.nchildren");

#[derive(Clone, Data, Lens, PartialEq)]
pub struct Library {
    //todo: change this
    pub nbooks: u16,
    selected: Option<u16>,
    books: Vector<Book>,
    children_per_row: usize,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            nbooks: 0,
            selected: None,
            books: Vector::new(),
            children_per_row: 5, // todo: placeholder value
        }
    }
}

impl From<&Library> for WidgetPod<Library, Flex<Library>> {
    fn from(lib: &Library) -> Self {
        let mut flex = Flex::column();
        let children_per_row = lib.children_per_row;
        let nrows = lib.nbooks as usize / children_per_row + 1;

        for row_idx in 0..nrows {
            let mut row = Flex::row();
            for col_idx in 0..children_per_row {
                let book_idx = row_idx * children_per_row + col_idx;
                if book_idx >= lib.nbooks as usize {
                    break;
                }

                if let Some(book) = lib.books.get(book_idx) {
                    row.add_child(
                        book.clone()
                            .widget()
                            .lens(Library::books.index(book_idx))
                            .padding(7.5),
                    );
                }
            }
            row.set_main_axis_alignment(SpaceEvenly);
            row.set_must_fill_main_axis(true);
            flex.add_child(row);
        }

        let child = flex
            .background(Color::GRAY)
            .rounded(7.5)
            .padding(10.0)
            .expand();

        WidgetPod::new(Flex::row().with_flex_child(child, 1.0))
    }
}

impl From<Library> for LibraryWidget {
    fn from(val: Library) -> Self {
        LibraryWidget {
            inner: (&val).into(),
            state: val,
            children_per_row: 0,
        }
    }
}

impl Library {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_book(&mut self, title: String, npages: u16) {
        let book = Book::new()
            .with_title(title)
            .with_npages(npages)
            .with_idx(self.nbooks);
        self.books.push_back(book);
        self.nbooks += 1;
    }

    pub fn remove_book(&mut self, idx: u16) {
        let len = self.books.len();
        let idx = idx as usize;

        if len == 0 || idx >= len {
            return;
        }

        self.books.remove(idx);
    }

    pub fn set_selected(&mut self, idx: u16) {
        let len = self.books.len();
        let idx = idx as usize;

        if len == 0 || idx >= len {
            return;
        }

        if let Some(old_selected) = self.selected {
            if let Some(old_selected_book) = self.books.get_mut(old_selected as usize) {
                old_selected_book.unselect();
            }
        }

        self.selected = Some(idx as u16);
    }

    pub fn unselect_current(&mut self) {
        if let Some(old_selected) = self.selected {
            if let Some(old_selected_book) = self.books.get_mut(old_selected as usize) {
                old_selected_book.unselect();
            }
        }
        self.selected = None;
    }

    pub fn selected_book_title(&self) -> String {
        if let Some(selected_idx) = self.selected {
            if let Some(selected_book) = self.books.get(selected_idx as usize) {
                return selected_book.get_title();
            }
        }
        "No Book Selected".into()
    }

    fn set_children_per_row(&mut self, num: usize) {
        self.children_per_row = num;
    }
}

#[derive(Lens)]
pub struct LibraryWidget {
    inner: WidgetPod<Library, Flex<Library>>,
    state: Library,
    children_per_row: usize,
}

impl LibraryWidget {
    fn compute_children_per_row(&mut self) -> usize {
        let size = self.inner.layout_rect().size().width;
        let child_size = book::BOOK_WIDGET_WIDTH;
        let children_per_row = (size / child_size) as usize;
        children_per_row
    }

    //todo: make this a method of `Library`
    pub fn rebuild_inner(&mut self) {
        // This panics...
        // self.inner = self.state.clone().into();
    }
}

impl Widget<Library> for LibraryWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Library, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_handled() {
                    data.unselect_current();
                }
            }
            Event::Notification(cmd) => {
                if cmd.is(SELECTED_BOOK) {
                    if let Some(idx) = cmd.get(SELECTED_BOOK) {
                        let idx = *idx;
                        data.set_selected(idx);
                    }
                }
            }
            Event::Command(cmd) => {
                if cmd.is(UPDATE_NCHILDREN) {
                    let children_per_row = self.compute_children_per_row();
                    data.set_children_per_row(children_per_row);
                }
            }
            _ => (),
        }
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Library, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Library, data: &Library, env: &Env) {
        if data != old_data {
            self.inner.update(ctx, data, env);
            if data.children_per_row != old_data.children_per_row {
                self.rebuild_inner();
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
        let cmd = Command::new(UPDATE_NCHILDREN.into(), (), Target::Auto);
        ctx.submit_command(cmd);

        let size = self.inner.layout(ctx, bc, data, env);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Library, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}

use components::book::{Book, BookReading, GUIBook};
use components::book_details::BookDetails;
use components::cover_library::CoverLibrary;
use components::library::GUILibrary;
use components::listing_library::ListLibrary;
use components::mockup::{LibraryFilterLens, MockupLibrary, SortBy};
use components::rbtn::RoundedButton;
use components::reader_view::{
    build_dual_view, build_dual_view_edit, build_single_view, build_single_view_edit,
};

use druid::widget::{Button, Either, Flex, Label, Scroll, ViewSwitcher};
use druid::{
    AppDelegate, AppLauncher, Color, Data, Env, EventCtx, Handled, Lens, PlatformError, Selector,
    Widget, WidgetExt, WindowDesc,
};
use once_cell::sync::Lazy;
use std::rc::Rc;
use std::sync::Mutex;
use utils::button_functions::{self, undo_button}; // 1.3.1
use utils::envmanager::MyEnv;

mod components;
mod utils;
type Library = MockupLibrary<Book>;

pub const ENTERING_READING_MODE: Selector<()> = Selector::new("reading-mode.on");
pub const LEAVING_READING_MODE: Selector<()> = Selector::new("reading-mode.off");

//Create a global ENV variable
#[allow(dead_code)]
static MYENV: Lazy<Mutex<MyEnv>> = Lazy::new(|| Mutex::new(MyEnv::new()));

#[derive(Clone, Data, Lens)]
pub struct ReadingState {
    single_view: Option<bool>,
    is_editing: Option<bool>,
    text_0: String,
    text_1: String,
}

impl ReadingState {
    fn enable<S: Into<Option<Rc<String>>>>(&mut self, text: S) {
        self.single_view = Some(true);
        self.is_editing = Some(false);
    }
    fn disable(&mut self) {
        self.single_view = None;
        self.is_editing = None;
        self.text_0 = String::default();
        self.text_1 = String::default();
    }
}

impl Default for ReadingState {
    fn default() -> Self {
        Self {
            single_view: None,
            is_editing: None,
            text_0: String::default(),
            text_1: String::default(),
        }
    }
}

#[derive(Clone, PartialEq, Data)]
pub enum DisplayMode {
    Cover,
    List,
}

#[derive(Clone, Data, Lens)]
struct CrabReaderState {
    user: UserState,
    library: Library,
    display_mode: DisplayMode,
    reading: bool,
    reading_state: ReadingState,
}

impl Default for CrabReaderState {
    fn default() -> Self {
        Self {
            user: UserState::new(),
            library: Library::new(),
            display_mode: DisplayMode::Cover,
            reading: false,
            reading_state: ReadingState::default(),
        }
    }
}

#[derive(Clone, Data)]
struct UserState {
    username: Rc<String>,
}

impl UserState {
    pub fn new() -> Self {
        Self {
            username: Rc::from("Username".to_string()),
        }
    }
}

fn book_details_panel() -> impl Widget<CrabReaderState> {
    BookDetails::new()
        .background(Color::GRAY)
        .rounded(10.0)
        .expand_width()
        .lens(CrabReaderState::library)
}

fn title_sorter_btn() -> impl Widget<Library> {
    RoundedButton::dynamic(|data: &Library, _env: &Env| {
        let arrow = match data.get_sort_order() {
            SortBy::Title => "v",
            _ => "^",
        };
        format!("Title {}", arrow)
    })
    .with_color(Color::rgb8(70, 70, 70))
    .with_hot_color(Color::rgb8(50, 50, 50))
    .with_active_color(Color::rgb8(0, 0, 0))
    .with_text_size(18.0)
    .with_on_click(|ctx, data: &mut Library, _: &Env| {
        let sort = data.get_sort_order();
        if sort == SortBy::Title {
            data.sort_by(SortBy::TitleRev);
        } else {
            data.sort_by(SortBy::Title);
        }
        ctx.request_update();
    })
    .padding(5.0)
}

fn author_sorter_btn() -> impl Widget<Library> {
    RoundedButton::dynamic(|data: &Library, _env: &Env| {
        let arrow = match data.get_sort_order() {
            SortBy::Author => "v",
            _ => "^",
        };
        format!("Author {}", arrow)
    })
    .with_color(Color::rgb8(70, 70, 70))
    .with_hot_color(Color::rgb8(50, 50, 50))
    .with_active_color(Color::rgb8(0, 0, 0))
    .with_text_size(18.0)
    .with_on_click(|ctx, data: &mut Library, _| {
        let sort = data.get_sort_order();
        if sort == SortBy::Author {
            data.sort_by(SortBy::AuthorRev);
        } else {
            data.sort_by(SortBy::Author);
        }
        ctx.request_update();
    })
    .padding(5.0)
}

fn completion_sorter_btn() -> impl Widget<Library> {
    RoundedButton::dynamic(|data: &Library, _env: &Env| {
        let arrow = match data.get_sort_order() {
            SortBy::PercRead => "v",
            _ => "^",
        };
        format!("Completion {}", arrow)
    })
    .with_color(Color::rgb8(70, 70, 70))
    .with_hot_color(Color::rgb8(50, 50, 50))
    .with_active_color(Color::rgb8(0, 0, 0))
    .with_text_size(18.0)
    .with_on_click(|ctx, data: &mut Library, _| {
        let sort = data.get_sort_order();
        if sort == SortBy::PercRead {
            data.sort_by(SortBy::PercReadRev);
        } else {
            data.sort_by(SortBy::PercRead);
        }
        ctx.request_update();
    })
    .padding(5.0)
}

// Showcase per Sam su come si usa
pub fn disabled_btn() -> impl Widget<Library> {
    RoundedButton::from_text("I am a disabled button")
        .with_text_size(18.0)
        .with_color(Color::rgb8(200, 20, 20))
        .with_hot_color(Color::rgb8(170, 20, 20))
        .with_on_click(|_, _, _| println!("You won't see this"))
        .disabled()
}

fn picker_sort_by() -> impl Widget<Library> {
    Flex::row()
        .with_child(Label::new("Sort by"))
        .with_child(completion_sorter_btn())
        .with_child(author_sorter_btn())
        .with_child(title_sorter_btn())
        .with_child(disabled_btn())
        .padding(5.0)
        .background(Color::GRAY)
        .rounded(5.0)
        .padding(druid::Insets::uniform_xy(10.0, 5.0))
        .expand_width()
}

fn picker_filter_by() -> impl Widget<Library> {
    let text_edit = druid::widget::TextBox::new()
        .with_placeholder("Filter by")
        .lens(LibraryFilterLens)
        .fix_width(500.0);
    Flex::row()
        .with_child(Label::new("Filter by"))
        .with_child(text_edit)
        .padding(5.0)
        .background(Color::GRAY)
        .rounded(5.0)
        .padding(druid::Insets::uniform_xy(10.0, 5.0))
        .expand_width()
        .fix_height(50.0)
}

fn picker_controller() -> impl Widget<Library> {
    let sort_by = picker_sort_by();
    let filter_by = picker_filter_by();
    Flex::column().with_child(sort_by).with_child(filter_by)
}

fn build_ui() -> impl Widget<CrabReaderState> {
    let library_cover = CoverLibrary::new().lens(CrabReaderState::library);
    let library_list = ListLibrary::new().lens(CrabReaderState::library);

    let view_either = Either::new(
        |data: &CrabReaderState, _env| data.display_mode == DisplayMode::List,
        library_list.padding(5.0),
        library_cover,
    )
    .background(Color::GRAY)
    .rounded(10.0)
    .padding(10.0);

    let ctls = picker_controller();
    let left_panel = Flex::column()
        .with_child(ctls.lens(CrabReaderState::library))
        .with_child(view_either)
        .padding(15.0);
    let scroll = Scroll::new(left_panel).vertical();
    let right_panel = Scroll::new(book_details_panel()).vertical();
    let right_col = Flex::column()
        .with_child(
            RoundedButton::dynamic(
                |data: &CrabReaderState, _env: &Env| match data.display_mode {
                    DisplayMode::List => "Passa a visualizzazione a lista".into(),
                    DisplayMode::Cover => "Passa a visualiazione a copertine".into(),
                },
            )
            .with_color(Color::rgb8(70, 70, 70))
            .with_hot_color(Color::rgb8(50, 50, 50))
            .with_active_color(Color::rgb8(20, 20, 20))
            .with_text_size(24.0)
            .with_text_color(Color::WHITE)
            .with_on_click(|ctx, data: &mut CrabReaderState, _| {
                data.display_mode = match data.display_mode {
                    DisplayMode::List => DisplayMode::Cover,
                    DisplayMode::Cover => DisplayMode::List,
                };
                ctx.request_update();
            })
            .padding((0.0, 20.0)),
        )
        .with_flex_child(right_panel, 1.0)
        .padding(10.0);

    let inner = Flex::row()
        .with_flex_child(scroll, 2.0)
        .with_flex_child(right_col, 1.0);

    Flex::column().with_flex_child(inner, 1.0)
}

#[derive(Clone, PartialEq, Data)]
enum VS {
    Reading,
    Browsing,
}

fn vs_child_picker(state: &CrabReaderState, _: &Env) -> VS {
    if state.reading {
        VS::Reading
    } else {
        VS::Browsing
    }
}

fn vs_child_builder(mode: &VS, _: &CrabReaderState, _: &Env) -> Box<dyn Widget<CrabReaderState>> {
    match mode {
        VS::Reading => Box::new(read_book_ui()),
        VS::Browsing => Box::new(build_ui()),
    }
}

fn get_viewswitcher() -> impl Widget<CrabReaderState> {
    ViewSwitcher::new(vs_child_picker, vs_child_builder)
}

fn read_book_ui() -> impl Widget<CrabReaderState> {
    let title = Label::dynamic(|data: &CrabReaderState, _env: &_| {
        data.library
            .get_selected_book()
            .map_or("Titolo libro non trovato".into(), |book| book.get_title())
    })
    .with_text_size(24.0);

    let current_chapter = Label::dynamic(|data: &CrabReaderState, _env: &_| {
        format!(
            "Chapter {}",
            data.library
                .get_selected_book()
                .unwrap()
                .get_chapter_number()
                .to_string()
        )
    })
    .with_text_size(16.0)
    .center();

    let text = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.single_view.unwrap(),
        Either::new(
            |data: &CrabReaderState, _env| data.reading_state.is_editing.unwrap(),
            build_single_view_edit(),
            build_single_view(),
        ),
        Either::new(
            |data: &CrabReaderState, _env| data.reading_state.is_editing.unwrap(),
            build_dual_view_edit(),
            build_dual_view(),
        ),
    )
    .fix_size(800.0, 450.0);

    let leave_btn = RoundedButton::from_text("Torna a selezione libro")
        .with_on_click(|_, data: &mut CrabReaderState, _| {
            data.reading = false;
        })
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(24.0);
    let leave_btn = Flex::row().with_child(leave_btn).align_left();

    // todo() switch to change single view and double view
    // this is a mock to test layout
    let views_btn = RoundedButton::dynamic(|data: &CrabReaderState, _env: &_| {
        if data.reading_state.single_view.unwrap() {
            "Attiva doppia pagina".into()
        } else {
            "Attiva singola pagina".into()
        }
    })
    .with_on_click(|_, data: &mut CrabReaderState, _| {
        data.reading_state.single_view = Some(!data.reading_state.single_view.unwrap());
    })
    .with_color(Color::rgb8(70, 70, 70))
    .with_hot_color(Color::rgb8(50, 50, 50))
    .with_active_color(Color::rgb8(20, 20, 20))
    .with_text_color(Color::WHITE)
    .with_text_size(24.0);

    let next_btn = RoundedButton::from_text("Prossima pagina")
        .with_on_click(|ctx, data: &mut CrabReaderState, _| {
            let book = data.library.get_selected_book_mut().unwrap();
            button_functions::change_page(
                ctx,
                book,
                data.reading_state.is_editing.unwrap(),
                data.reading_state.single_view.unwrap(),
                true,
            );
        })
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(18.0);

    let back_btn = RoundedButton::from_text("Pagina precedente")
        .with_on_click(|ctx, data: &mut CrabReaderState, _| {
            let book = data.library.get_selected_book_mut().unwrap();
            button_functions::change_page(
                ctx,
                book,
                data.reading_state.is_editing.unwrap(),
                data.reading_state.single_view.unwrap(),
                false,
            );
        })
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(18.0);

    let edit_btn = RoundedButton::dynamic(|data: &CrabReaderState, _env: &_| {
        if data.reading_state.is_editing.unwrap() {
            "Termina modifica".into()
        } else {
            "Modifica testo".into()
        }
    })
    .with_color(Color::rgb8(70, 70, 70))
    .with_hot_color(Color::rgb8(50, 50, 50))
    .with_active_color(Color::rgb8(20, 20, 20))
    .with_text_color(Color::WHITE)
    .with_text_size(24.0)
    .with_on_click(|_, data: &mut CrabReaderState, _| {
        if data.reading_state.is_editing.unwrap() {
            data.reading_state.is_editing = Some(false);
            button_functions::undo_button(&mut data.reading_state)
        } else {
            data.reading_state.single_view = Some(true);
            button_functions::edit_button(
                &mut data.reading_state,
                data.library.get_selected_book().unwrap(),
            );
        }
    })
    .align_right();

    let save_changes_btn = RoundedButton::from_text("Salva modifiche")
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(18.0)
        .with_on_click(|ctx, data: &mut CrabReaderState, _| {
            button_functions::save_button(
                ctx,
                &mut data.reading_state,
                &mut data.library.get_selected_book_mut().unwrap(),
            );
        });

    let undo_changes_btn = RoundedButton::from_text("Annulla modifiche")
        .with_color(Color::rgb8(70, 70, 70))
        .with_hot_color(Color::rgb8(50, 50, 50))
        .with_active_color(Color::rgb8(20, 20, 20))
        .with_text_color(Color::WHITE)
        .with_text_size(18.0)
        .with_on_click(|_, data: &mut CrabReaderState, _| {
            button_functions::undo_button(&mut data.reading_state);
        });

    let current_page = Label::dynamic(|data: &CrabReaderState, _env: &_| {
        let page_number = data
            .library
            .get_selected_book()
            .unwrap()
            .get_cumulative_current_page_number();
        let odd = page_number % 2;

        if data.reading_state.single_view.unwrap() {
            format!("Page {}", page_number.to_string())
        } else {
            if odd == 0 {
                format!(
                    "Page {}-{}",
                    page_number.to_string(),
                    (page_number + 1).to_string()
                )
            } else {
                format!(
                    "Page {}-{}",
                    (page_number - 1).to_string(),
                    page_number.to_string()
                )
            }
        }
    })
    .with_text_size(12.0);

    let header_btns = Flex::row()
        .with_child(edit_btn)
        .with_spacer(10.0)
        .with_child(views_btn)
        .align_right();

    let header = Flex::row()
        .with_flex_child(leave_btn, 1.0)
        .with_flex_child(header_btns, 1.0);

    let footer = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.is_editing.unwrap(),
        Flex::row()
            .with_child(undo_changes_btn)
            .with_child(save_changes_btn),
        Flex::row()
            .with_child(back_btn)
            .with_spacer(10.0)
            .with_child(current_page)
            .with_spacer(10.0)
            .with_child(next_btn),
    )
    .center();

    let flex = Flex::column()
        .with_child(header)
        .with_child(title)
        .with_child(current_chapter)
        .with_spacer(20.0)
        .with_child(text)
        .with_flex_spacer(5.0)
        .with_child(footer)
        .padding(15.0);

    flex
}

struct ReadModeDelegate;

impl AppDelegate<CrabReaderState> for ReadModeDelegate {
    fn command(
        &mut self,
        _: &mut druid::DelegateCtx,
        _: druid::Target,
        cmd: &druid::Command,
        data: &mut CrabReaderState,
        _: &Env,
    ) -> Handled {
        match cmd {
            notif if notif.is(ENTERING_READING_MODE) => {
                data.reading = true;
                data.reading_state.enable(
                    data.library
                        .get_selected_book()
                        .unwrap()
                        .get_page_of_chapter(),
                );
                Handled::Yes
            }
            notif if notif.is(LEAVING_READING_MODE) => {
                data.reading = false;
                data.reading_state.disable();
                Handled::Yes
            }
            _ => Handled::No,
        }
    }
}

fn main() -> Result<(), PlatformError> {
    let crab_state = CrabReaderState::default();
    AppLauncher::with_window(
        WindowDesc::new(get_viewswitcher)
            .title("CrabReader")
            .window_size((1280.0, 720.0)),
    )
    .delegate(ReadModeDelegate)
    .launch(crab_state)?;
    Ok(())
}

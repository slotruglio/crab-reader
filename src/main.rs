use crate::models::book::Book;
use crate::utils::colors;
use components::book::book_details::BookDetails;
use components::buttons::{rbtn::RoundedButton, reader_btns::ReaderBtn};
use components::library::cover_library::{CoverLibrary, DO_PAINT_SHADOWS};
use components::library::listing_library::ListLibrary;
use components::mockup::{LibraryFilterLens, MockupLibrary, SortBy};

use components::views::reader_view::{current_chapter_widget, ReaderView};
use components::views::sidebar::Sidebar;
use druid::widget::{Either, Flex, Label, Scroll, SizedBox, ViewSwitcher};
use druid::{
    AppLauncher, Data, Env, Lens, PlatformError, Selector, UnitPoint, Widget, WidgetExt, WindowDesc,
};

use once_cell::sync::Lazy;
use std::rc::Rc;
use std::sync::Mutex;
use traits::gui::{GUIBook, GUILibrary};
use utils::colors::{update_theme, CrabTheme};
use utils::envmanager::MyEnv;
use utils::fonts::Font;
use utils::{ctx_menu, delegates};

mod components;
mod models;
mod traits;
mod utils;
type Library = MockupLibrary<Book>;

pub const ENTERING_READING_MODE: Selector<()> = Selector::new("reading-mode.on");
pub const LEAVING_READING_MODE: Selector<()> = Selector::new("reading-mode.off");
const UP_ARROW: &str = " ↑";
const DOWN_ARROW: &str = " ↓";
const ROUND_FACTR: f64 = 10.0;

//Create a global ENV variable
#[allow(dead_code)]
static MYENV: Lazy<Mutex<MyEnv>> = Lazy::new(|| Mutex::new(MyEnv::new()));

#[derive(Clone, Data, Lens)]
pub struct ReadingState {
    single_view: bool,
    is_editing: bool,
    pages_btn_style: u8,
    sidebar_open: bool,
    text_0: String,
    text_1: String,
    notes: String,
    is_editing_notes: bool,
}

impl ReadingState {
    fn enable<S: Into<Option<Rc<String>>>>(&mut self, _: S) {
        self.single_view = true;
        self.is_editing = false;
        self.is_editing_notes = false;
        self.pages_btn_style = 0;
        self.sidebar_open = false;
    }
    fn disable(&mut self) {
        self.single_view = false;
        self.is_editing = false;
        self.is_editing_notes = false;
        self.pages_btn_style = 0;
        self.sidebar_open = false;
        self.text_0 = String::default();
        self.text_1 = String::default();
        self.notes = String::default();
    }
}

impl Default for ReadingState {
    fn default() -> Self {
        Self {
            single_view: true,
            is_editing: false,
            is_editing_notes: false,
            pages_btn_style: 0,
            sidebar_open: false,
            text_0: String::default(),
            text_1: String::default(),
            notes: String::default(),
        }
    }
}

#[derive(Clone, PartialEq, Data)]
pub enum DisplayMode {
    Cover,
    List,
}

#[derive(Clone, Data, Lens)]
pub struct CrabReaderState {
    library: Library,
    display_mode: DisplayMode,
    reading: bool,
    reading_state: ReadingState,
    ocr: bool,
    ocr_inverse: bool,
    pub theme: CrabTheme,
    pub paint_shadows: bool,
}

impl Default for CrabReaderState {
    fn default() -> Self {
        Self {
            library: Library::new(),
            display_mode: DisplayMode::Cover,
            reading: false,
            reading_state: ReadingState::default(),
            ocr: false,
            ocr_inverse: false,
            theme: CrabTheme::Light,
            paint_shadows: false,
        }
    }
}

fn book_details_panel() -> impl Widget<CrabReaderState> {
    BookDetails::new()
        .background(colors::BACKGROUND_VARIANT)
        .rounded(ROUND_FACTR)
        .lens(CrabReaderState::library)
}

fn title_sorter_btn() -> impl Widget<Library> {
    RoundedButton::dynamic(|data: &Library, _env: &Env| {
        let arrow = match data.get_sort_order() {
            SortBy::Title => DOWN_ARROW,
            SortBy::TitleRev => UP_ARROW,
            _ => "",
        };
        format!("Titolo{}", arrow)
    })
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
    .with_toggle(|data: &Library, _env: &Env| {
        let order = data.get_sort_order();
        order == SortBy::Title || order == SortBy::TitleRev
    })
    .padding(5.0)
}

fn author_sorter_btn() -> impl Widget<Library> {
    RoundedButton::dynamic(|data: &Library, _env: &Env| {
        let arrow = match data.get_sort_order() {
            SortBy::Author => DOWN_ARROW,
            SortBy::AuthorRev => UP_ARROW,
            _ => "",
        };
        format!("Autore{}", arrow)
    })
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
    .with_toggle(|data: &Library, _env: &Env| {
        let order = data.get_sort_order();
        order == SortBy::Author || order == SortBy::AuthorRev
    })
    .padding(5.0)
}

fn filter_fav_btn() -> impl Widget<Library> {
    let emoji_font = Font::default().emoji().xs().get();
    RoundedButton::from_text("❤")
        .with_text_size(18.0)
        .with_on_click(|_, data: &mut Library, _| {
            data.toggle_fav_filter();
        })
        .with_font(emoji_font)
        .with_toggle(|data: &Library, _env: &Env| data.only_fav())
        .secondary()
        .padding(5.0)
}

fn completion_sorter_btn() -> impl Widget<Library> {
    RoundedButton::dynamic(|data: &Library, _env: &Env| {
        let arrow = match data.get_sort_order() {
            SortBy::PercRead => DOWN_ARROW,
            SortBy::PercReadRev => UP_ARROW,
            _ => "",
        };
        format!("Progresso{}", arrow)
    })
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
    .with_toggle(|data: &Library, _env: &Env| {
        let sort = data.get_sort_order();
        sort == SortBy::PercRead || sort == SortBy::PercReadRev
    })
    .padding(5.0)
}

fn picker_sort_by() -> impl Widget<Library> {
    let label = Label::new("Ordina")
        .with_text_color(colors::ON_BACKGROUND)
        .center()
        .expand_width();

    Flex::row()
        .with_flex_child(label, 1.0)
        .with_flex_child(completion_sorter_btn(), 1.0)
        .with_flex_child(author_sorter_btn(), 1.0)
        .with_flex_child(title_sorter_btn(), 1.0)
        .padding(druid::Insets::uniform_xy(15.0, 5.0))
        .background(colors::BACKGROUND_VARIANT)
        .rounded(ROUND_FACTR)
        .expand_width()
}

fn picker_filter_by() -> impl Widget<Library> {
    let text_edit = druid::widget::TextBox::new()
        .with_text_size(18.0)
        .with_placeholder("Titolo, autore, genere...")
        .with_text_color(colors::ON_BACKGROUND)
        .lens(LibraryFilterLens);

    let label = Label::new("Cerca libro")
        .with_text_color(colors::ON_BACKGROUND)
        .center()
        .expand_width();

    Flex::row()
        .with_flex_child(label, 1.0)
        .with_flex_child(text_edit.expand_width(), 3.0)
        .with_flex_child(filter_fav_btn(), 0.5)
        .padding(druid::Insets::uniform_xy(15.0, 10.0))
        .background(colors::BACKGROUND_VARIANT)
        .rounded(ROUND_FACTR)
        .expand_width()
}

fn picker_controller() -> impl Widget<Library> {
    let sort_by = picker_sort_by();
    let filter_by = picker_filter_by();
    Flex::column()
        .with_child(sort_by)
        .with_default_spacer()
        .with_child(filter_by)
}

fn build_ui() -> impl Widget<CrabReaderState> {
    let library_cover = CoverLibrary::new()
        .background(colors::BACKGROUND_VARIANT)
        .rounded(ROUND_FACTR)
        .lens(CrabReaderState::library);

    let library_list = ListLibrary::new().lens(CrabReaderState::library);

    let view_either = Either::new(
        |data: &CrabReaderState, _env| data.display_mode == DisplayMode::List,
        library_list.padding(10.0),
        library_cover,
    )
    .background(colors::BACKGROUND_VARIANT)
    .rounded(ROUND_FACTR);

    let ctls = picker_controller();
    let left_panel = Flex::column()
        .with_child(ctls.lens(CrabReaderState::library))
        .with_default_spacer()
        .with_child(view_either)
        .padding(15.0);
    let scroll = Scroll::new(left_panel)
        .vertical()
        .align_vertical(UnitPoint::TOP);
    let right_panel = Scroll::new(book_details_panel()).vertical();
    let right_col = Flex::column()
        .with_child(
            RoundedButton::dynamic(
                |data: &CrabReaderState, _env: &Env| match data.display_mode {
                    DisplayMode::List => "Passa a visualizzazione a copertine".into(),
                    DisplayMode::Cover => "Passa a visualiazione a liste".into(),
                },
            )
            .with_on_click(|ctx, data: &mut CrabReaderState, _| {
                data.display_mode = match data.display_mode {
                    DisplayMode::List => DisplayMode::Cover,
                    DisplayMode::Cover => DisplayMode::List,
                };
                ctx.request_update();
            })
            .with_text_color(colors::ON_PRIMARY)
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
        VS::Reading => Box::new(read_book_ui().background(colors::BACKGROUND)),
        VS::Browsing => Box::new(build_ui().background(colors::BACKGROUND)),
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
    .with_text_color(colors::ON_BACKGROUND)
    .with_text_size(24.0);

    let current_chapter = current_chapter_widget().with_text_size(16.0).center();

    let sidebar_lx = Sidebar::LEFT.get();

    let sidebar_rx = Sidebar::RIGHT.get();

    let text = Flex::row()
        .with_flex_child(sidebar_lx, 1.0)
        .with_flex_spacer(0.2)
        .with_flex_child(ReaderView::dynamic_view(), 4.0)
        .with_flex_spacer(0.2)
        .with_flex_child(sidebar_rx, 1.0);

    let leave_btn = Flex::row()
        .with_child(ReaderBtn::Leave.button())
        .align_left();

    let next_btn = ReaderBtn::NextPage.button();
    let back_btn = ReaderBtn::PrevPage.button();
    let edit_btn = ReaderBtn::Edit.button().align_right();
    let undo_changes_btn = ReaderBtn::Undo.button();
    let save_changes_btn = ReaderBtn::Save.button();

    let current_page = ReaderBtn::PageNumberSwitch.button();

    let container_page_number = SizedBox::new(current_page.center())
        .width(180.0)
        .height(30.0);

    let header_btns = Flex::row()
        .with_child(edit_btn)
        .align_right();

    let header = Flex::row()
        .with_flex_child(leave_btn, 1.0)
        .with_flex_child(header_btns, 1.0);

    let footer = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.is_editing,
        Flex::row()
            .with_child(undo_changes_btn)
            .with_default_spacer()
            .with_child(save_changes_btn),
        Flex::row()
            .with_flex_spacer(1.0)
            .with_child(back_btn)
            .with_default_spacer()
            .with_child(container_page_number)
            .with_default_spacer()
            .with_child(next_btn)
            .with_flex_spacer(1.0),
    )
    .center();

    let ui = Flex::column()
        .with_child(header)
        .with_child(title)
        .with_child(current_chapter)
        .with_spacer(20.0)
        .with_flex_child(text, 1.0)
        .with_child(footer)
        .padding(15.0);

    ui
}

fn main() -> Result<(), PlatformError> {
    let crab_state = CrabReaderState::default();
    AppLauncher::with_window(
        WindowDesc::new(get_viewswitcher().env_scope(|env, data| update_theme(env, data)))
            .title("CrabReader")
            .window_size((1280.0, 720.0))
            .menu(|_, _, _| ctx_menu::main_window()),
    )
    .delegate(delegates::ReadModeDelegate)
    .launch(crab_state)?;
    Ok(())
}

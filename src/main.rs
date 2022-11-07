use components::book::{Book, BookReading, GUIBook};
use components::book_details::BookDetails;
use components::cover_library::CoverLibrary;
use components::display_mode_button::{DisplayMode, DisplayModeButton};
use components::library::GUILibrary;
use components::listing_library::ListLibrary;
use components::mockup::MockupLibrary;
use components::reader_btns::{ReaderBtn};
use components::reader_view::{ReaderView, current_chapter_widget, title_widget};
use druid::widget::{Button, Either, Flex, Label, Scroll, ViewSwitcher, LineBreaking};
use druid::{
    AppDelegate, AppLauncher, Color, Data, Env, Handled, Lens, PlatformError, Selector, Widget,
    WidgetExt, WindowDesc, EventCtx,
};
use once_cell::sync::Lazy;
use std::rc::Rc;
use std::sync::Mutex;
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
    pages_btn_style: Option<u8>,
    text_0: String,
    text_1: String,
}

impl ReadingState {
    fn enable<S: Into<Option<Rc<String>>>>(&mut self, text: S) {
        self.single_view = Some(true);
        self.is_editing = Some(false);
        self.pages_btn_style = Some(0);
    }
    fn disable(&mut self){
        self.single_view = None;
        self.is_editing = None;
        self.pages_btn_style = None;
        self.text_0 = String::default();
        self.text_1 = String::default();
    }
}

impl Default for ReadingState {
    fn default() -> Self {
        Self {
            single_view: None,
            is_editing: None,
            pages_btn_style: None,
            text_0: String::default(),
            text_1: String::default(),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct CrabReaderState {
    user: UserState,
    library: Library,
    display_mode: DisplayMode,
    reading: bool,
    reading_state: ReadingState
}

impl Default for CrabReaderState {
    fn default() -> Self {
        Self {
            user: UserState::new(),
            library: Library::new(),
            display_mode: DisplayMode::Cover,
            reading: false,
            reading_state: ReadingState::default()
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

    let scroll = Scroll::new(view_either).vertical();

    let right_panel = Scroll::new(book_details_panel()).vertical().padding(5.0);
    let right_col = Flex::column()
        .with_child(
            DisplayModeButton
                .padding(10.0)
                .expand_width()
                .lens(CrabReaderState::display_mode),
        )
        .with_flex_child(right_panel, 1.0);

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
    
    let current_chapter = current_chapter_widget();
    
    let text = ReaderView::dynamic_view();

    let header_btns = Flex::row()
    .with_child(ReaderBtn::Edit.button())
    .with_child(ReaderBtn::ViewsSwitch.button())
    .center();

    let header = Flex::row()
        .with_child(ReaderBtn::Leave.button())
        .with_default_spacer()
        .with_flex_child(title_widget(), 1.0)
        .with_default_spacer()
        .with_child(header_btns)
        .center();
    
    let footer = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.is_editing.unwrap(),
        Flex::row()
        .with_child(ReaderBtn::Undo.button())
        .with_child(ReaderBtn::Save.button())
        .center(),

        Flex::row()
        .with_flex_spacer(2.0)
        .with_child(ReaderBtn::PrevPage.button())
        .with_flex_spacer(1.0)
        .with_child(ReaderBtn::PageNumberSwitch.button())
        .with_flex_spacer(1.0)
        .with_child(ReaderBtn::NextPage.button())
        .with_flex_spacer(2.0)
        .center()
    );

    let ui = Flex::column()
        .with_child(header)
        .with_child(current_chapter)
        .with_spacer(5.0)
        .with_child(text)
        .with_flex_spacer(5.0)
        .with_child(footer)
        .with_spacer(5.0);

    ui
}

struct DumbDelegate;

impl AppDelegate<CrabReaderState> for DumbDelegate {
    fn event(
        &mut self,
        _: &mut druid::DelegateCtx,
        _: druid::WindowId,
        event: druid::Event,
        _: &mut CrabReaderState,
        _: &Env,
    ) -> Option<druid::Event> {
        Some(event)
    }

    fn command(
        &mut self,
        _: &mut druid::DelegateCtx,
        _: druid::Target,
        cmd: &druid::Command,
        data: &mut CrabReaderState,
        _: &Env,
    ) -> Handled {
        println!("Command: {:?}", cmd);
        match cmd {
            notif if notif.is(ENTERING_READING_MODE) => {
                println!("Entering reading mode!");
                data.reading = true;
                data.reading_state.enable(data.library.get_selected_book().unwrap().get_page_of_chapter());

                Handled::Yes
            }
            notif if notif.is(LEAVING_READING_MODE) => {
                println!("Leaving reading mode!");
                data.reading = false;
                data.reading_state.disable();

                Handled::Yes
            }
            _ => Handled::No,
        }
    }

    fn window_added(
        &mut self,
        _: druid::WindowId,
        _: &mut CrabReaderState,
        _: &Env,
        _: &mut druid::DelegateCtx,
    ) {
    }

    fn window_removed(
        &mut self,
        _: druid::WindowId,
        _: &mut CrabReaderState,
        _: &Env,
        _: &mut druid::DelegateCtx,
    ) {
    }
}

fn main() -> Result<(), PlatformError> {
    let crab_state = CrabReaderState::default();
    AppLauncher::with_window(
        WindowDesc::new(get_viewswitcher)
            .title("CrabReader")
            .window_size((1280.0, 720.0)),
    )
    .delegate(DumbDelegate)
    .launch(crab_state)?;
    Ok(())
}

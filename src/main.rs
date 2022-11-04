use components::book::{Book, BookReading, GUIBook, BookManagement};
use components::book_details::BookDetails;
use components::cover_library::CoverLibrary;
use components::display_mode_button::{DisplayMode, DisplayModeButton};
use components::library::GUILibrary;
use components::listing_library::ListLibrary;
use components::mockup::MockupLibrary;
use components::reader_view::{build_single_view_edit, build_single_view, build_dual_view};
use druid::widget::{Button, Either, Flex, Label, LineBreaking, Scroll, ViewSwitcher, Switch};
use druid::{
    AppDelegate, AppLauncher, Color, Data, Env, Handled, Lens, PlatformError, Selector, Widget,
    WidgetExt, WindowDesc,
};
use once_cell::sync::Lazy;
use utils::button_functions; // 1.3.1
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
    text: Option<Rc<String>>
}

impl ReadingState {
    fn enable<S: Into<Option<Rc<String>>>>(&mut self, text: S) {
        self.single_view = Some(true);
        self.is_editing = Some(false);
        self.text = text.into()
    }
    fn disable(&mut self){
        self.single_view = None;
        self.is_editing = None;
        self.text = None;
    }
}

impl Default for ReadingState {
    fn default() -> Self {
        Self {
            single_view: None,
            is_editing: None,
            text: None
        }
    }
}

#[derive(Clone, Data, Lens)]
struct CrabReaderState {
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
    
    let title = Label::dynamic(
        |data: &CrabReaderState, _env: &_| data.library.get_selected_book().unwrap().get_title().to_string(),
    )
        .with_text_size(32.0)
        .padding(10.0)
        .center();
    
    let current_chapter = Label::dynamic(
        |data: &CrabReaderState, _env: &_| format!("Chapter {}",data.library.get_selected_book().unwrap().get_chapter_number().to_string())
    )
        .with_text_size(16.0)
        .padding(10.0)
        .center();
    
    let text = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.is_editing.unwrap(),
        build_single_view_edit(),
        Either::new(
            |data: &CrabReaderState, _env| data.reading_state.single_view.unwrap(),
            build_single_view(),
            build_dual_view()
        )
    ).fix_size(800.0, 450.0);
    
    let leave_btn = Button::new("Go back to Browsing")
        .on_click(|_, data: &mut CrabReaderState, _| {
            data.reading = false;
        })
        .fix_height(64.0)
        .center();
    
    // todo() switch to change single view and double view
    // this is a mock to test layout
    let views_btn = Button::new("Single/Double View")
        .on_click(|_, data: &mut CrabReaderState, _| {
            data.reading_state.single_view = Some(!data.reading_state.single_view.unwrap())
        })
        .fix_height(64.0)
        .center();
    
    let next_btn = Button::new("Next")
        .on_click(|ctx, data: &mut CrabReaderState, _| {

            let book = data.library.get_selected_book_mut().unwrap();
            button_functions::change_page(ctx, book, data.reading_state.is_editing.unwrap(), data.reading_state.single_view.unwrap(), true);
        })
        .center();

    let back_btn = Button::new("Back")
        .on_click(|ctx, data: &mut CrabReaderState, _| {

            let book = data.library.get_selected_book_mut().unwrap();
            button_functions::change_page(ctx, book, data.reading_state.is_editing.unwrap(), data.reading_state.single_view.unwrap(), false);
        })
        .center();

    let edit_btn = Button::new("Edit")
        .on_click(|ctx, data: &mut CrabReaderState, _| {
            let (status, text) = button_functions::edit_button(
                ctx, 
                data.library.get_selected_book_mut().unwrap(), 
                data.reading_state.text.as_ref().unwrap().to_string(),
                data.reading_state.is_editing.unwrap()
            );

            data.reading_state.text = Some(text.into());
            data.reading_state.is_editing = Some(status);
        })
        .fix_height(64.0)
        .center();

    let save_changes_btn = Button::new("Save")
    .on_click(|ctx, data: &mut CrabReaderState, _| {
        data.library.get_selected_book_mut().unwrap().edit_text(data.reading_state.text.as_ref().unwrap().to_string());
        data.reading_state.is_editing = Some(false);
        ctx.request_paint();
    })
    .center();

    let undo_changes_btn = Button::new("Undo")
    .on_click(|_, data: &mut CrabReaderState, _| {
        data.reading_state.is_editing = Some(false);
    })
    .center();

    let current_page = Label::dynamic(
        |data: &CrabReaderState, _env: &_| {
            let page_number = data.library.get_selected_book().unwrap().get_cumulative_current_page_number();
            let odd = page_number % 2;

            if data.reading_state.single_view.unwrap() {
                format!("Page {}", page_number.to_string())
            } else {
                if odd == 0 {
                    format!("Page {}-{}", page_number.to_string(), (page_number + 1).to_string())
                } else {
                    format!("Page {}-{}", (page_number - 1).to_string(), page_number.to_string())
                }
            }
        }
    )
        .with_text_size(12.0)
        .padding(10.0)
        .center();

    let header_btns = Flex::row()
        .with_child(edit_btn)
        .with_child(views_btn)
        .center();

    let header = Flex::row()
        .with_child(leave_btn)
        .with_default_spacer()
        .with_flex_child(title, 1.0)
        .with_default_spacer()
        .with_child(header_btns)
        .center();
    
    let footer = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.is_editing.unwrap(),
        Flex::row()
        .with_child(undo_changes_btn)
        .with_child(save_changes_btn)
        .center(),

        Flex::row()
        .with_child(back_btn)
        .with_child(current_page)
        .with_child(next_btn)
        .center()
    );

    let flex = Flex::column()
        .with_child(header)
        .with_child(current_chapter)
        .with_spacer(5.0)
        .with_child(text)
        .with_flex_spacer(5.0)
        .with_child(footer)
        .with_spacer(5.0);

    flex
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

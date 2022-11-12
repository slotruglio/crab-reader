use components::book::{Book, BookReading, GUIBook};
use components::book_details::BookDetails;
use components::cover_library::CoverLibrary;
use components::display_mode_button::{DisplayMode, DisplayModeButton};
use components::library::GUILibrary;
use components::listing_library::ListLibrary;
use components::mockup::{MockupLibrary, SortBy};
use components::rbtn::RoundedButton;
use components::reader_view::{
    build_dual_view, build_dual_view_edit, build_single_view, build_single_view_edit,
};
use druid::widget::{Button, Controller, Either, Flex, Label, Scroll, ViewSwitcher};
use druid::{
    AppDelegate, AppLauncher, Color, Data, Env, EventCtx, Handled, Lens, PlatformError, Selector,
    Widget, WidgetExt, WindowDesc, FileSpec, Command, Target
};

use druid::FileDialogOptions;

use druid::commands::{SHOW_OPEN_PANEL, OPEN_FILE};

use once_cell::sync::Lazy;
use std::rc::Rc;
use std::sync::Mutex;
use utils::{button_functions, ocrmanager}; // 1.3.1
use utils::envmanager::MyEnv;

use crate::components::book::BookManagement;

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

struct FilterController;

impl<W: Widget<Library>> Controller<MockupLibrary<Book>, W> for FilterController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut MockupLibrary<Book>,
        env: &Env,
    ) {
        let filter = data.get_filter_text_input();
        if filter != *data.get_filter_string() {
            data.set_filter_string(filter);
        }
        child.event(ctx, event, data, env)
    }
}

fn picker_filter_by() -> impl Widget<Library> {
    let text_edit = druid::widget::TextBox::new()
        .with_placeholder("Filter by")
        .lens(Library::filter_text_input)
        .controller(FilterController)
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
    let title = Label::dynamic(|data: &CrabReaderState, _env: &_| {
        data.library
            .get_selected_book()
            .unwrap()
            .get_title()
            .to_string()
    })
    .with_text_size(32.0)
    .padding(10.0)
    .center();

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
    .padding(10.0)
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

    let ocr_btn = Button::new("OCR")
        .on_click(|event_ctx, _: &mut CrabReaderState, _| {

            //Trigger a FILE PICKER
            let cmd = Command::new(
                SHOW_OPEN_PANEL,
                FileDialogOptions::new().allowed_types(vec![FileSpec::JPG, FileSpec::PNG]),
                Target::Auto,
            );

            event_ctx.submit_command(cmd);

        })
        .fix_height(64.0)
        .center();

    let next_btn = Button::new("Next")
        .on_click(|ctx, data: &mut CrabReaderState, _| {
            println!("DEBUG: PRESSED NEXT START");
            let book = data.library.get_selected_book_mut().unwrap();
            button_functions::change_page(
                ctx,
                book,
                data.reading_state.is_editing.unwrap(),
                data.reading_state.single_view.unwrap(),
                true,
            );
            println!("DEBUG: PRESSED NEXT END\n");
        })
        .center();

    let back_btn = Button::new("Back")
        .on_click(|ctx, data: &mut CrabReaderState, _| {
            println!("DEBUG: PRESSED BACK START");
            let book = data.library.get_selected_book_mut().unwrap();
            button_functions::change_page(
                ctx,
                book,
                data.reading_state.is_editing.unwrap(),
                data.reading_state.single_view.unwrap(),
                false,
            );

            println!("DEBUG: PRESSED BACK END\n");
        })
        .center();

    let edit_btn = Button::new("Edit")
        .on_click(|_, data: &mut CrabReaderState, _| {
            println!("DEBUG: PRESSED EDIT BUTTON");

            button_functions::edit_button(
                &mut data.reading_state,
                data.library.get_selected_book().unwrap(),
            );
        })
        .fix_height(64.0)
        .center();

    let save_changes_btn = Button::new("Save")
        .on_click(|ctx: &mut EventCtx, data: &mut CrabReaderState, _| {
            println!("DEBUG: PRESSED SAVE BUTTON");

            button_functions::save_button(
                ctx,
                &mut data.reading_state,
                &mut data.library.get_selected_book_mut().unwrap(),
            );
        })
        .center();

    let undo_changes_btn = Button::new("Undo")
        .on_click(|_, data: &mut CrabReaderState, _| {
            button_functions::undo_button(&mut data.reading_state);
        })
        .center();

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
    .with_text_size(12.0)
    .padding(10.0)
    .center();

    let header_btns = Flex::row()
        .with_child(ocr_btn)
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
            .center(),
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
        match cmd {
            notif if notif.is(ENTERING_READING_MODE) => {
                println!("Entering reading mode!");
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
                println!("Leaving reading mode!");
                data.reading = false;
                data.reading_state.disable();

                Handled::Yes
            }
            notif if notif.is(OPEN_FILE) => {
                println!("Opening file!");
                let file = cmd.get_unchecked(OPEN_FILE);

                //get file path
                let path = file.path();

                let selected_book_path = data.library.get_selected_book().unwrap().get_path();
                
                //split by slash, get last element, split by dot, get first element
                let folder_name = selected_book_path.split("/").last().unwrap().split(".").next().unwrap();

                //call ocr on the img path
                let ocr_result = ocrmanager::get_ebook_page(folder_name.to_string(), path.to_str().unwrap().to_string());

                match ocr_result {
                    Some(ocr_result) => {
                        //move to the found page
                        data.library.get_selected_book_mut().unwrap().set_chapter_number(ocr_result.0, true);
                        data.library.get_selected_book_mut().unwrap().set_chapter_current_page_number(ocr_result.1);
                    }
                    None => {
                        println!("ERROR: OCR page not found");
                    }
                }

                

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

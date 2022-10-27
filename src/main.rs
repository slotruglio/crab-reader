use components::book::Book;
use components::book_details::BookDetails;
use components::cover_library::CoverLibrary;
use components::display_mode_button::{DisplayMode, DisplayModeButton};
use components::listing_library::ListLibrary;
use components::mockup::MockupLibrary;
use druid::widget::{Either, Flex, Label, LineBreaking, Scroll, ViewSwitcher};
use druid::{AppLauncher, Color, Data, Env, Lens, PlatformError, Widget, WidgetExt, WindowDesc};
use once_cell::sync::Lazy; // 1.3.1
use std::rc::Rc;
use std::sync::Mutex;
use utils::envmanager::MyEnv;

mod components;
mod utils;
type Library = MockupLibrary<Book>;

pub const LIPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Arcu cursus vitae congue mauris rhoncus aenean. Elit ut aliquam purus sit. Feugiat pretium nibh ipsum consequat. Ornare massa eget egestas purus. Orci a scelerisque purus semper. Vel pharetra vel turpis nunc eget lorem dolor sed. Scelerisque purus semper eget duis at tellus at urna. Urna neque viverra justo nec ultrices. Tellus molestie nunc non blandit. Risus viverra adipiscing at in tellus integer feugiat scelerisque. Vel elit scelerisque mauris pellentesque pulvinar pellentesque habitant morbi. A erat nam at lectus urna duis. Consequat semper viverra nam libero justo. Purus in massa tempor nec feugiat nisl pretium. Semper feugiat nibh sed pulvinar proin gravida hendrerit lectus a. Quam viverra orci sagittis eu volutpat odio facilisis mauris sit. Vel facilisis volutpat est velit egestas dui id ornare arcu. Aliquet sagittis id consectetur purus ut.
Habitant morbi tristique senectus et netus et. Enim sit amet venenatis urna cursus. Tellus molestie nunc non blandit massa. In metus vulputate eu scelerisque felis imperdiet proin fermentum leo. Pharetra pharetra massa massa ultricies mi. Enim nunc faucibus a pellentesque sit. Blandit turpis cursus in hac habitasse. Auctor elit sed vulputate mi sit amet mauris commodo quis. Non enim praesent elementum facilisis leo. Accumsan sit amet nulla facilisi. Urna molestie at elementum eu facilisis sed. Eget felis eget nunc lobortis. Dolor sit amet consectetur adipiscing elit.
Tempus egestas sed sed risus pretium. Fames ac turpis egestas sed tempus urna. Nec ullamcorper sit amet risus nullam. Volutpat blandit aliquam etiam erat velit scelerisque. Mattis vulputate enim nulla aliquet porttitor. Ultricies mi eget mauris pharetra et ultrices neque ornare aenean. Luctus venenatis lectus magna fringilla urna porttitor. Convallis a cras semper auctor. Turpis massa sed elementum tempus egestas. Suspendisse in est ante in nibh mauris. Viverra justo nec ultrices dui. Id volutpat lacus laoreet non. Sed lectus vestibulum mattis ullamcorper velit sed ullamcorper morbi. Nec nam aliquam sem et tortor consequat id porta. Libero enim sed faucibus turpis. Nisl nisi scelerisque eu ultrices vitae. Pharetra et ultrices neque ornare aenean euismod. Dui accumsan sit amet nulla facilisi morbi tempus iaculis.
At tellus at urna condimentum mattis pellentesque. Interdum posuere lorem ipsum dolor sit amet consectetur. Lectus quam id leo in vitae turpis massa sed elementum. Tellus at urna condimentum mattis pellentesque id. Condimentum lacinia quis vel eros donec ac odio tempor. Congue mauris rhoncus aenean vel elit scelerisque. Commodo nulla facilisi nullam vehicula ipsum. Leo urna molestie at elementum. Et netus et malesuada fames. Phasellus faucibus scelerisque eleifend donec pretium vulputate. Egestas maecenas pharetra convallis posuere morbi leo urna molestie. Sagittis orci a scelerisque purus semper eget duis at tellus. Nec tincidunt praesent semper feugiat nibh sed pulvinar proin gravida. Lacus sed turpis tincidunt id aliquet. Tincidunt arcu non sodales neque sodales ut etiam sit.
Imperdiet massa tincidunt nunc pulvinar sapien et ligula ullamcorper malesuada. Turpis egestas integer eget aliquet nibh praesent. Bibendum neque egestas congue quisque egestas. Interdum velit laoreet id donec ultrices. Nullam vehicula ipsum a arcu cursus vitae congue. In vitae turpis massa sed elementum tempus egestas sed. Duis tristique sollicitudin nibh sit amet commodo nulla facilisi nullam. Viverra adipiscing at in tellus integer feugiat. Pretium aenean pharetra magna ac placerat vestibulum lectus. Odio ut enim blandit volutpat maecenas volutpat blandit aliquam. Metus dictum at tempor commodo ullamcorper a lacus vestibulum. Mauris vitae ultricies leo integer malesuada nunc vel risus.
Adipiscing tristique risus nec feugiat in fermentum. Lorem ipsum dolor sit amet consectetur adipiscing elit. At varius vel pharetra vel turpis nunc eget. Eget mauris pharetra et ultrices neque ornare aenean. Nibh mauris cursus mattis molestie a iaculis. Diam donec adipiscing tristique risus nec feugiat. Libero justo laoreet sit amet. Nam at lectus urna duis. Facilisis leo vel fringilla est ullamcorper eget nulla. Consectetur lorem donec massa sapien faucibus et molestie ac. Urna nunc id cursus metus aliquam eleifend mi in nulla. Lectus sit amet est placerat.
Ac odio tempor orci dapibus ultrices in iaculis. Turpis egestas integer eget aliquet nibh praesent. Porta lorem mollis aliquam ut porttitor. Vulputate mi sit amet mauris commodo quis imperdiet massa tincidunt. Lorem ipsum dolor sit amet consectetur adipiscing elit ut aliquam. Tempor nec feugiat nisl pretium fusce id velit ut tortor. Sed velit dignissim sodales ut eu sem integer vitae justo. Sapien nec sagittis aliquam malesuada bibendum arcu vitae elementum curabitur. Et molestie ac feugiat sed lectus vestibulum mattis. Dui accumsan sit amet nulla facilisi. At tempor commodo ullamcorper a lacus vestibulum. Nibh venenatis cras sed felis eget.
Eget duis at tellus at urna. Aenean euismod elementum nisi quis. Dictumst quisque sagittis purus sit amet volutpat consequat. Vitae justo eget magna fermentum iaculis eu. At tempor commodo ullamcorper a lacus vestibulum sed arcu. Morbi tempus iaculis urna id. Risus pretium quam vulputate dignissim suspendisse in. Quis lectus nulla at volutpat diam ut venenatis tellus. Et malesuada fames ac turpis egestas maecenas. Lobortis feugiat vivamus at augue. Nulla pharetra diam sit amet nisl suscipit adipiscing bibendum est.
Habitant morbi tristique senectus et netus et malesuada. Tellus orci ac auctor augue mauris augue. Id venenatis a condimentum vitae sapien pellentesque habitant morbi tristique. Quam lacus suspendisse faucibus interdum posuere lorem ipsum. Luctus accumsan tortor posuere ac ut consequat. Orci porta non pulvinar neque laoreet. Scelerisque eleifend donec pretium vulputate sapien nec sagittis aliquam malesuada. Mauris vitae ultricies leo integer malesuada nunc vel risus commodo. Velit laoreet id donec ultrices tincidunt arcu non. Nec sagittis aliquam malesuada bibendum arcu vitae. Dui accumsan sit amet nulla facilisi morbi tempus iaculis. Accumsan in nisl nisi scelerisque eu ultrices vitae auctor. Dictumst quisque sagittis purus sit amet volutpat consequat mauris.
Ut consequat semper viverra nam libero justo. Non tellus orci ac auctor augue mauris augue neque gravida. Tempus imperdiet nulla malesuada pellentesque. Non pulvinar neque laoreet suspendisse interdum consectetur libero id faucibus. Semper eget duis at tellus at. Malesuada nunc vel risus commodo viverra. Varius morbi enim nunc faucibus a pellentesque sit amet. Iaculis eu non diam phasellus vestibulum lorem sed risus. Pharetra et ultrices neque ornare aenean euismod elementum. Bibendum neque egestas congue quisque egestas diam. Feugiat in fermentum posuere urna nec tincidunt praesent semper. Lorem donec massa sapien faucibus et molestie ac feugiat. Suscipit tellus mauris a diam maecenas sed enim.";

//Create a global ENV variable
#[allow(dead_code)]
static MYENV: Lazy<Mutex<MyEnv>> = Lazy::new(|| Mutex::new(MyEnv::new()));

#[derive(Clone, Data, Lens)]
pub struct AppState {
    single_view: bool,
    is_editing: bool,
    book: Rc<Book>,
    text: Rc<String>,
}

#[derive(Clone, Data, Lens)]
struct CrabReaderState {
    user: UserState,
    library: Library,
    display_mode: DisplayMode,
    reading: bool,
}

impl Default for CrabReaderState {
    fn default() -> Self {
        Self {
            user: UserState::new(),
            library: Library::new(),
            display_mode: DisplayMode::Cover,
            reading: true,
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
    let title = Label::new("Entered Book Reading Mode")
        .with_text_size(32.0)
        .padding(10.0)
        .center();
    let text = Label::new(LIPSUM)
        .with_line_break_mode(LineBreaking::WordWrap)
        .padding(10.0);
    let flex = Flex::column()
        .with_child(title)
        .with_spacer(5.0)
        .with_child(text);
    Scroll::new(flex).vertical()
}

fn main() -> Result<(), PlatformError> {
    let crab_state = CrabReaderState::default();
    AppLauncher::with_window(
        WindowDesc::new(get_viewswitcher)
            .title("CrabReader")
            .window_size((1280.0, 720.0)),
    )
    .launch(crab_state)?;
    Ok(())
}

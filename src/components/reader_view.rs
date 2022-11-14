use druid::widget::{
    Button, Container, CrossAxisAlignment, Either, Flex, Label, LineBreaking, List, ListIter,
    MainAxisAlignment, RawLabel, Scroll, TextBox,
};
use druid::{Color, EventCtx, LensExt, TextAlignment, UnitPoint, Widget, WidgetExt, WidgetPod};

use crate::{CrabReaderState, Library, ReadingState};

use super::book::{BookReading, GUIBook};
use super::library::GUILibrary;
use super::rbtn::RoundedButton;
use super::reader_btns::{chapter_label, ReaderBtn};

use crate::MYENV;

pub enum ReaderView {
    Single,
    SingleEdit,
    Dual,
    DualEdit,
}

impl ReaderView {
    pub fn get_view(&self) -> impl Widget<CrabReaderState> {
        match self {
            ReaderView::Single => single_view_widget(),
            ReaderView::SingleEdit => single_view_edit_widget(),
            ReaderView::Dual => dual_view_widget(),
            ReaderView::DualEdit => dual_view_edit_widget(),
        }
    }
    /// Returns a widget with the correct widget to show page(s) in reading or edit mode
    pub fn dynamic_view() -> impl Widget<CrabReaderState> {
        let either = Either::new(
            |data: &CrabReaderState, _env| data.reading_state.single_view,
            Either::new(
                |data: &CrabReaderState, _env| data.reading_state.is_editing,
                ReaderView::SingleEdit.get_view(),
                ReaderView::Single.get_view(),
            ),
            Either::new(
                |data: &CrabReaderState, _env| data.reading_state.is_editing,
                ReaderView::DualEdit.get_view(),
                ReaderView::Dual.get_view(),
            ),
        )
        .center()
        .padding(10.0)
        .fix_size(800.0, 450.0);

        either
    }
}

// single page view for text reader
fn single_view_widget() -> Container<CrabReaderState> {
    let myenv = MYENV.lock().unwrap();
    let font = myenv.font.clone();
    let font_color = myenv.font_color.clone();

    let view = Scroll::new(
        Label::dynamic(|data: &CrabReaderState, _env: &_| {
            data.library
                .get_selected_book()
                .unwrap()
                .get_page_of_chapter()
                .to_string()
        })
        .with_text_color(font_color)
        .with_font(font)
        .with_text_alignment(TextAlignment::Justified)
        .with_line_break_mode(LineBreaking::WordWrap),
    )
    .vertical();

    Container::new(view)
}

// single page view for text editing
fn single_view_edit_widget() -> Container<CrabReaderState> {
    let myenv = MYENV.lock().unwrap();
    let font = myenv.font.clone();
    let font_color = myenv.font_color.clone();

    let text_box = TextBox::multiline()
        .with_text_color(font_color)
        .with_font(font)
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0));

    let view = Scroll::new(text_box.fix_size(500.0, 500.0)).vertical();

    Container::new(view)
}

// dual page view for text reader
fn dual_view_widget() -> Container<CrabReaderState> {
    let myenv = MYENV.lock().unwrap();
    let font = myenv.font.clone();
    let font_color = myenv.font_color.clone();

    let views = Flex::row()
        .with_flex_child(
            Scroll::new(
                Label::dynamic(|data: &CrabReaderState, _env: &_| {
                    data.library
                        .get_selected_book()
                        .unwrap()
                        .get_dual_pages()
                        .0
                        .to_string()
                })
                .with_text_color(font_color.clone())
                .with_font(font.clone())
                .with_text_alignment(TextAlignment::Justified)
                .with_line_break_mode(LineBreaking::WordWrap),
            )
            .vertical()
            .fix_size(400.0, 300.0),
            1.0,
        )
        .with_spacer(20.0)
        .with_flex_child(
            Scroll::new(
                Label::dynamic(|data: &CrabReaderState, _env: &_| {
                    data.library
                        .get_selected_book()
                        .unwrap()
                        .get_dual_pages()
                        .1
                        .to_string()
                })
                .with_text_color(font_color)
                .with_font(font)
                .with_text_alignment(TextAlignment::Justified)
                .with_line_break_mode(LineBreaking::WordWrap),
            )
            .vertical()
            .fix_size(400.0, 300.0),
            1.0,
        );

    Container::new(views)
}

// dual page view for text editing
fn dual_view_edit_widget() -> Container<CrabReaderState> {
    let myenv = MYENV.lock().unwrap();
    let font = myenv.font.clone();
    let font_color = myenv.font_color.clone();

    let text_box_page_0 = TextBox::multiline()
        .with_text_color(font_color.clone())
        .with_font(font.clone())
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0));

    let text_box_page_1 = TextBox::multiline()
        .with_text_color(font_color)
        .with_font(font)
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_1));

    let views = Flex::row()
        .with_child(Scroll::new(text_box_page_0.fix_size(500.0, 500.0)).vertical())
        .with_spacer(10.0)
        .with_child(Scroll::new(text_box_page_1.fix_size(500.0, 500.0)).vertical());
    Container::new(views)
}

pub fn title_widget() -> impl Widget<CrabReaderState> {
    Label::dynamic(|data: &CrabReaderState, _env: &_| {
        data.library
            .get_selected_book()
            .unwrap()
            .get_title()
            .to_string()
    })
    .with_line_break_mode(LineBreaking::Clip)
    .with_text_size(32.0)
    .padding(10.0)
    .center()
}

pub fn current_chapter_widget() -> impl Widget<CrabReaderState> {
    Label::dynamic(|data: &CrabReaderState, _env: &_| {
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
    .center()
}

pub fn sidebar_widget() -> impl Widget<CrabReaderState> {
    let btn = RoundedButton::dynamic(|data: &ReadingState, _env: &_| {
        if !data.sidebar_open {
            "Apri selezione capitoli".into()
        } else {
            "Chiudi selezione capitoli".into()
        }
    })
    .with_on_click(|ctx, data: &mut ReadingState, _env| {
        data.sidebar_open = !data.sidebar_open;
        ctx.request_layout();
    })
    .with_color(Color::rgb8(70, 70, 70))
    .with_hot_color(Color::rgb8(50, 50, 50))
    .with_active_color(Color::rgb8(20, 20, 20))
    .with_text_color(Color::WHITE)
    .with_text_size(18.0)
    .align_vertical(UnitPoint::CENTER)
    .lens(CrabReaderState::reading_state);

    let sidebar_closed = Flex::column();
    let chapters_list = ChaptersList { children: vec![] };

    let sidebar_open = Flex::column()
        .with_child(chapters_list)
        .lens(CrabReaderState::library);

    let sidebar = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.sidebar_open,
        Scroll::new(sidebar_open).vertical().fix_height(500.0),
        sidebar_closed,
    );

    Flex::column().with_child(btn).with_child(sidebar)
}

struct ChaptersList {
    children: Vec<WidgetPod<Library, Box<dyn Widget<Library>>>>,
}

impl Widget<Library> for ChaptersList {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut Library,
        env: &druid::Env,
    ) {
        for pod in self.children.iter_mut() {
            pod.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Library,
        env: &druid::Env,
    ) {
        while data.get_selected_book().unwrap().get_number_of_chapters() > self.children.len() {
            let idx = self.children.len();
            self.children.push(WidgetPod::new(Box::new(
                Label::dynamic(move |_: &Library, _env: &_| format!("Captiolo {}", idx + 1))
                    .on_click(move |ctx, data: &mut Library, _env| {
                        data.get_selected_book_mut()
                            .unwrap()
                            .set_chapter_number(idx, false);
                        ctx.request_layout();
                    }),
            )));
        }

        for pod in self.children.iter_mut() {
            pod.lifecycle(ctx, event, data, env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _: &Library,
        data: &Library,
        env: &druid::Env,
    ) {
        for pod in self.children.iter_mut() {
            pod.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Library,
        env: &druid::Env,
    ) -> druid::Size {
        let mut h = 0.0;
        for pod in self.children.iter_mut() {
            let size = pod.layout(ctx, bc, data, env);
            pod.set_origin(ctx, data, env, (0.0, h).into());
            h += size.height;
        }

        let w = if bc.is_width_bounded() {
            bc.max().width
        } else {
            400.0
        };

        (w, h).into()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Library, env: &druid::Env) {
        for pod in self.children.iter_mut() {
            pod.paint(ctx, data, env);
        }
    }
}

use druid::{
    widget::{Container, Either, Flex, Label, LineBreaking, Scroll, TextBox},
    Color, FontDescriptor, LensExt, TextAlignment, UnitPoint, Widget, WidgetExt,
};

use crate::{
    components::{buttons::rbtn::RoundedButton, chapter_selector::ChapterSelector},
    traits::{
        gui::{GUIBook, GUILibrary},
        note::NoteManagement,
        reader::BookReading,
    },
    utils::colors,
    CrabReaderState, ReadingState, MYENV,
};

pub enum ReaderView {
    Single,
    SingleEdit,
    Dual,
    DualEdit,
}

impl ReaderView {
    pub fn get_view(&self) -> impl Widget<CrabReaderState> {
        let myenv = MYENV.lock().unwrap();
        let font = myenv.font.clone();
        let font_color = myenv.font_color.clone();

        match self {
            ReaderView::Single => single_view_widget(font, font_color),
            ReaderView::SingleEdit => single_view_edit_widget(font, font_color), // single_view_edit_widget(font, font_color),
            ReaderView::Dual => dual_view_widget(font, font_color),
            ReaderView::DualEdit => dual_view_edit_widget(font, font_color),
        }
    }

    /// Returns a widget with the correct widget to show page(s) in reading or edit mode
    pub fn dynamic_view() -> impl Widget<CrabReaderState> {
        Either::new(
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
        .background(colors::BACKGROUND)
        .center()
        .expand()
    }
}

// single page view for text reader
fn single_view_widget(font: FontDescriptor, font_color: Color) -> Container<CrabReaderState> {
    let inner = Scroll::new(
        Label::dynamic(|data: &CrabReaderState, _env: &_| {
            data.library
                .get_selected_book()
                .unwrap()
                .get_page_of_chapter()
        })
        .with_text_color(colors::ON_BACKGROUND)
        .with_font(font)
        .with_text_alignment(TextAlignment::Justified)
        .with_line_break_mode(LineBreaking::WordWrap),
    )
    .vertical();

    Container::new(inner)
}

// single page view for text editing
fn single_view_edit_widget(font: FontDescriptor, font_color: Color) -> Container<CrabReaderState> {
    let tb = TextBox::multiline()
        .with_text_color(colors::ON_BACKGROUND)
        .with_font(font)
        .with_placeholder("Text editing is not yet implemented")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0))
        .expand_width();

    Container::new(Scroll::new(tb).vertical())
}

// dual page view for text reader
fn dual_view_widget(font: FontDescriptor, font_color: Color) -> Container<CrabReaderState> {
    let inner = Flex::row()
        .with_flex_child(
            Scroll::new(
                Label::dynamic(|data: &CrabReaderState, _env: &_| {
                    data.library.get_selected_book().unwrap().get_dual_pages().0
                })
                .with_text_color(colors::ON_BACKGROUND)
                .with_font(font.clone())
                .with_text_alignment(TextAlignment::Justified)
                .with_line_break_mode(LineBreaking::WordWrap),
            )
            .vertical(),
            1.0,
        )
        .with_flex_spacer(0.1)
        .with_flex_child(
            Scroll::new(
                Label::dynamic(|data: &CrabReaderState, _env: &_| {
                    data.library.get_selected_book().unwrap().get_dual_pages().1
                })
                .with_text_color(colors::ON_BACKGROUND)
                .with_font(font)
                .with_text_alignment(TextAlignment::Justified)
                .with_line_break_mode(LineBreaking::WordWrap)
                .expand_width(),
            )
            .vertical(),
            1.0,
        );
    Container::new(inner)
}

// dual page view for text editing
fn dual_view_edit_widget(font: FontDescriptor, font_color: Color) -> Container<CrabReaderState> {
    let text_box_page_0 = TextBox::multiline()
        .with_text_color(colors::ON_BACKGROUND)
        .with_font(font.clone())
        .with_placeholder("There is no page here... but you can add one!")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_0));

    let text_box_page_1 = TextBox::multiline()
        .with_text_color(colors::ON_BACKGROUND)
        .with_font(font)
        .with_placeholder("There is no page here... but you can add one!")
        .lens(CrabReaderState::reading_state.then(ReadingState::text_1));

    let inner = Flex::row()
        .with_flex_child(Scroll::new(text_box_page_0).vertical(), 1.0)
        .with_flex_spacer(0.1)
        .with_flex_child(Scroll::new(text_box_page_1).vertical(), 1.0);

    Container::new(inner)
}

pub fn title_widget() -> impl Widget<CrabReaderState> {
    Label::dynamic(|data: &CrabReaderState, _env: &_| {
        data.library
            .get_selected_book()
            .unwrap()
            .get_title()
            .to_string()
    })
    .with_text_color(colors::ON_BACKGROUND)
    .with_line_break_mode(LineBreaking::Clip)
    .with_text_size(32.0)
    .padding(10.0)
    .center()
}

pub fn current_chapter_widget() -> Label<CrabReaderState> {
    Label::dynamic(|data: &CrabReaderState, _env: &_| {
        // + 1
        let display_number = data
            .library
            .get_selected_book()
            .unwrap()
            .get_chapter_number()
            + 1;

        format!("Chapter {}", display_number)
    })
    .with_text_color(colors::ON_BACKGROUND)
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
    })
    .with_text_size(18.0)
    .align_horizontal(UnitPoint::CENTER)
    .lens(CrabReaderState::reading_state);

    let sidebar_closed = Flex::column();

    let cs = ChapterSelector::new().lens(CrabReaderState::library);
    let sidebar_open = Flex::column().with_child(cs);

    let sidebar = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.sidebar_open,
        Scroll::new(sidebar_open).vertical(),
        sidebar_closed,
    );

    Flex::column()
        .with_child(btn)
        .with_default_spacer()
        .with_flex_child(sidebar, 1.0)
}

pub fn sidebar_right_widget() -> impl Widget<CrabReaderState> {
    let myenv = MYENV.lock().unwrap();

    let notes = Label::dynamic(|data: &CrabReaderState, _env: &_| {
        data.library
            .get_selected_book()
            .unwrap()
            .get_current_note()
            .unwrap_or("".to_string())
    })
    .with_line_break_mode(LineBreaking::WordWrap)
    .with_text_color(colors::ON_BACKGROUND);

    let tb = TextBox::multiline()
        .with_placeholder("Scrivi...")
        .lens(CrabReaderState::reading_state.then(ReadingState::notes));

    let notes_either = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.is_editing_notes,
        tb,
        notes,
    );

    let edit_note_btn = RoundedButton::dynamic(|data: &CrabReaderState, _env: &_| {
        if data
            .library
            .get_selected_book()
            .unwrap()
            .get_current_note()
            .is_none()
        {
            "Aggiungi nota".into()
        } else {
            "Modifica nota".into()
        }
    })
    .with_on_click(|_, data: &mut CrabReaderState, _| {
        data.reading_state.notes = data
            .library
            .get_selected_book()
            .unwrap()
            .get_current_note()
            .unwrap_or_default();

        data.reading_state.is_editing_notes = true;
    });

    let del_note_btn = RoundedButton::from_text("Rimuovi nota").with_on_click(
        |ctx, data: &mut CrabReaderState, _| {
            data.library.get_selected_book_mut().unwrap().delete_note();
        },
    );

    let undo_note_btn =
        RoundedButton::from_text("Annulla").with_on_click(|ctx, data: &mut CrabReaderState, _| {
            data.reading_state.is_editing_notes = false;
        });

    let save_note_btn = RoundedButton::from_text("Salva")
        .with_on_click(|ctx, data: &mut CrabReaderState, _| {
            data.library
                .get_selected_book_mut()
                .unwrap()
                .edit_note(data.reading_state.notes.clone());
            data.reading_state.is_editing_notes = false;
        })
        .disabled_if(|data: &CrabReaderState, _env| data.reading_state.notes.is_empty());

    let bottom_bts = Either::new(
        |data: &CrabReaderState, _env| data.reading_state.is_editing_notes,
        Flex::row()
            .with_flex_child(save_note_btn, 5.0)
            .with_flex_spacer(1.0)
            .with_flex_child(undo_note_btn, 5.0),
        Flex::row()
            .with_flex_child(edit_note_btn, 5.0)
            .with_flex_spacer(1.0)
            .with_flex_child(del_note_btn, 5.0),
    );

    Flex::column()
        .with_flex_child(notes_either, 5.0)
        .with_flex_spacer(1.0)
        .with_flex_child(bottom_bts, 1.0)
        .center()
}

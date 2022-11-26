use druid::{widget::{Label, LineBreaking, TextBox, Either, Flex, List, Container}, Widget, LensExt, WidgetExt, Lens, EventCtx};

use crate::{CrabReaderState, ReadingState, traits::{gui::GUILibrary, note::NoteManagement, reader::BookManagement}, models::note::{BookNotes, Note}, utils::colors, ROUND_FACTR};

use super::{buttons::rbtn::RoundedButton, mockup::LibrarySelectedBookLens, };

pub struct SelectedBookNotesLens;

impl<B: BookManagement> Lens<B, BookNotes> for SelectedBookNotesLens {
    fn with<V, F: FnOnce(&BookNotes) -> V>(&self, data: &B, f: F) -> V {
        f(data.get_notes())
    }

    fn with_mut<V, F: FnOnce(&mut BookNotes) -> V>(&self, data: &mut B, f: F) -> V {
        f(data.get_notes_mut())
    }
}

pub fn get_notes_list() -> impl Widget<CrabReaderState> {
    let notes = List::new( || {
        let header = Label::new(|note: &Note, _env: &_| format!("{}...", note.get_start()[0..10].to_string()))
            .with_text_size(8.0)
            .with_text_color(colors::ON_SECONDARY)
            .with_line_break_mode(LineBreaking::WordWrap)
            .with_text_alignment(druid::TextAlignment::Start)
            .padding(2.0);

        let content = Label::new(|note: &Note, _env: &_| note.get_text().to_string())
            .with_line_break_mode(LineBreaking::WordWrap)
            .with_text_color(colors::ON_SECONDARY)
            .with_text_alignment(druid::TextAlignment::Start)
            .padding(2.0);

        Container::new(
            Flex::column()
                .with_child(header)
                .with_default_spacer()
                .with_child(content)
                .with_default_spacer()
        )
        .expand_width()
        .background(colors::SECONDARY)
        .rounded(ROUND_FACTR)
    })
    .lens(CrabReaderState::library.then(LibrarySelectedBookLens).then(SelectedBookNotesLens));

    notes
}

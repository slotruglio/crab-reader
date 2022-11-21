use super::button_functions::{self, go_next, go_prev};
use crate::{
    components::{
        book::{BookManagement, BookReading, GUIBook},
        library::GUILibrary,
    },
    CrabReaderState, DisplayMode, ENTERING_READING_MODE,
};
use druid::{AppDelegate, Code, Env, Event, Handled};
use std::rc::Rc;

pub struct ReadModeDelegate;

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
                data.reading_state.enable(Rc::new(
                    data.library
                        .get_selected_book()
                        .unwrap()
                        .get_page_of_chapter(),
                ));
                Handled::Yes
            }
            notif if notif.is(ENTERING_READING_MODE) => {
                data.reading = false;
                data.reading_state.disable();
                Handled::Yes
            }
            _ => Handled::No,
        }
    }

    fn event(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        window_id: druid::WindowId,
        event: druid::Event,
        data: &mut CrabReaderState,
        env: &Env,
    ) -> Option<druid::Event> {
        match &event {
            Event::KeyDown(key) => {
                let key = key.code;
                match key {
                    Code::Escape => {
                        handle_esc(ctx, window_id, event, data, env);
                        None
                    }
                    Code::ArrowLeft => {
                        handle_arrow_left(ctx, window_id, event, data, env);
                        None
                    }
                    Code::ArrowRight => {
                        handle_arrow_right(ctx, window_id, event, data, env);
                        None
                    }
                    Code::Tab => {
                        handle_tab(ctx, window_id, event, data, env);
                        None
                    }
                    Code::Enter | Code::NumpadEnter => {
                        handle_enter(ctx, window_id, event, data, env);
                        None
                    }
                    Code::KeyF => {
                        handle_f(ctx, window_id, event, data, env);
                        None
                    }
                    _ => Some(event),
                }
            }
            _ => Some(event),
        }
    }
}

fn handle_arrow_right(
    _ctx: &mut druid::DelegateCtx,
    _window_id: druid::WindowId,
    _event: druid::Event,
    data: &mut CrabReaderState,
    _env: &Env,
) {
    if data.reading_state.is_editing {
        return;
    }

    if data.reading {
        go_next(data);
        return;
    }

    let nbooks = data.library.number_of_books();
    if nbooks == 0 {
        return;
    }

    let Some(idx) = data.library.next_book_idx() else {
        return;
    };

    if let Some(book) = data.library.get_selected_book_mut() {
        book.unselect();
        data.library.unselect_current_book();
    };

    data.library.set_selected_book_idx(idx);
}

fn handle_arrow_left(
    _ctx: &mut druid::DelegateCtx,
    _window_id: druid::WindowId,
    _event: Event,
    data: &mut CrabReaderState,
    _env: &Env,
) {
    if data.reading_state.is_editing {
        return;
    }

    if data.reading {
        go_prev(data);
        return;
    }

    let Some(idx) = data.library.prev_book_idx() else {
        return;
    };

    if let Some(book) = data.library.get_selected_book_mut() {
        book.unselect();
        data.library.unselect_current_book();
    };

    data.library.set_selected_book_idx(idx);
}

fn handle_esc(
    _ctx: &mut druid::DelegateCtx,
    _window_id: druid::WindowId,
    _event: druid::Event,
    data: &mut CrabReaderState,
    _env: &Env,
) {
    if data.reading_state.is_editing {
        data.reading_state.is_editing = false;
        return;
    }

    if data.reading {
        data.reading = false;
        return;
    }

    if let Some(book) = data.library.get_selected_book_mut() {
        book.unselect();
        data.library.unselect_current_book();
        return;
    }
}

fn handle_tab(
    _ctx: &mut druid::DelegateCtx,
    _window_id: druid::WindowId,
    _event: druid::Event,
    data: &mut CrabReaderState,
    _env: &Env,
) {
    if data.reading {
        if data.reading_state.is_editing {
            button_functions::undo_btn_fn(&mut data.reading_state);
        } else {
            button_functions::edit_btn_fn(
                &mut data.reading_state,
                data.library.get_selected_book().unwrap(),
            );
        }
    }

    if data.display_mode == DisplayMode::Cover {
        data.display_mode = DisplayMode::List;
    } else {
        data.display_mode = DisplayMode::Cover;
    }
}

fn handle_enter(
    ctx: &mut druid::DelegateCtx,
    _window_id: druid::WindowId,
    _event: Event,
    data: &mut CrabReaderState,
    _env: &Env,
) {
    if let Some(book) = data.library.get_selected_book_mut() {
        book.load_chapter();
        ctx.submit_command(ENTERING_READING_MODE);
    }
}

fn handle_f(
    _ctx: &mut druid::DelegateCtx,
    _window_id: druid::WindowId,
    _event: Event,
    data: &mut CrabReaderState,
    _env: &Env,
) {
    if let Some(book) = data.library.get_selected_book_mut() {
        let fav = book.is_favorite();
        book.set_favorite(!fav);
    }
}

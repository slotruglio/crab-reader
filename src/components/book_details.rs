use druid::widget::{Button, Container, Flex, Label, LineBreaking};
use druid::{
    ArcStr, BoxConstraints, Color, Env, Event, EventCtx, FontDescriptor, FontFamily, FontWeight,
    LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetExt, WidgetPod,
    WindowDesc,
};
use std::rc::Rc;

use crate::CrabReaderState;

use super::{
    book::{Book, GUIBook},
    library::GUILibrary,
    mockup::MockupLibrary,
};

type Library = MockupLibrary<Book>;

pub struct BookDetails {
    title: WidgetPod<Rc<String>, Label<Rc<String>>>,
    authors: WidgetPod<Rc<String>, Label<Rc<String>>>,
    percent: WidgetPod<f64, Label<f64>>,
    read_btn: WidgetPod<String, Box<dyn Widget<String>>>,
}

impl BookDetails {
    fn title() -> WidgetPod<Rc<String>, Label<Rc<String>>> {
        let mut label = Label::dynamic(|title: &Rc<String>, _| title.to_string())
            .with_text_color(Color::rgb8(0, 0, 0))
            .with_font(FontDescriptor::new(FontFamily::SERIF).with_weight(FontWeight::BOLD))
            .with_text_size(28.0)
            .with_text_alignment(druid::TextAlignment::Center);
        label.set_line_break_mode(LineBreaking::WordWrap);
        WidgetPod::new(label)
    }

    fn authors() -> WidgetPod<Rc<String>, Label<Rc<String>>> {
        let label = Label::dynamic(|authors: &Rc<String>, _| format!("Di {}", authors.to_string()))
            .with_text_color(Color::rgb8(0, 0, 0))
            .with_font(FontDescriptor::new(FontFamily::SERIF).with_weight(FontWeight::NORMAL))
            .with_text_size(14.0)
            .with_text_alignment(druid::TextAlignment::Center);
        WidgetPod::new(label)
    }

    fn read_btn() -> WidgetPod<String, Box<dyn Widget<String>>> {
        let label_text = ArcStr::from("Continua a Leggere");
        let btn = Button::<String>::new(label_text).on_click(
            |ctx: &mut EventCtx, _: &mut String, _: &Env| {
                println!("Continua a leggere");
                let w = WindowDesc::new(|| {
                    Container::new(Flex::<CrabReaderState>::row())
                        .expand()
                        .background(Color::RED)
                });
                ctx.new_window(w);
            },
        );
        WidgetPod::new(Box::new(btn))
    }

    fn percent() -> WidgetPod<f64, Label<f64>> {
        let label = Label::dynamic(|percent: &f64, _| format!("Letto al {}%", percent))
            .with_text_color(Color::rgb8(0, 0, 0))
            .with_font(FontDescriptor::new(FontFamily::SERIF).with_weight(FontWeight::NORMAL))
            .with_text_size(14.0)
            .with_text_alignment(druid::TextAlignment::Center);
        WidgetPod::new(label)
    }

    pub fn new() -> Self {
        let title = Self::title();
        let authors = Self::authors();
        let read_btn = Self::read_btn();
        let percent = Self::percent();

        Self {
            title,
            authors,
            read_btn,
            percent,
        }
    }

    fn handle_widget_added(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {}
            _ => {
                return;
            }
        };

        let placeholder = Rc::from(String::from(""));
        self.title.lifecycle(ctx, event, &placeholder, env);
        self.authors.lifecycle(ctx, event, &placeholder, env);
        self.read_btn
            .lifecycle(ctx, event, &"Read".to_string(), env);
        self.percent.lifecycle(ctx, event, &0.0, env);
    }
}

impl Widget<Library> for BookDetails {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Library, env: &Env) {
        if let Some(book) = data.get_selected_book_mut() {
            self.title.event(ctx, event, &mut book.get_title(), env);
            self.authors.event(ctx, event, &mut book.get_author(), env);
            self.read_btn
                .event(ctx, event, &mut "Read".to_string(), env);
            self.percent.event(ctx, event, &mut 0.0, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Library, env: &Env) {
        self.handle_widget_added(ctx, event, env);

        if let Some(book) = data.get_selected_book() {
            self.title.lifecycle(ctx, event, &book.get_title(), env);
            self.authors.lifecycle(ctx, event, &book.get_author(), env);
            self.read_btn
                .lifecycle(ctx, event, &"Read".to_string(), env);
            self.percent.lifecycle(ctx, event, &0.0, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Library, data: &Library, env: &Env) {
        if let Some(book) = data.get_selected_book() {
            let title = book.get_title();
            let authors = book.get_author();
            if let Some(old_book) = old_data.get_selected_book() {
                if old_book != book {
                    self.title.update(ctx, &title, env);
                    self.authors.update(ctx, &authors, env);
                    self.percent.update(ctx, &40.0, env);
                }
            } else {
                self.title.update(ctx, &title, env);
                self.authors.update(ctx, &authors, env);
                self.percent.update(ctx, &10.0, env);
            }
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Library,
        env: &Env,
    ) -> Size {
        if let Some(book) = data.get_selected_book() {
            let mut hh = 0.0; // Total layout heigth
            let ww = bc.max().width;
            let bc = bc.shrink((10.0, 0.0)); // ???

            let title = book.get_title();
            let title_size = self.title.layout(ctx, &bc, &title, env);
            self.title.set_origin(ctx, &title, env, (5.0, hh).into());
            hh += title_size.height - 5.0;

            let author = book.get_author();
            let authors_size = self.authors.layout(ctx, &bc, &author, env);
            self.authors.set_origin(ctx, &author, env, (5.0, hh).into());
            hh += authors_size.height;
            hh += 10.0;

            let percent = 50.0;
            let percent_size = self.percent.layout(ctx, &bc, &percent, env);
            self.percent
                .set_origin(ctx, &percent, env, (5.0, hh).into());
            hh += percent_size.height;

            let null = String::from("");
            let read_btn_size = self.read_btn.layout(ctx, &bc, &null, env);
            self.read_btn.set_origin(ctx, &null, env, (5.0, hh).into());
            hh += read_btn_size.height;
            hh += 10.0;

            return (ww, hh).into();
        }
        (0.0, 0.0).into()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Library, env: &Env) {
        if let Some(book) = data.get_selected_book() {
            self.title.paint(ctx, &book.get_title(), env);
            self.authors.paint(ctx, &book.get_author(), env);
            self.read_btn.paint(ctx, &"Read".to_string(), env);
            self.percent.paint(ctx, &30.0, env);
        }
    }
}

use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WidgetExt, Color, Data, Lens};
use druid::widget::{Flex, Container, TextBox};


#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    surname: String
}

fn build_ui() -> impl Widget<HelloState> {

    Flex::column()
        .with_flex_child(
            TextBox::multiline()
                .with_placeholder("Who are we greeting?")
                .fix_height(400.0)
                .fix_width(400.0)
                .lens(HelloState::name),
            50.0
        )
        .with_flex_child(
            Container::new(
                TextBox::multiline()
                .with_placeholder("Who are we greeting?")
                .fix_height(200.0)
                .fix_width(200.0)
                .lens(HelloState::surname).background(Color::RED),
            ),
            50.0
        )
}

fn main() -> Result<(), PlatformError> {
    
    AppLauncher::with_window(WindowDesc::new(|| build_ui())).launch(HelloState { name: "test".into(), surname:"losbruglio".into() })?;
    Ok(())
}
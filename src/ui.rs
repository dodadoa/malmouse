use druid::widget::{Button, Flex, Label, Slider};
use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc, Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub value: f64,
}


pub fn ui_builder() -> impl Widget<AppState> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("MOUSE Gauge");
    let label = Label::new(text).padding(5.0).center();
    let bar = Slider::new()
        .with_range(0.0, 20.0)
        .with_step(1.0)
        .lens(AppState::value);

    Flex::column().with_child(label).with_default_spacer().with_child(bar)
}

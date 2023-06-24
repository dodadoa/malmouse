use druid::widget::{Flex, Label};
use druid::{LocalizedString, Widget, WidgetExt, Data, Lens};
use std::sync::{Arc, Mutex};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub data_outside: Arc<Mutex<f64>>
}

pub fn ui_builder() -> impl Widget<AppState> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("MOUSE Gauge").with_arg("count", |data: &AppState, _env| {
            let v = data.data_outside.lock().unwrap();
            println!("data_outside: {:?}", *v);
            (*v).to_string().into()
        });
    let label = Label::new(text).padding(5.0).center();

    Flex::column().with_child(label).with_default_spacer()
}

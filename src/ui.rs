use druid::widget::prelude::*;
use druid::widget::{Flex, Label};
use druid::{Data, Lens, Widget, WidgetExt, LocalizedString};
use std::sync::{Arc, Mutex};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub data_outside: Arc<Mutex<f64>>,
}

struct UpdatedLabelWidget {}

impl Widget<AppState> for UpdatedLabelWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        let max_size = bc.max();
        let min_side = max_size.height.min(max_size.width);
        Size {
            width: min_side,
            height: min_side,
        }
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &AppState, _env: &Env) {}
}

pub fn ui_builder() -> impl Widget<AppState> {

    let text = LocalizedString::new("Gate")
        .with_arg("gate", |data: &AppState, _env: _| {
            let v = data.data_outside.lock().unwrap();
            format!("{:.2}", *v).into()
        });
    let label = Label::new(text).padding(5.0).center();

    Flex::column().with_child(label).with_default_spacer()
}

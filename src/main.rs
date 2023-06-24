pub mod ui;

use rdev::{listen, Event, simulate, EventType, SimulateError};
use std::{thread, time};
use druid::widget::{Button, Flex, Label, Slider};
use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc, Data, Lens};


fn callback(event: Event) {
    println!("My callback {:?}", event);
    match event.event_type {
        EventType::MouseMove { x, y } => {
            println!("User wrote {:?} {:?}", x, y);
            // send(&EventType::MouseMove { x: 10.0, y: 200.0 });
        },
        EventType::KeyPress(key) => {
            println!("User wrote {:?}", key);
        },
        EventType::KeyRelease(key) => {
            println!("User wrote {:?}", key);
        },
        EventType::ButtonPress(_) => {
        },
        EventType::ButtonRelease(_) => {
        },
        EventType::Wheel { delta_x, delta_y } => {
        },
    }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => println!("We could send {:?}", event_type),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

fn main() {
    // create the initial app state
    let initial_state: ui::AppState = ui::AppState {
        value: 0.0,
    };

    thread::spawn(move || {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });

    let main_window = WindowDesc::new(ui::ui_builder());

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
    
}

pub mod ui;

use druid::{AppLauncher, WindowDesc};
use rdev::{listen, simulate, EventType, SimulateError};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::{thread, time};

fn _send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => println!("We could send {:?}", event_type),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    
    // Let ths OS catchup - at least MacOS
    thread::sleep(delay);
}

fn main() {
    
    let (schan, rchan) = channel();
    let data_outside = Arc::new(Mutex::new(0.0));
    let data_outside_clone = data_outside.clone();

    thread::spawn(move || {
        for event in rchan.iter() {
            println!("Received! {:?}", event);
            let mut data = data_outside_clone.lock().unwrap();
            *data += 0.1;
        }
    });

    let initial_state: ui::AppState = ui::AppState { 
        data_outside,
        data_inside: 0.0
    };

    thread::spawn(move || {
        listen(move |event| {
            match event.event_type {
                EventType::MouseMove { x: _x, y: _y } => {
                    schan
                        .send(event)
                        .unwrap_or_else(|e| println!("Could not send event {:?}", e));
                    // send(&EventType::MouseMove { x: 10.0, y: 200.0 });
                }
                EventType::KeyPress(_) => {}
                EventType::KeyRelease(_) => {}
                EventType::ButtonPress(_) => {}
                EventType::ButtonRelease(_) => {}
                EventType::Wheel { delta_x: _, delta_y: _ } => {}
            }
        })
        .expect("Could not listen");
    });

    let main_window = WindowDesc::new(ui::ui_builder());

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

pub mod ui;

use druid::{AppLauncher, WindowDesc};
use rdev::{listen, simulate, EventType, SimulateError};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::{thread, time};

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        // Ok(()) => println!("We could send {:?}", event_type),
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    
    // Let ths OS catchup - at least MacOS
    thread::sleep(delay);
}

fn main() {
    
    let (schan, rchan) = channel();
    let mouse_data = Arc::new(Mutex::new(0.0));
    let data_outside_clone = mouse_data.clone();

    thread::spawn(move || {
        listen(move |event| {
            match event.event_type {
                EventType::MouseMove { x: _x, y: _y } => {
                    let data = data_outside_clone.lock().unwrap();
                    if *data < 5.0 {
                        send(&EventType::MouseMove { x: 10.0, y: 200.0 });
                    } else {
                        println!("CAN USE MOUSE NOW");
                        schan
                            .send(event)
                            .unwrap_or_else(|e| println!("Could not send event {:?}", e));
                    }
                }
                EventType::KeyPress(_) => {}
                EventType::KeyRelease(_) => {}
                EventType::ButtonPress(_) => {
                    schan
                        .send(event)
                        .unwrap_or_else(|e| println!("Could not send event {:?}", e));
                }
                EventType::ButtonRelease(_) => {}
                EventType::Wheel { delta_x: _, delta_y: _ } => {}
            }
        })
        .expect("Could not listen");
    });

    let mouse_data_clone = mouse_data.clone();
    // thread::spawn(move || {
        for _event in rchan.iter() {
            let mut data = mouse_data_clone.lock().unwrap();
            *data += 0.1;
            if *data > 50.0 {
                *data = 0.0;
                println!("END")
            }

            println!("{} ", *data)
        }
    // });

    // let initial_state: ui::AppState = ui::AppState { 
    //     data_outside,
    // };
    // let main_window = WindowDesc::new(ui::ui_builder());

    // AppLauncher::with_window(main_window)
    //     .log_to_console()
    //     .launch(initial_state)
    //     .expect("Failed to launch application");
}

use rdev::{listen, Event, simulate, EventType, SimulateError};
use std::{thread, time};


fn callback(event: Event) {
    println!("My callback {:?}", event);
    match event.name {
        Some(string) => {
            println!("User wrote {:?}", string);
            
        },
        None => send(&EventType::MouseMove { x: 10.0, y: 200.0 }),
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
    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

// Simulate mouse move and keyboard
use enigo::{Enigo, MouseControllable};
//listen events globally
use rdev::{listen, Event, EventType, Key, SimulateError};

use std::{
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
    sync::Arc,
    thread, time,
};

fn main() {
    println!("Press F1 key to activate/deactivate cursor lock ");

    // new three thread , each for activation state ,
    // mouse_x and mouse_y
    // And instatiated them to (0,0)
    let is_activated = Arc::new(AtomicBool::new(false));
    let is_activated_thread = Arc::clone(&is_activated);
    let mouse_x = Arc::new(AtomicI32::new(0));
    let mouse_x_thread = Arc::clone(&mouse_x);
    let mouse_y = Arc::new(AtomicI32::new(0));
    let mouse_y_thread = Arc::clone(&mouse_x);
    thread::spawn(move || {
        let mut enigo = Enigo::new();
        // For each pair of (x,y) , we listen to the event
        // And record the coordinate of it
        loop {
            if is_activated_thread.load(Ordering::Relaxed) {
                enigo.mouse_move_to(
                    mouse_x_thread.load(Ordering::Relaxed),
                    mouse_y_thread.load(Ordering::Relaxed),
                );
                thread::sleep(time::Duration::from_millis(3));
            }
        }
    });

    // match the listen event
    if let Err(error) = listen(move |hitKey| {
        // When user hits the key , judge the current status
        // of the cursor and change it
        if hitKey.event_type == EventType::KeyPress(Key::F1) {
            // change the value of is_activated from false to true
            is_activated.store(!is_activated.load(Ordering::Relaxed), Ordering::Relaxed);
            if is_activated.load(Ordering::Relaxed) {
                println!("Cursor lock  activated ");
            } else {
                println!("Cursor lock deactivated ");
            }

            // After the activation thread
            // we store the value and memory ordering to mouse
            let cursor_location: (i32, i32) = Enigo::mouse_location();

            mouse_x.store(cursor_location.0, Ordering::Relaxed);
            mouse_y.store(cursor_location.1, Ordering::Relaxed);
        }
    }) {
        println!("Error:{:?}", error)
    }
}

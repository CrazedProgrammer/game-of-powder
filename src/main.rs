extern crate sdl2;
mod types;
mod ui;

use sdl2::event::Event;
use std::{thread, time};
use std::boxed::Box;
use std::sync::{Arc, Mutex};
use types::{Flags, Playfield};

const playfield_width: i32 = 4096;
const playfield_height: i32 = 4096;


fn main() {
    //let playfield = Arc::new(Mutex::new());
    let flags = Arc::new(Mutex::new(Flags{ running: true }));

    {
        //let playfield = playfield.clone();
        let flags = flags.clone();
        let child = thread::spawn(move || {
            ui::run(flags);
        });
    }

    loop {
        thread::sleep(time::Duration::from_millis(100));
        let flags = flags.clone();
        if !flags.lock().unwrap().running {
            break;
        }
    }
}

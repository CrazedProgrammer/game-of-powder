extern crate sdl2;
extern crate time;

mod types;
mod ui;

use std::thread;
use std::boxed::Box;
use std::sync::{Arc, RwLock};
use types::{UISync, UIEvent, Playfield};

const PLAYFIELD_WIDTH: i32 = 300;
const PLAYFIELD_HEIGHT: i32 = 200;


fn main() {
    let playfield = Arc::new(RwLock::new(Playfield::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT)));
    let uisync = Arc::new(RwLock::new(UISync{ running: true, events: vec![] }));

    {
        let playfield = playfield.clone();
        let uisync = uisync.clone();
        let child = thread::spawn(move || {
            ui::run(uisync, playfield);
        });
    }

    let uisync = uisync.clone();
    let playfield_local = playfield.clone();
    loop {
        let running;
        {
            let mut uisync = uisync.write().unwrap();
            for event in &uisync.events {
                match *event {
                    UIEvent::SpawnBlock {x, y, block} => {
                        let mut playfield = playfield_local.write().unwrap();
                        playfield.write_nowrap(x, y, block);
                    },
                }
            }
            uisync.events.clear();
            running = uisync.running;
        }
        if running {
            thread::sleep(std::time::Duration::from_millis(10));
        } else {
            break;
        }
    }
}

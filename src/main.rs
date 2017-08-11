extern crate sdl2;
extern crate time;

mod types;
mod ui;
mod sim;

use std::thread;
use std::sync::{Arc, RwLock};
use types::{UISync, Playfield, PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT};



fn main() {
    let playfield = Arc::new(RwLock::new(Playfield::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT)));
    let uisync = Arc::new(RwLock::new(UISync{ running: true, events: vec![] }));

    {
        let playfield = playfield.clone();
        let uisync = uisync.clone();
        thread::spawn(move || {
            ui::run(uisync, playfield);
        });
    }

    let uisync = uisync.clone();
    let playfield_local = playfield.clone();
    sim::main_loop(playfield_local, uisync);
}

extern crate sdl2;

use sdl2::event::Event;
use std::sync::{Arc, Mutex};
use types::Flags;


pub fn run(flags: Arc<Mutex<Flags>>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game of Powder", 800, 600)
        .position_centered()
        .resizable()
        .build().unwrap();

    let mut renderer = window.into_canvas().present_vsync().build().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'event_loop,
                _ => {}
            }
        }
    }

    {
        flags.lock().unwrap().running = false;
    }
}

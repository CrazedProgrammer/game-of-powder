extern crate sdl2;

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use sdl2::rect::{Point, Rect};
use sdl2::event::{Event, WindowEvent};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use types::{Block, Viewport, FPoint, Playfield, UISync, UIEvent};


pub fn run(uisync: Arc<RwLock<UISync>>, playfield: Arc<RwLock<Playfield>>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game of Powder", 800, 600)
        .position_centered()
        .resizable()
        .build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut viewport = calculate_viewport(Viewport { x: 400.0, y: 300.0, width: 0.0, height: 0.0, zoom: 10.0 }, &canvas);
    let block_colors = [
        (Block::Empty, Color::RGB(0, 0, 0)),
        (Block::Full, Color::RGB(255, 255, 255))
    ].iter().cloned().collect();


    'event_loop: loop {
        {
            let mut uisync = uisync.write().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'event_loop,

                    Event::Window {win_event, ..} => {
                        match win_event {
                            WindowEvent::Resized {..} => {
                                viewport = calculate_viewport(viewport, &canvas);
                            }
                            _ => {},
                        }
                    },


                    Event::KeyDown {keycode: Some(keycode), ..} => {
                        match keycode {
                            Keycode::Escape => {break 'event_loop},
                            _ => {},
                        }
                    },

                    Event::MouseButtonDown {x, y, mouse_btn, ..} => {
                        let block_point = window_to_viewport(x, y, viewport);
                        uisync.events.push(UIEvent::SpawnBlock {x: block_point.x, y: block_point.y, block: Block::Full});
                        println!("epic dude {} {}", block_point.x, block_point.y);
                    },

                    Event::MouseMotion {x, y, ..} => {
                        let block_point = window_to_viewport(x, y, viewport);
                        uisync.events.push(UIEvent::SpawnBlock {x: block_point.x, y: block_point.y, block: Block::Full});
                    },

                    _ => {},
                }
            }
        }
        draw(&mut canvas, &playfield.read().unwrap(), viewport, &block_colors);
    }

    {
        uisync.write().unwrap().running = false;
    }
}

fn draw(canvas: &mut Canvas<Window>, playfield: &Playfield, viewport: Viewport, block_colors: &HashMap<Block, Color>){
    canvas.clear();

    for j in 0..((viewport.height + 1.0) as u32) {
        for i in 0..((viewport.width + 1.0) as u32) {
            let block_viewport_x = i as i32 + (viewport.x - viewport.width / 2.0) as i32;
            let block_viewport_y = j as i32 + (viewport.y - viewport.height / 2.0) as i32;
            let block = playfield.read_nowrap(block_viewport_x, block_viewport_y);
            let block_color = block_colors[&block];
            let block_window_point = viewport_to_window(block_viewport_x, block_viewport_y, viewport);
            let block_window_point_next = viewport_to_window(block_viewport_x + 1, block_viewport_y + 1, viewport);
            let block_window_width = (block_window_point_next.x - block_window_point.x) as u32;
            let block_window_height = (block_window_point_next.y - block_window_point.y) as u32;
            canvas.set_draw_color(block_color);
            canvas.fill_rect(Rect::new(block_window_point.x, block_window_point.y, block_window_width, block_window_height));
            if block == Block::Full {
                //println!("x: {}, y: {}", block_window_point.x, block_window_point.y);
            }
        }
    }

    canvas.present();
}

fn calculate_viewport(viewport: Viewport, canvas: &Canvas<Window>) -> Viewport {
    let window_size = canvas.window().size();
    Viewport {
        x: viewport.x,
        y: viewport.y,
        width: (window_size.0 as f32) / viewport.zoom,
        height: (window_size.1 as f32) / viewport.zoom,
        zoom: viewport.zoom
    }
}

fn viewport_to_window(x: i32, y: i32, viewport: Viewport) -> Point {
    let block_x = ((((x as f32) - viewport.x) + viewport.width / 2.0) * viewport.zoom) as i32;
    let block_y = ((((y as f32) - viewport.y) + viewport.height / 2.0) * viewport.zoom) as i32;

    Point::new(block_x, block_y)
}

fn window_to_viewport(x: i32, y: i32, viewport: Viewport) -> Point {
    let block_x = (viewport.x + (x as f32) / viewport.zoom - viewport.width / 2.0) as i32;
    let block_y = (viewport.y + (y as f32) / viewport.zoom - viewport.height / 2.0) as i32;
    Point::new(block_x, block_y)
}


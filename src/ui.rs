extern crate sdl2;
extern crate time;

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use sdl2::rect::{Point, Rect};
use sdl2::event::{Event, WindowEvent};
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::keyboard::Keycode;
use types::{Block, Viewport, Playfield, UISync, UIEvent, PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT};



pub fn run(uisync: Arc<RwLock<UISync>>, playfield: Arc<RwLock<Playfield>>) {
    let block_colors = [
        (Block::Empty, Color::RGB(0, 0, 0)),
        (Block::Full, Color::RGB(255, 255, 255))
    ].iter().cloned().collect();


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game of Powder", 800, 600)
        .position_centered()
        .resizable()
        .build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut viewport = Viewport { x: PLAYFIELD_WIDTH as f32 / 2.0, y: PLAYFIELD_HEIGHT as f32 / 2.0, width: 0.0, height: 0.0, zoom: 1.0 };
    recalculate_viewport(&mut viewport, &canvas);
    let mut render_target;
    {
        let window_size = canvas.window().size();
        render_target = texture_creator
            .create_texture_target(PixelFormatEnum::RGB888, window_size.0, window_size.1)
            .unwrap();
    }
    let mut prev_nano_time = time::precise_time_ns();
    let mut frame_counter = 0;

    let mut draw_full = true;
    let mut prev_playfield = Playfield::new(0, 0); // this is not possibly uninitialised.


    'event_loop: loop {
        {
            let mut uisync = uisync.write().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'event_loop,

                    Event::Window {win_event, ..} => {
                        match win_event {
                            WindowEvent::Resized {..} => {
                                let window_size = canvas.window().size();
                                render_target = texture_creator
                                    .create_texture_target(PixelFormatEnum::RGB888, window_size.0, window_size.1)
                                    .unwrap();
                                recalculate_viewport(&mut viewport, &canvas);
                                draw_full = true;
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

                    Event::MouseMotion {x, y, ..} => {
                        let block_point = window_to_viewport(x, y, viewport);
                        uisync.events.push(UIEvent::SpawnBlock {x: block_point.x, y: block_point.y, block: Block::Full});
                    },

                    Event::MouseWheel {..} => {
                        viewport.zoom = viewport.zoom % 10.0 + 1.0;
                        draw_full = true;
                        recalculate_viewport(&mut viewport, &canvas);
                    }


                    _ => {},
                }
            }
        }
        let playfield = playfield.read().unwrap();
        canvas.with_texture_canvas(&mut render_target, |mut texture_canvas| {
            draw(&playfield, &prev_playfield, &mut texture_canvas, viewport, &block_colors, draw_full);
        }).unwrap();
        {
            let window_size = canvas.window().size();
            canvas.copy_ex(&render_target, None, Rect::new(0, 0, window_size.0, window_size.1), 0.0, Some(Point::new(window_size.0 as i32, window_size.1 as i32)), false, false).unwrap();
        }
        if draw_full {
            draw_full = false;
        }
        prev_playfield = playfield.clone();

        frame_counter += 1;
        let nano_time = time::precise_time_ns();
        if (nano_time - prev_nano_time) >= 1000000000u64 {
            println!("FPS: {} Viewport width: {} height: {}", frame_counter, viewport.width, viewport.height);
            frame_counter = 0;
            prev_nano_time = nano_time;
        }
    }

    uisync.write().unwrap().running = false;
}

fn recalculate_viewport(viewport: &mut Viewport, canvas: &Canvas<Window>)
{
    let window_size = canvas.window().size();
    viewport.width = (window_size.0 as f32) / viewport.zoom;
    viewport.height = (window_size.1 as f32) / viewport.zoom;
}


fn draw(playfield: &Playfield, prev_playfield: &Playfield, canvas: &mut Canvas<Window>, viewport: Viewport, block_colors: &HashMap<Block, Color>, draw_full: bool) {
    let mut current_color = Color::RGB(234, 92, 193);
    if draw_full {
        current_color = block_colors[&Block::Full];
        canvas.set_draw_color(current_color);
        canvas.clear();
    }

    for j in 0..((viewport.height + 2.0) as u32) {
        for i in 0..((viewport.width + 2.0) as u32) {
            let block_viewport_x = i as i32 + (viewport.x - viewport.width / 2.0) as i32;
            let block_viewport_y = j as i32 + (viewport.y - viewport.height / 2.0) as i32;
            let block = playfield.read_nowrap(block_viewport_x, block_viewport_y);
            if !draw_full {
                if block == prev_playfield.read_nowrap(block_viewport_x, block_viewport_y) {
                    continue;
                }
            }
            let block_window_point = viewport_to_window(block_viewport_x, block_viewport_y, viewport);
            let block_window_point_next = viewport_to_window(block_viewport_x + 1, block_viewport_y + 1, viewport);
            let block_window_width = (block_window_point_next.x - block_window_point.x) as u32;
            let block_window_height = (block_window_point_next.y - block_window_point.y) as u32;
            let block_color = block_colors[&block];
            if block_color != current_color {
                canvas.set_draw_color(block_color);
                current_color = block_color;
            }
            canvas.fill_rect(Rect::new(block_window_point.x, block_window_point.y, block_window_width, block_window_height)).unwrap();
        }
    }

    canvas.present();
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


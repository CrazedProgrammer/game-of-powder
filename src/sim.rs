use std::thread;
use std::time;
use std::sync::{Arc, RwLock};
use types::{Block, Playfield, UISync, UIEvent, PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT};

pub fn main_loop(playfield: Arc<RwLock<Playfield>>, uisync: Arc<RwLock<UISync>>)
{
    let mut next_playfield = Playfield::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT);
    loop {
        let running;
        {
            {
                let mut playfield = playfield.write().unwrap();
                let mut uisync = uisync.write().unwrap();
                for event in &uisync.events {
                    match *event {
                        UIEvent::SpawnBlock {x, y, block} => {
                            playfield.write_nowrap(x, y, block);
                        },
                    }
                }
                uisync.events.clear();
            }
            {
                let playfield = playfield.read().unwrap();
                for j in 0..playfield.height {
                    for i in 0..playfield.width {
                        next_playfield.write(i as i32, j as i32, simulate_block(&playfield, i as i32, j as i32));
                    }
                }
            }
            {
                let mut playfield = playfield.write().unwrap();
                for j in 0..playfield.height {
                    for i in 0..playfield.width {
                        playfield.write(i as i32, j as i32, next_playfield.read(i as i32, j as i32));
                    }
                }
            }
            running = uisync.read().unwrap().running;
        }
        if running {
            thread::sleep(time::Duration::from_millis(10));
        } else {
            break;
        }
    }
}

#[inline]
fn simulate_block(playfield: &Playfield, x: i32, y: i32) -> Block {
    if playfield.read(x, y) == Block::Full {
        match count_neighbors(playfield, x, y) {
            0|1 => { Block::Empty }, // underpopulation
            2|3 => { Block::Full }, // lives on
            _ => { Block::Empty }, // overpopulation
        }
    } else {
        if count_neighbors(playfield, x, y) == 3 {
            Block::Full // birth
        } else {
            Block::Empty // stays dead
        }
    }
}

#[inline]
fn count_neighbors(playfield: &Playfield, x: i32, y: i32) -> u32 {
    let mut count = 0u32;
    if playfield.read_nowrap(x - 1, y - 1) == Block::Full {
        count += 1;
    }
    if playfield.read_nowrap(x, y - 1) == Block::Full {
        count += 1;
    }
    if playfield.read_nowrap(x + 1, y - 1) == Block::Full {
        count += 1;
    }
    if playfield.read_nowrap(x - 1, y) == Block::Full {
        count += 1;
    }
    if playfield.read_nowrap(x + 1, y) == Block::Full {
        count += 1;
    }
    if playfield.read_nowrap(x - 1, y + 1) == Block::Full {
        count += 1;
    }
    if playfield.read_nowrap(x, y + 1) == Block::Full {
        count += 1;
    }
    if playfield.read_nowrap(x + 1, y + 1) == Block::Full {
        count += 1;
    }
    count
}

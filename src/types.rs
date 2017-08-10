pub struct UISync {
    pub running: bool,
    pub events: Vec<UIEvent>,
}

pub enum UIEvent {
    SpawnBlock {x: i32, y: i32, block: Block},
}



pub struct Playfield {
    pub width: i32,
    pub height: i32,
    pub data: Vec<Block>,
}

impl Playfield {
    pub fn new(width: i32, height: i32) -> Playfield {
        Playfield {
            width: width,
            height: height,
            data: vec![Block::Empty; (width * height) as usize],
        }
    }

    fn inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn write(&mut self, x: i32, y: i32, block: Block) {
        let x = x % self.width;
        let y = y % self.height;
        self.data[(x + y * self.width) as usize] = block;
    }

    pub fn write_nowrap(&mut self, x: i32, y: i32, block: Block) {
        if self.inside(x, y) {
            self.data[(x + y * self.width) as usize] = block;
        }
    }

    pub fn read(&self, x: i32, y: i32) -> Block {
        let x = x % self.width;
        let y = y % self.height;
        self.data[(x + y * self.width) as usize].clone()
    }

    pub fn read_nowrap(&self, x: i32, y: i32) -> Block {
        if self.inside(x, y) {
            self.data[(x + y * self.width) as usize].clone()
        } else {
            Block::Empty
        }
    }
}

#[derive(Clone, Copy)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub zoom: f32,
}

#[derive(Clone, Copy)]
pub struct FPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Block {
    Empty,
    Full,
}

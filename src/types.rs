pub struct UISync {
    pub running: bool,
    pub exit: bool,
    pub events: Vec<UIEvent>,
}

pub enum UIEvent {
    SpawnBlock {x: i32, y: i32, block: Block},
}


#[derive(Clone)]
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

    #[inline]
    pub fn write(&mut self, x: i32, y: i32, block: Block) {
        self.data[(x + y * self.width) as usize] = block;
    }

    pub fn write_nowrap(&mut self, x: i32, y: i32, block: Block) {
        if self.inside(x, y) {
            self.write(x, y, block);
        }
    }

    #[inline]
    pub fn read(&self, x: i32, y: i32) -> Block {
        self.data[(x + y * self.width) as usize].clone()
    }

    #[inline]
    pub fn read_wrap(&self, x: i32, y: i32) -> Block {
        let mut x = x % self.width;
        let mut y = y % self.height;
        while x < 0 {
            x += self.width;
        }
        while y < 0 {
            y += self.height;
        }
        self.read(x, y)
    }

    #[inline]
    pub fn read_nowrap(&self, x: i32, y: i32) -> Block {
        if self.inside(x, y) {
            self.read(x, y)
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

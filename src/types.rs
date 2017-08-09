pub struct Flags {
    pub running: bool
}

pub struct Playfield {
    width: i32,
    height: i32,
    data: Vec<Block>
}

struct Camera {
    x: f32,
    y: f32,
    zoom: f32
}

#[derive(Clone)]
pub enum Block {
    Empty,
    Full
}

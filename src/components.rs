#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Location {
    pub fn new(x: f32, y: f32) -> Location {
        Location { x, y }
    }
}

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

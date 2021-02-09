#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    x: f32,
    y: f32,
}

impl Location {
    pub fn new(x: f32, y: f32) -> Location {
        Location { x, y }
    }
}

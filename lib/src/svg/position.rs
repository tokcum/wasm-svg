use crate::geometry::Point;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
    
    pub fn x(&self) -> i32 {
        self.x
    }
    
    pub fn y(&self) -> i32 {
        self.y
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

impl From<Point> for Position {
    fn from(p: Point) -> Self {
        Position{x: p.x(), y: p.y()}
    }
}

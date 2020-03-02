use crate::geometry::Point;

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
    
    pub fn x(&self) -> i32 {
        self.x
    }
    
    pub fn y(&self) -> i32 {
        self.y
    }
}

impl Default for Pos {
    fn default() -> Self {
        Pos { x: 0, y: 0 }
    }
}

impl From<Point> for Pos {
    fn from(p: Point) -> Self {
        Pos{x: p.x(), y: p.y()}
    }
}

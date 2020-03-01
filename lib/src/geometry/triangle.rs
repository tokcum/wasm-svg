use crate::geometry::line::Line;
use crate::geometry::point::Point;

#[derive(Debug)]
pub struct Triangle {
    b: Line,
    h: Line,
}

impl Triangle {
    pub fn new() -> Self {
        Triangle {
            b: Line::create(Point { x: -2, y: 0 }, Point { x: 2, y: 0 }),
            h: Line::create(Point { x: 0, y: 0 }, Point { x: 0, y: 6 }),
        }
    }

    pub fn create(b: Line, h: Line) -> Self {
        Triangle { b, h }
    }
}

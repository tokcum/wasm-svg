use std::ops::Mul;

use crate::geometry::line::Line;
use crate::geometry::point::Point;

#[derive(Debug, PartialEq)]
pub struct Triangle {
    b: Line,
    h: Line,
}

impl Triangle {
    pub fn new(b: Line, h: Line) -> Self {
        Triangle { b, h }
    }

    pub fn edges(&self) -> (Point, Point, Point) {
        (self.b.start(), self.h.end(), self.b.end())
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Triangle::new(
            Line::new(Point::new(-1, 0), Point::new(1, 0)),
            Line::new(Point::new(0, 0), Point::new(0, 1)),
        )
    }
}

impl Mul<i32> for Triangle {
    type Output = Triangle;

    fn mul(mut self, rhs: i32) -> Self::Output {
        self.b = self.b * rhs;
        self.h = self.h * rhs;
        self
    }
}

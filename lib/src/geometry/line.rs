use std::ops::Mul;

use crate::geometry::point::Point;

#[derive(Debug, PartialEq)]
pub struct Line {
    p1: Point,
    p2: Point,
    t: LineType,
}

#[derive(Debug, PartialEq)]
enum LineType {
    // ToDo: LineType::Full unimplemented
    #[allow(dead_code)]
    Full,
    // ToDo: LineType::Half unimplemented
    #[allow(dead_code)]
    Half,
    Segment,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Line {
            p1,
            p2,
            t: LineType::Segment,
        }
    }

    pub fn start(&self) -> Point {
        self.p1
    }

    pub fn end(&self) -> Point {
        self.p2
    }
}

impl Default for Line {
    fn default() -> Self {
        Line {
            p1: Point::new(0, 0),
            p2: Point::new(1, 1),
            t: LineType::Segment,
        }
    }
}

impl Mul<i32> for Line {
    type Output = Line;

    fn mul(mut self, rhs: i32) -> Self::Output {
        self.p1 = self.p1 * rhs;
        self.p2 = self.p2 * rhs;
        self
    }
}

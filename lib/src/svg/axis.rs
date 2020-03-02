use crate::geometry::Line as GeoLine;
use crate::geometry::{Point, Triangle};
use crate::svg::class::Class;
use crate::svg::{Line, SvgElement};
use crate::svg::Polygon;

#[derive(Debug)]
pub struct Axis {
    head: Option<Polygon>,
    line: Line,
}

impl Axis {
    pub fn head(&self) -> Option<&Polygon> {
        self.head.as_ref()
    }
    
    pub fn line(&self) -> Line {
        self.line
    }
}

impl Default for Axis {
    fn default() -> Self {
        let head = Polygon::from(Triangle::new(
            GeoLine::new(Point::new(0, -2), Point::new(0, 2)),
            GeoLine::new(Point::new(0, 0), Point::new(3, 0)),
        ));
        let line = Line::from(GeoLine::new(Point::new(-1, 0), Point::new(400, 0)));

        Axis {
            head: Some(head),
            line: line,
        }
    }
}

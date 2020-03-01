use crate::geometry::point::Point;

#[derive(Debug)]
pub struct Line {
  p1: Point,
  p2: Point,
  t: LineType,
}

#[derive(Debug)]
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
  pub fn new() -> Self {
    Line{ p1: Point{x: 0, y: 0},  p2: Point{x: 1, y: 1}, t: LineType::Segment }
  }
  
  pub fn create(p1: Point, p2: Point) -> Self {
    Line{ p1, p2, t: LineType::Segment }
  }
}

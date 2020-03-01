use std::ops::Mul;

#[derive(Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[must_use = "Do not forget to make use of your point ;)"]
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(mut self, rhs: i32) -> Self::Output {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    min: i32,
    max: i32,
}

impl Interval {
    pub fn new(min: i32, max: i32) -> Self {
        Interval { min, max }
    }

    pub fn min(&self) -> i32 {
        self.min
    }

    pub fn max(&self) -> i32 {
        self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval { min: 0, max: 1 }
    }
}

#[derive(Debug)]
pub enum Data {
    VecI32(Vec<i32>),
}

impl Data {
    pub fn new(d: Data) -> Self {
        match d {
            Data::VecI32(d) => {
                let mut v: Vec<i32> = Vec::new();
                v.push(-1);
                v.push(0);
                v.push(1);

                Data::VecI32(v)
            }
        }
    }
    pub fn unwrap(self) -> Vec<i32> {
        match self {
            Data::VecI32(d) => d,
        }
    }
}

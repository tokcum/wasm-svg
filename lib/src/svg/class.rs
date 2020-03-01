#[derive(Debug)]
pub enum Class {
    Triangle,
}

impl Class {
    pub fn to_string(&self) -> String {
        match self {
            Triangle => format!("triangle")
        }
    }
}

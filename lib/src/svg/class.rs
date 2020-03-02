#[derive(Debug)]
pub enum Class {
    Line,
    Triangle,
    Polyline,
    Polygon,
}

impl Class {
    pub fn to_string(&self) -> String {
        match self {
            Class::Line => format!("line"),
            Class::Triangle => format!("triangle"),
            Class::Polygon => format!("polygone"),
            Class::Polyline => format!("polyline"),
        }
    }
}

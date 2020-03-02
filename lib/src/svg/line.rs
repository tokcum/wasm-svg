use crate::geometry::Line as GeoLine;
use crate::svg::class::Class;
use crate::svg::Pos;

#[derive(Clone, Copy, Debug)]
pub struct Line {
    //id: String,
    //class: Option<Class>,
    p1: Pos,
    p2: Pos,
}

impl Line {
    pub fn p1(&self) -> Pos {
        self.p1
    }
    
    pub fn p2(&self) -> Pos {
        self.p2
    }
}

impl From<GeoLine> for Line {
    fn from(line: GeoLine) -> Self {
        Line {
            //id: "".to_string(),
            //class: Some(Class::Line),
            p1: Pos::from(line.start()),
            p2: Pos::from(line.end()),
        }
    }
}

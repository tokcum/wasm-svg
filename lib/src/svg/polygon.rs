use crate::geometry::Triangle;
use crate::svg::class::Class;
use crate::svg::position::Position;

#[derive(Debug)]
pub struct Polygon {
    id: String,
    class: Option<Class>,
    points: Vec<Position>,
}

impl Polygon {
    pub fn id(&self) -> String {
        self.id.clone()
    }
    
    pub fn class(&self) -> String {
        self.class.as_ref().unwrap().to_string()
    }
    
    pub fn points(&self) -> Vec<Position> {
        self.points.clone()
    }
    
    pub fn to_svg(&self) -> String {
        let mut points= String::new();
        for v in self.points.iter() {
            points += format!("{},{} ", v.x(), v.y() ).as_str();
        }
        
        format!("id=\"{}\" class=\"{}\" points=\"{}\"", self.id, self.class.as_ref().unwrap().to_string(), points)
    }
}

impl Default for Polygon {
    fn default() -> Self {
        let e = Triangle::default().edges();
        let mut v: Vec<Position> = Vec::new();
        v.push(Position::from(e.0));
        v.push(Position::from(e.1));
        v.push(Position::from(e.2));

        Polygon {
            id: "".to_string(),
            class: Some(Class::Triangle),
            points: v,
        }
    }
}

impl From<Triangle> for Polygon {
    fn from(t: Triangle) -> Self {
        let e = t.edges();
        let mut v: Vec<Position> = Vec::new();
        v.push(Position::from(e.0));
        v.push(Position::from(e.1));
        v.push(Position::from(e.2));
    
        Polygon {
            id: "".to_string(),
            class: Some(Class::Triangle),
            points: v,
        }
    }
}

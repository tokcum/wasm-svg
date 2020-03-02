use crate::geometry::Point;
use crate::geometry::Triangle;
use crate::svg::class::Class;
use crate::svg::Pos;

#[derive(Clone, Debug)]
pub struct Polygon {
    //id: String,
    //class: Option<Class>,
    points: Vec<Pos>,
}

impl Polygon {
    /*pub fn id(&self) -> String {
        self.id.clone()
    }
    */
    /*
    pub fn class(&self) -> String {
        self.class.as_ref().unwrap().to_string()
    }
    */

    pub fn points(&self) -> Vec<Pos> {
        self.points.clone()
    }

    pub fn to_svg(&self) -> String {
        let mut points = String::new();
        for v in self.points.iter() {
            points += format!("{},{} ", v.x(), v.y()).as_str();
        }

        /*
        format!(
            "id=\"{}\" class=\"{}\" points=\"{}\"",
            self.id,
            self.class.as_ref().unwrap().to_string(),
            points
        )
        */
        format!("points=\"{}\"", points)
    }
}

impl Default for Polygon {
    fn default() -> Self {
        let e = Triangle::default().edges();
        let mut v: Vec<Pos> = Vec::new();
        v.push(Pos::from(e.0));
        v.push(Pos::from(e.1));
        v.push(Pos::from(e.2));

        Polygon {
            //id: "".to_string(),
            //class: Some(Class::Triangle),
            points: v,
        }
    }
}

impl From<Triangle> for Polygon {
    fn from(triangle: Triangle) -> Self {
        let e = triangle.edges();
        let mut v: Vec<Pos> = Vec::new();
        v.push(Pos::from(e.0 * (1, -1)));
        v.push(Pos::from(e.1 * (1, -1)));
        v.push(Pos::from(e.2 * (1, -1)));

        Polygon {
            //id: "".to_string(),
            //class: Some(Class::Triangle),
            points: v,
        }
    }
}

impl From<Vec<Point>> for Polygon {
    fn from(points: Vec<Point>) -> Self {
        let mut v: Vec<Pos> = Vec::new();
        for (i, point) in points.iter().enumerate() {
            v.push(Pos::from(*point * (1, -1)))
        }
        Polygon {
            //id: "".to_string(),
            //class: Some(Class::Polygon),
            points: v,
        }
    }
}

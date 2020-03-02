#[cfg(test)]
use crate::geometry::Line;
use crate::geometry::Point;
use crate::geometry::Triangle;

#[test]
fn triangle_create() {
    let b: Line = Line::new(Point::new(5, 10), Point::new(11, 10));
    let h: Line = Line::new(Point::new(8, 10), Point::new(8, 15));

    let t: Triangle = Triangle::new(b, h);

    assert_eq!(
        t,
        Triangle::new(
            Line::new(Point::new(5, 10), Point::new(11, 10)),
            Line::new(Point::new(8, 10), Point::new(8, 15))
        )
    );
}

#[test]
fn triangle_scale() {
    let mut t1: Triangle = Triangle::default();
    t1 = t1 * 2;

    let t2: Triangle = Triangle::new(
        Line::new(Point::new(-20, 0), Point::new(20, 0)),
        Line::new(Point::new(0, 0), Point::new(0, 20)),
    );

    assert_eq!(t1, t2);
}

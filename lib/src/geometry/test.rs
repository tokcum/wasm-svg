#[cfg(test)]

use crate::geometry::*;

#[test]
fn triangle_create() {
  let b: Line = Line::create( Point{ x: 5, y: 10 }, Point{ x: 11, y: 10} );
  let h: Line = Line::create( Point{ x: 8, y: 10 }, Point{ x: 8,  y: 15} );
  
  let t: Triangle::create(b, h);
  
  asserteq!(t, Triangle::create(Line { Point{ x: 5, y: 10}, Point{ x: 11, y: 10} }, Line { Point{ x: 8, y: 10}, Point{ x: 8, y: 15} }))
}

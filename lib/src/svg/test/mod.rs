#[cfg(test)]

use crate::svg::interval::Interval;
use crate::svg::scale::Scale;
use crate::utils::data::Data;

#[test]
fn test_scale() {
  let scale = Scale::new(Interval::new(0, 1), Interval::new(0, 2));
  
  let d1 = Data::VecI32(Vec::new());
  
  let d2 = scale.scale(&d1);
  
  assert_eq!(d1.unwrap().len(), d2.unwrap().len());
  
  for (i, v) in Data::VecI32(d1).iter().enumerate() {
    assert_eq!(*v, v*2);
  }
}

use crate::svg::interval::Interval;
use crate::utils::data::Data;

pub struct Scale {
    // In German: Definitionsmenge
    domain: Interval,
    // In German: Wertemenge
    image: Interval,
    t: ScaleType,
}

enum ScaleType {
    Linear,
}

impl Scale {
    pub fn new(domain: Interval, image: Interval) -> Self {
        Scale {
            domain: domain,
            image: image,
            t: ScaleType::Linear,
        }
    }

    pub fn domain(&self) -> Interval {
        self.domain.clone()
    }

    pub fn range(&self) -> Interval {
        self.image.clone()
    }
  
  pub fn scale(&self, data: &Data) -> Data {
    match self.t {
      ScaleType::Linear => {
        let f: f32 = self.image.max() as f32 / self.domain.max() as f32;
        
        match data {
          Data::VecI32(data) => {
            let mut v: Vec<i32> = Vec::new();
            for d in data.iter() {
              v.push((*d as f32 * f).round() as i32);
            }
            Data::VecI32(v)
          }
        }
      }
    }
  }
}

impl Default for Scale {
    fn default() -> Self {
        Scale {
            domain: Interval::default(),
            image: Interval::default(),
            t: ScaleType::Linear,
        }
    }
}

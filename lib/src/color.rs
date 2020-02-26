//! Representing a Color and providing an interface to color manipulation and color space
//! conversion.


/// Color supports different formats. As of today this is
/// # HSV
/// # RGB
/// # HEX
enum Color {
    ColorHSV(ColorHSV),
    ColorRGB(ColorRGB),
    ColorHEX(ColorHEX),
}

/// HSV colors are composed of three dimensions:
/// # hue ∈ [0,360] in degrees
/// # saturation ∈ [0,100], in %
/// # value ∈ [0,100], think of it as brightness in %
#[derive(Debug, Default)]
pub struct ColorHSV {
    h: u16,
    s: u8,
    v: u8,
}

impl ColorHSV {
  pub fn new() -> Self {
    ColorHSV { ..Default::default() }
  }
  
  pub fn create(h: u16, s: u8, v: u8, ) -> Self {
    ColorHSV { h, s, v, ..Default::default() }
  }
  
  pub fn shift_hue(&mut self, degree: u16) -> &Self {
    self.h = (self.h + degree) % 360;
    self
  }
  
  fn to_rgb(&self) -> ColorRGB {
    let s: f32 = self.s as f32 / 100.0;
    let v: f32 = self.v as f32 / 100.0;
    
    if s == 0.0
    {
      let c: u8 = (v * 255.0).round() as u8;
      return ColorRGB::new(c, c, c)
    }
    
    let h= (self.h as f32 / 60.0).floor() as i8;
    let f: f32 = self.h as f32/ 60.0 - h as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    
    match h {
      0 | 6 => ColorRGB {r: (v * 255.0).round() as u8, g: (t * 255.0).round() as u8, b: (p * 255.0).round() as u8},
      1 => ColorRGB {r: (q * 255.0).round() as u8, g: (v * 255.0).round() as u8, b: (p * 255.0).round() as u8},
      2 => ColorRGB {r: (p * 255.0).round() as u8, g: (v * 255.0).round() as u8, b: (t * 255.0).round() as u8},
      3 => ColorRGB {r: (p * 255.0).round() as u8, g: (q * 255.0).round() as u8, b: (v * 255.0).round() as u8},
      4 => ColorRGB {r: (t * 255.0).round() as u8, g: (p * 255.0).round() as u8, b: (v * 255.0).round() as u8},
      5 => ColorRGB {r: (v * 255.0).round() as u8, g: (p * 255.0).round() as u8, b: (q * 255.0).round() as u8},
      _ => ColorRGB::new(0, 0, 0)
    }
  }
  
  pub fn to_hex(&self) -> String {
    self.to_rgb().to_hex()
  }
}



#[derive(Debug)]
pub struct ColorRGB {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorRGB {
    fn new(r: u8, g: u8, b: u8) -> ColorRGB {
        ColorRGB { r, g, b }
    }

    fn to_hex(&self) -> String {
        format!("#{:x}{:x}{:x}", self.r, self.g, self.b)
    }
}

impl std::cmp::PartialEq for ColorRGB {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


#[derive(Debug, Default)]
pub struct ColorHEX {
  hex: String,
}

#[test]
fn test_to_rgb() {
  

    assert_eq!(ColorHSV::create(0, 100, 100).to_rgb(), ColorRGB::new(255, 0, 0));
  assert_eq!(ColorHSV::create(240, 100, 100).to_rgb(), ColorRGB::new(0, 0, 255));
  assert_eq!(ColorHSV::create(20, 75, 36).to_rgb(), ColorRGB::new(92, 46, 23));
  
  
}

#[test]
fn test_tohex() {
    let color_rgb = ColorRGB::new(66, 135, 245);

    assert_eq!("#4287f5", color_rgb.to_hex());
}

#[test]
fn test_shift() {
    let mut color = ColorHSV::create(89, 30, 100);
    color.shift_hue(20);

    assert_eq!(109, color.h);

    color.shift_hue(270);

    assert_eq!(19, color.h)
}

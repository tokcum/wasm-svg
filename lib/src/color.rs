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
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    
    // Convert to interval [0, 1]
    let h = self.h as f32 / 360.0;
    let s = self.s as f32 / 100.0;
    let v = self.v as f32 / 100.0;
    
    let chroma = v * s;
    let h1 = h * 6.0;
    let x = chroma * (1 - (h1 % 2 -1 ).abs());
    
    let i = (h * 6.0).floor();
    
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    
    if h1 >= 0.0 && h1 < 1.0 {
      r = chroma;
        g = x;
        b = 0.0;
      }
    
    match h1 % 6.0 {
      1 => {
        r = q;
        g = v;
        b = p;
      }
      2 => {
        r = p;
        g = v;
        b = t;
      }
      3 => {
        r = p;
        g = q;
        b = v;
      }
      4 => {
        r = t;
        g = p;
        b = v;
      }
      5 => {
        r = v;
        g = p;
        b = q;
      }
      _ => {}
    }
    
    ColorRGB {
      r: (r * 255.0) as u8,
      g: (g * 255.0) as u8,
      b: (b * 255.0) as u8,
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
fn test_convert() {
    let color = ColorHSV::create(89, 30, 100);
    let color_rgb = color.to_rgb();

    assert_eq!(ColorRGB::new(218, 255, 178), color_rgb);
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

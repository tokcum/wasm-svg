

pub struct Color_HSV {
    h: u16,
    s: u8,
    v: u8,
}

#[derive(Debug)]
pub struct Color_RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl Color_RGB {

    fn new(r: u8, g: u8, b: u8) -> Color_RGB {
        Color_RGB { r, g, b }
    }

    fn to_hex(&self) -> String {

        format!("#{:x}{:x}{:x}", self.r, self.g, self.b)

    }

}
impl std::cmp::PartialEq for Color_RGB {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Color_HSV {

    pub fn new(h: u16, s: u8, v: u8) -> Color_HSV {
        Color_HSV { h: h, s: s, v: v }
    }

    pub fn shift_hue(&mut self, degree: u16) -> &Self {

        self.h = (self.h + degree) % 360;

        self

    }

    fn to_rgb(&self) -> Color_RGB {

        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        let h = self.h as f32 / 360.0;
        let s = self.s as f32 / 100.0;
        let v = self.v as f32 / 100.0;

        let i = (h * 6.0).floor();

        let f = h * 6.0 - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        match (i as u8) % 6 {
            0 => {
                r = v;
                g = t;
                b = p;
            },
            1 => {
                r = q;
                g = v;
                b = p;
            },
            2 => {
                r = p;
                g = v;
                b = t;
            },
            3 => {
                r = p;
                g = q;
                b = v;
            },
            4 => {
                r = t;
                g = p;
                b = v;
            },
            5 => {
                r = v;
                g = p;
                b = q;
            },
            _ => {

            }
        }

        Color_RGB {r: (r * 255.0) as u8, g: (g * 255.0) as u8, b: (b * 255.0) as u8}

    }

    pub fn to_hex(&self) -> String {
        self.to_rgb().to_hex()
    }

}

#[test]
fn test_convert() {

    let color = Color_HSV::new(89, 30, 100);
    let color_rgb = color.to_rgb();

    assert_eq!(Color_RGB::new(218, 255, 178), color_rgb);

}

#[test]
fn test_tohex() {

    let color_rgb = Color_RGB::new(66, 135, 245);

    assert_eq!("#4287f5", color_rgb.to_hex());

}

#[test]
fn test_shift() {

    let mut color = Color_HSV::new(89, 30, 100);
    color.shift_hue(20);

    assert_eq!(109, color.h);

    color.shift_hue(270);

    assert_eq!(19, color.h)

}
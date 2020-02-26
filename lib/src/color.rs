

struct Color_HSV {
    h: u8,
    s: u8,
    v: u8,
}

#[derive(Debug)]
struct Color_RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl Color_RGB {

    fn new(r: u8, g: u8, b: u8) -> Color_RGB {
        Color_RGB { r, g, b }
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

    fn new(h: u8, s: u8, v: u8) -> Color_HSV {
        Color_HSV { h: h, s: s, v: v }
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

}

#[test]
fn test_convert() {

    let color = Color_HSV::new(89, 30, 100);
    let color_rgb = color.to_rgb();

    assert_eq!(Color_RGB::new(218, 255, 178), color_rgb);

}
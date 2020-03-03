use crate::web::Document;
use crate::web::Element;

pub struct SvgCanvas(pub web_sys::Element);

impl SvgCanvas {
    pub fn new(width: f32, height: f32) -> SvgCanvas {
        let mut e = Element::new("svg");
        e.attr("width", &width.to_string())
            .attr("height", &height.to_string())
            .attr(
                "viewBox",
                format!("{} {} {} {}", width / 2.0 * -1.0, height / 2.0 * -1.0, width, height).as_str(),
            );
        SvgCanvas(e.0)
    }
}

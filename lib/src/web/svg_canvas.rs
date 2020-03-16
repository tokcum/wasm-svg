use crate::web::Namespace::SVG;
use crate::web::{Document, Element};

#[derive(Debug)]
pub struct SvgCanvas {
    n: web_sys::SvgsvgElement,
}

impl SvgCanvas {
    pub fn new(width: f32, height: f32) -> Self {
        let doc = Document::new().unwrap();

        let mut canvas = SvgCanvas::from(doc.create("svg", Some(SVG)));
        canvas
            .attr("width", &width.to_string())
            .attr("height", &height.to_string())
            .attr(
                "viewBox",
                format!(
                    "{} {} {} {}",
                    width / 2.0 * -1.0,
                    height / 2.0 * -1.0,
                    width,
                    height
                )
                .as_str(),
            )
    }

    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.n.set_attribute(name, value).unwrap();
        self
    }

    pub fn get(self) -> web_sys::SvgsvgElement {
        self.n
    }
}

impl From<Element> for SvgCanvas {
    fn from(e: Element) -> Self {
        SvgCanvas { n: e.into() }
    }
}

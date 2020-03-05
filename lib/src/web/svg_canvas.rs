use crate::web::Namespace::SVG;
use crate::web::{Document, Element};
use std::ops::Deref;

#[derive(Debug)]
pub struct SvgCanvas(web_sys::SvgsvgElement);

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
        self.0.set_attribute(name, value).unwrap();
        self
    }
}

impl Deref for SvgCanvas {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
      &self
    }
}

impl From<Element> for SvgCanvas {
    fn from(element: Element) -> Self {
        SvgCanvas(element.into())
    }
}

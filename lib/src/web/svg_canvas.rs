use crate::web::Namespace::SVG;
use crate::web::{Document, Element};

use rand::Rng;

#[derive(Debug)]
pub struct SvgCanvas {
    n: web_sys::SvgsvgElement,
    class: Option<String>,
}

impl SvgCanvas {
    pub fn new(width: f32, height: f32) -> Option<Self> {
        let doc = Document::new().unwrap();

        let mut rng = rand::thread_rng();
        let class = format!("c{}", rng.gen_range(1000, 10000));

        let mut canvas = SvgCanvas::from(doc.create("svg", Some(SVG)));
        canvas
            .class(class.as_str())
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
            );

        Some(canvas)
    }

    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self {
        self.n.set_attribute(name, value).unwrap();
        self
    }

    pub fn get(self) -> web_sys::SvgsvgElement {
        self.n
    }

    pub fn class(&mut self, class: &str) -> &mut Self {
        // Todo: think about adding a class instead of overwriting an existing class
        self.n.set_attribute("class", class).unwrap();
        self.class = Some(class.to_string());
        self
    }

    pub fn _class(&self) -> Option<String> {
        self.class.clone()
    }
}

impl From<Element> for SvgCanvas {
    fn from(e: Element) -> Self {
        SvgCanvas {
          class: e._class(),
          n: e.into(),
        }
    }
}

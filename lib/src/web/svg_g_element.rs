use crate::web::document::Document;
use crate::web::element::Element;
use crate::web::Namespace::SVG;

#[derive(Debug)]
pub struct SvgGElement(web_sys::SvggElement);

impl SvgGElement {
    pub fn new() -> Self {
        let doc = Document::new().unwrap();
        SvgGElement::from(doc.create("g", Some(SVG)))
    }
}

impl From<Element> for SvgGElement {
    fn from(element: Element) -> Self {
        SvgGElement(element.into())
    }
}

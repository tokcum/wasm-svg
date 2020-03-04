use crate::web::document::Document;
use crate::web::element::Element;
use crate::web::Namespace::SVG;

pub struct SvgElement(web_sys::SvgElement);

impl SvgElement {
    pub fn new(name: &str) -> Self {
        let doc = Document::new().unwrap();
        SvgElement::from(doc.create(name, Some(SVG)))
    }

    pub fn get_id(&self) -> String {
        self.0.id()
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.0.set_id(id);
        self
    }
    
    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self {
        self.0.set_attribute(name, value).unwrap();
        self
    }
    
    pub fn class(&mut self, class: &str) -> &mut Self {
        // Todo: think about adding a class instead of overwriting an existing class
        self.0.set_attribute("class", class).unwrap();
        self
    }
    
    pub fn html(&self, s: &str) {
        self.0.set_inner_html(s);
    }
    
    pub fn append(&mut self, element: SvgElement) -> &mut Self {
        self
    }
}

impl From<Element> for SvgElement {
    fn from(element: Element) -> Self {
        SvgElement(element.into())
    }
}

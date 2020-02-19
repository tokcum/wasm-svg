use wasm_bindgen::JsCast;

use crate::document::*;

pub struct Element(pub web_sys::Element);

impl Element {
    /*
    fn create() -> Element {
        let d = Document::new();
        Element(d.0.create_element("none").unwrap())
    }
    */
    
    pub fn new(node: &str) -> Element {
        let d = Document::new();
        Element(d.0.create_element_ns(Some("http://www.w3.org/2000/svg"), node).unwrap())
    }

    pub fn new_svg() -> Element {
        let d = Document::new();
        Element(d.0.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg").unwrap())
    }
    
    pub fn new_svg_element(s: &str) -> Element {
        let d = Document::new();
        Element(d.0.create_element_ns(Some("http://www.w3.org/2000/svg"), s).unwrap())
    }
    
    pub fn append_svg(&self) -> Option<Element> {
        let e = Element::new_svg();

        Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
    }
    
    pub fn append_svg_element(&self, s: &str) -> Option<Element> {
        let e = Element::new_svg_element(s);
        
        Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
    }

    pub fn attr(&mut self, name: &str, value: &str) -> &mut Element {
        self.0.set_attribute(name, value).unwrap();
        self
    }

    pub fn from(e: web_sys::Element) -> Element {
        Element(e)
    }

    pub fn html(&self, s: &str) {
        self.0.set_inner_html(s);
    }
}

impl super::Selection for Element {
    fn select (&self, s: &str) -> Option<Element> {
        Some(Element::from(self.0.query_selector(s).unwrap().unwrap()))
    }
    
    fn append(&self, s: &str) -> Option<Element> {
        let e = Element::new(s);

        Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
    }
}

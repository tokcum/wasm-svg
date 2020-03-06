use wasm_bindgen::JsCast;

use crate::web::element::Element;
use crate::web::Namespace;

#[derive(Debug)]
pub struct Document {
    n: web_sys::Document,
}

impl Document {
    pub fn new() -> Option<Document> {
        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        Some(Document{ n: doc })
    }

    pub fn body(&self) -> Option<Element> {
        // web_sys::HtmlElement converted into web_sys::Element via .dyn_into::<T>()
        Some(Element::new( self.n
                .body()
                .unwrap()
                .dyn_into::<web_sys::Element>()
                .unwrap()))
    }

    pub fn create(&self, name: &str, ns: Option<Namespace>) -> Element {
        match ns {
            Some(Namespace::XHTML) => Element::new(
                self.n
                    .create_element_ns(Some("http://www.w3.org/1999/xhtml"), name)
                    .unwrap(),
            ),
            Some(Namespace::SVG) => Element::new(
                self.n
                    .create_element_ns(Some("http://www.w3.org/2000/svg"), name)
                    .unwrap(),
            ),
            None => Element::new(self.n.create_element(name).unwrap()),
        }
    }
    
    pub fn select(&self, selector: &str) -> Option<Element> {
        Some(Element::new( self.n.query_selector(selector).unwrap().unwrap()))
    }
}

use wasm_bindgen::JsCast;

use crate::web::element::Element;
use crate::web::Namespace;

#[derive(Debug)]
pub struct Document(web_sys::Document);

impl Document {
    pub fn new() -> Option<Document> {
        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        Some(Document(doc))
    }

    pub fn body(&self) -> Option<Element> {
        // web_sys::HtmlElement converted into web_sys::Element via .dyn_into::<T>()
        Some(Element::from(
            self.0
                .body()
                .unwrap()
                .dyn_into::<web_sys::Element>()
                .unwrap(),
        ))
    }

    pub fn create(&self, name: &str, ns: Option<Namespace>) -> Element {
        match ns {
            Some(Namespace::XHTML) => {
                Element::from(self.0
                  .create_element_ns(Some("http://www.w3.org/1999/xhtml"), name)
                  .unwrap())
            }
            Some(Namespace::SVG) => {
                Element::from(self.0
                  .create_element_ns(Some("http://www.w3.org/2000/svg"), name)
                  .unwrap())
            }
            None => {
                Element::from(self.0
                  .create_element(name)
                  .unwrap())
            }
        }
    }

    pub fn select(&self, selector: &str) -> Option<Element> {
        Some(Element::from(
            self.0
              .query_selector(selector).unwrap().unwrap()))
    }
}

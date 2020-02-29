use wasm_bindgen::JsCast;

use crate::element::*;
use crate::nodelist::*;

pub struct Document(pub web_sys::Document);

impl Document {
    pub fn new() -> Document {
        let w = web_sys::window().expect("no global `window` exists");
        let d = w.document().expect("should have a document on window");
        Document(d)
    }
}

impl super::Selection for Document {
    fn select (&self, s: &str) -> Option<Element> {
        Some(Element::from(self.0.query_selector(s).unwrap().unwrap()))
    }
  
  fn select_all(&self, _s: &str) -> Option<Nodes> {
    unimplemented!()
  }
  
  fn append(&self, s: &str) -> Option<Element> {
        let e = Element::new(s);

        Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
    }
}

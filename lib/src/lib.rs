use wasm_bindgen::prelude::*;

//Using the Newtype Pattern to Implement External Traits on External Types
//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types

pub struct Document(web_sys::Document);
pub struct ElementNG(web_sys::Element);
pub struct Selection(Vec<ElementNG>);

pub trait Select {
  fn select(&self, s: &str) -> Option<ElementNG>;
  fn select_all(&self, s: &str) -> Option<Selection>;
}

impl Select for Document {
  fn select (&self, s: &str) -> Option<ElementNG> {
    None
  }
  
  fn select_all (&self, s: &str) -> Option<Selection> {
    None
  }
}


// Homage to D3.js, Data-Driven Documents
pub struct D3 {
  pub document: web_sys::Document,
}

impl D3 {
  pub fn new() -> D3 {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let w = web_sys::window().expect("no global `window` exists");
    let d = w.document().expect("should have a document on window");
  
    D3 { document: d }
  }
  
  pub fn select(&self, s: &str) -> Option<Element> {
    let mut e = Element::create();
    e.element = self.document.query_selector(s).unwrap().unwrap();
    Some(e)
  }
}

pub struct Element {
  pub element: web_sys::Element,
}

impl Element {
  fn create() -> Element {
    let d3 = D3::new();
    Element { element: d3.document.create_element("none").unwrap() }
  }
  
  pub fn new(node: &str) -> Element {
    let d3 = D3::new();
    let e = d3.document.create_element_ns(Some("http://www.w3.org/2000/svg"), node).unwrap();
    
    Element { element: e }
  }
  
  pub fn attr(&mut self, name: &str, value: &str) -> &mut Element {
    self.element.set_attribute(name, value);
    self
  }
}

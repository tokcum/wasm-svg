use wasm_bindgen::prelude::*;

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
  
  pub fn select(&self, s: &str) -> Result<web_sys::Element, JsValue> {
    let el = self.document.query_selector(s)?.unwrap();
    Ok(el)
  }
}

pub struct Element {
  pub element: web_sys::Element,
}

impl Element {
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

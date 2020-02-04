use wasm_bindgen::prelude::*;

// Homage to D3.js, Data-Driven Documents
struct D3 {
  document: web_sys::Document,
}

impl D3 {
  fn new() -> D3 {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let w = web_sys::window().expect("no global `window` exists");
    let d = w.document().expect("should have a document on window");
  
    D3 { document: d }
  }
  
  fn select(&self, s: &str) -> Result<web_sys::Element, JsValue> {
    let el = self.document.query_selector(s)?.unwrap();
    Ok(el)
  }
}

struct Element {
  element: web_sys::Element,
}

impl Element {
  fn new(node: &str) -> Element {
    let d3 = D3::new();
    let e = d3.document.create_element_ns(Some("http://www.w3.org/2000/svg"), node).unwrap();
    
    Element { element: e }
  }
  
  fn attr(&mut self, name: &str, value: &str) -> &mut Element {
    self.element.set_attribute(name, value);
    self
  }
}

#[wasm_bindgen]
pub fn hello(name: String) -> String {
  let result = format!("Hello {}!", name);
  return result.into();
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
  let d3 = D3::new();
  let body = d3.document.body().expect("document should have a body");
  
  // Manufacture the element we're gonna append
  let val = d3.document.create_element("p")?;
  val.set_inner_html("Hello from Rust!");
  
  body.append_child(&val)?;
  
  Ok(())
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn nameit(selector: &str) -> Result<web_sys::Element, JsValue> {
  let d3 = D3::new();
  let el = d3.select(selector)?;
  //let el = d3.query_selector(selector)?.unwrap();
  el.set_inner_html("Test4");
  Ok(el)
}

#[wasm_bindgen]
pub fn append_svg(selector: &str, element: &str) -> Result<web_sys::Element, JsValue> {
  let d3 = D3::new();
  let body = d3.document.body().expect("document expect to have have a body");
  
  let mut svg = Element::new("svg");
  svg.attr("width", "300").attr("height", "300");
  
  body.append_child(&svg.element)?;
  
  Ok(svg.element)
}

#[wasm_bindgen]
pub fn append_rect(selector: &str, element: &str) -> Result<web_sys::Element, JsValue> {
  let d3 = D3::new();
  let el = d3.select(selector).unwrap();
  
  let mut rect = Element::new("rect");
  rect.attr("x", "1.0").attr("y", "1.0").attr("width", "150.0").attr("height", "150.0").attr("class", "rect").attr("id", "rect");
  el.append_child(&rect.element)?;
  
  Ok(rect.element)
}

#[wasm_bindgen]
pub fn move_rect(selector: &str, element: &str) -> Result<web_sys::Element, JsValue> {
  let d3 = D3::new();
  let el = d3.select(selector).unwrap();
  
  el.set_attribute("x", "30.0");
  el.set_attribute("y", "30.0");
  
  Ok(el)
}

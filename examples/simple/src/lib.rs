use wasm_bindgen::prelude::*;
use wasm_svg_lib::*;

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

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello(name: String) -> String {
  let result = format!("Hello {}!", name);
  return result.into();
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
  // Use `web_sys`'s global `window` function to get a handle on the global
  // window object.
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  
  // Manufacture the element we're gonna append
  let val = document.create_element("p")?;
  val.set_inner_html("Hello from Rust!");
  
  body.append_child(&val)?;
  
  Ok(())
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn select(selector: &str) -> Result<web_sys::Element, JsValue> {
  // Use `web_sys`'s global `window` function to get a handle on the global
  // window object.
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  
  let el = document.query_selector(selector)?.unwrap();
  el.set_inner_html("Test3");
 
  Ok(el)
}

#[wasm_bindgen]
pub fn append(selector: &str, element: &str) -> Result<web_sys::Element, JsValue> {
  // Use `web_sys`'s global `window` function to get a handle on the global
  // window object.
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document expect to have have a body");
  
  let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
  svg.set_attribute("width", "300");
  svg.set_attribute("height", "300");
  
  let rect1 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
  rect1.set_attribute("x", "1.0");
  rect1.set_attribute("y", "1.0");
  rect1.set_attribute("width", "150.0");
  rect1.set_attribute("height", "150.0");
  rect1.set_attribute("class", "rect");
  rect1.set_id("rect1");
  
  let rect2 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
  rect2.set_attribute("x", "21.0");
  rect2.set_attribute("y", "21.0");
  rect2.set_attribute("width", "50.0");
  rect2.set_attribute("height", "50.0");
  rect2.set_attribute("class", "rect");
  rect2.set_id("rect2");
  
  
  svg.append_child(&rect1)?;
  svg.append_child(&rect2)?;
  body.append_child(&svg)?;
  
  Ok(rect2)
}

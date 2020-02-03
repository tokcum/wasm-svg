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
  
  let svg = document.create_element("svg").unwrap();
  svg.set_attribute("width", "300");
  svg.set_attribute("height", "300");
  
  let rect = document.create_element("rect")?;
  rect.set_attribute("x", "10.0");
  rect.set_attribute("y", "10.0");
  rect.set_attribute("width", "150.0");
  rect.set_attribute("height", "150.0");
  rect.set_attribute("fill", "blue")?;
  
  svg.append_child(&rect)?;
  body.append_child(&svg)?;
  
  Ok((rect))
/*
  let sel = document.query_selector(selector)?.unwrap();
  let el = document.create_element(element)?;
  
  let e = sel.insert_adjacent_element("afterbegin", &el)?.unwrap();
  e.set_attribute("width", "300");
  e.set_attribute("height", "300");
  
  let rect = document.create_element("rect")?.dyn_into::<web_sys::SvgRect>()?;
  rect.set_x(10.0);
  rect.set_y(10.0);
  rect.set_width(150.0);
  rect.set_height(150.0);
  
  e.insert_adjacent_element("afterbegin", &rect)?.unwrap();
*/
}

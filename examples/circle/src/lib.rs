use wasm_bindgen::prelude::*;
use wasm_svg_lib::document::*;
//use wasm_svg_lib::node::*;
use wasm_svg_lib::Selection;

#[wasm_bindgen]
pub fn hello(name: String) -> String {
  let result = format!("Hello {}!", name);
  return result.into();
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
  let d = Document::new();
  let body = d.select("body").expect("document should have a body");
  
  body.append("p").unwrap().html("Hello from Rust!");
  
  body.append_svg().unwrap().attr("width", "300").attr("height", "300");
  
  let mut e = body.select("svg").unwrap().append_svg_element("circle").unwrap();
  e.attr("cx", "50");
  e.attr("cy", "50");
  e.attr("r", "50");
  
  let mut f = body.select("svg").unwrap().append_svg_element("circle").unwrap();
  f.attr("cx", "80");
  f.attr("cy", "80");
  f.attr("r", "30");
  
  Ok(())
}

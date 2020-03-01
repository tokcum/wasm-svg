use wasm_bindgen::prelude::*;
use wasm_svg_lib::web::Document;
use wasm_svg_lib::web::Selection;

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
  
  Ok(())
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn nameit(selector: &str) -> Result<(), JsValue> {
  let d = Document::new();
  let el = d.select(selector).unwrap();

  el.html("Test4");
  
  Ok(())
}

#[wasm_bindgen]
pub fn append_svg() -> Result<(), JsValue> {
  let d = Document::new();
  let body = d.select("body").expect("document expect to have have a body");
  
  body.append_svg().unwrap().attr("width", "300").attr("height", "300");
  
  Ok(())
}

#[wasm_bindgen]
pub fn append_rect(selector: &str) -> Result<(), JsValue> {
  let d = Document::new();
  let el = d.select(selector).unwrap();
  
  el.append("rect").unwrap().attr("x", "1.0").attr("y", "1.0").attr("width", "150.0")
    .attr("height", "150.0").attr("class", "rect").attr("id", "rect");
  
  Ok(())
}

#[wasm_bindgen]
pub fn move_rect(selector: &str) -> Result<(), JsValue> {
  let d = Document::new();
  let mut el = d.select(selector).unwrap();
  
  el.attr("x", "30.0")
    .attr("y", "30.0");
  
  Ok(())
}

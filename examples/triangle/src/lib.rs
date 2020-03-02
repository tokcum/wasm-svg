use wasm_bindgen::prelude::*;
use wasm_svg_lib::geometry::{Triangle};
use wasm_svg_lib::svg::{Polygon, SvgElement};
use wasm_svg_lib::web::*;
use wasm_svg_lib::web::{Document};

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
  
  let w: i32 = 500;
  let h: i32 = 500;
  
  let t = Triangle::default();
  let p = Polygon::from(t);
  
  //body.append("p").unwrap().html("Hello from Rust!");
  //body.append("p").unwrap().html(p.to_svg().as_str());
  
  body.append_svg().unwrap().attr("width", &w.to_string()).attr("height", &h.to_string()).attr("viewBox", &format!("{} {} {} {}", w/2*-1, h/2*-1, w, h));
  
  let mut g = body.select("svg").unwrap().append(&SvgElement::Polygon(&p)).unwrap();
  
  Ok(())
}

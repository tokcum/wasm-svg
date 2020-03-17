use wasm_bindgen::prelude::*;
use wasm_svg_lib::{d3, web};

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let w: i32 = 500;
    let h: i32 = 500;

    let doc = d3::Doc::new(w as f32, h as f32).unwrap();

    doc.document.body().unwrap().append(&web::Element::create("p")).html(format!("{:?}", doc.canvas).as_str());
  
  doc.document.body().unwrap().select(".test").unwrap().html(format!("{:?}", doc.document.body().unwrap().select(".test")).as_str());

    Ok(())
}

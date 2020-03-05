use wasm_bindgen::prelude::*;
use wasm_svg_lib::svg::SvgElement;
use wasm_svg_lib::web::{Document, Element, Selection, SvgCanvas};

use console_error_panic_hook;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let w: i32 = 500;
    let h: i32 = 500;

    let doc = Document::new().unwrap();
    let body = doc.body().unwrap();
    body.append(&Element::new("p"));
      //  .html("Hello from inside Rust WASM code base.");

    let canvas = SvgCanvas::new(w as f32, h as f32);
  //log!("{:?}", canvas);
  
  log!("{:?}", body.append(&canvas));

    Ok(())
}

use wasm_bindgen::prelude::*;
use wasm_svg_lib::web::{Document, Element, SvgCanvas};

use console_error_panic_hook;

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let w: i32 = 500;
    let h: i32 = 500;

    let doc = Document::new().unwrap();
    let body = doc.body().unwrap();
    body.append(Element::create("p"))
        .html("Hello from inside Rust WASM code base.");

    let canvas = SvgCanvas::new(w as f32, h as f32);
  body.append(Element::from(canvas));

    Ok(())
}

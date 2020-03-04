use wasm_bindgen::prelude::*;
use wasm_svg_lib::svg::SvgElement;
use wasm_svg_lib::web::{Document, Element};
use wasm_svg_lib::web::Selection;

#[wasm_bindgen]
pub fn hello(name: String) -> String {
    let result = format!("Hello {}!", name);
    return result.into();
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let doc = Document::new().unwrap();
    let body = doc.body().unwrap();

    body.append(&Element::new("p"))
        .html("Hello from Rust!");

    Ok(())
}

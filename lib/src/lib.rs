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

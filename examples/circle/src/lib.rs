use wasm_bindgen::prelude::*;
use wasm_svg_lib::document::*;
use wasm_svg_lib::element::*;
use wasm_svg_lib::nodelist::*;
use wasm_svg_lib::Selection;
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_svg_lib::color::*;
use wasm_svg_lib::circle::*;

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
  
  body.append("style").unwrap().html(r#"#A-31:hover, #A-32:hover, #A-33:hover {
    #fill: #ec008c;
    opacity: 0.5;
  }
  #A-31:hover + text {
  display: block;
  }

  "#);
  
  let w: i32 = 500;
  let h: i32 = 500;
  
  let circle: Circle = Circle{cx: 0.0, cy: 0.0, r : 80.0 };
  
  let data = [10, 15, 25, 13, 45];
  
  let mut vec: HashMap<&str, &str> = HashMap::new();
  vec.insert("A-31", "blue");
  vec.insert("A-32", "green");
  vec.insert("A-33", "red");
  vec.insert("A-34", "red");
  vec.insert("A-35", "red");
  vec.insert("A-36", "red");
  vec.insert("A-37", "red");
  vec.insert("A-38", "red");
  vec.insert("A-39", "red");
  
  body.append_svg().unwrap().attr("width", &w.to_string()).attr("height", &h.to_string()).attr("viewBox", &format!("{} {} {} {}", w/2*-1, h/2*-1, w, h));
  
  let mut g = body.select("svg").unwrap().append_svg_circle(&circle).unwrap();
  
  g.attr("fill", "white")
    .attr("stroke", "gray")
    .attr("stroke-width", "1");
  
  let h = body.select("svg").unwrap().append_svg_element("g").unwrap();
  let mut i = 0;
  let mut color = ColorHSV::create(0, 30, 82);
  
  for v in vec.iter() {
    h.append_svg_element("circle").unwrap()
      .id(&(v.0).to_string())
      .class("node")
      .attr("cx", "0")
      .attr("cy", &(circle.r*-1.0).to_string())
      .attr("r", "10")
      .attr("fill", &color.shift_hue((360/vec.len()) as u16).to_hex()) // &(v.1).to_string()
      .attr("stroke", "gray")
      .attr("stroke-width", "1")
      .attr("transform", &format!("rotate({}, 0, 0)", 360/vec.len()*i));
    h.append_svg_element("text").unwrap()
        .attr("x", "0")
        .attr("y", "0")
        .attr("fill", "black")
        .attr("display", "none")
        .html(&v.0);
    i += 1;
  }
  
  let j = g.select_all(".node").unwrap().data(&data);
  g.html(format!("Test {:?}", j.data).as_str());
  
  /*
  h.append_svg_element("circle").unwrap()
    .attr("id", "circle1")
    .attr("cx", "0")
    .attr("cy", &(r*-1).to_string())
    .attr("r", "10")
    .attr("fill", "blue")
    .attr("stroke", "gray")
    .attr("stroke-width", "1")
    .attr("transform", "rotate(0, 0, 0)")
    .attr("style", "");
  
  h.append_svg_element("circle").unwrap()
    .attr("id", "circle2")
    .attr("cx", "0")
    .attr("cy", &(r*-1).to_string())
    .attr("r", "10")
    .attr("fill", "green")
    .attr("stroke", "gray")
    .attr("stroke-width", "1")
    .attr("transform", "rotate(120, 0, 0)");
  
  h.append_svg_element("circle").unwrap()
    .attr("id", "circle3")
    .attr("cx", "0")
    .attr("cy", &(r*-1).to_string())
    .attr("r", "10")
    .attr("fill", "red")
    .attr("stroke", "gray")
    .attr("stroke-width", "1")
    .attr("transform", "rotate(240, 0, 0)");
    */
    
  Ok(())
}

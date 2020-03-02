use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_svg_lib::geometry::Circle;
use wasm_svg_lib::geometry::Point;
use wasm_svg_lib::svg::Polyline;
use wasm_svg_lib::svg::SvgElement;
use wasm_svg_lib::utils::color::*;
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

    body.append(&SvgElement::String("p".to_string()))
        .unwrap()
        .html("Hello from Rust!");

    let w: i32 = 500;
    let h: i32 = 500;

    let mut points: Vec<Point> = Vec::new();
    points.push(Point::new(0, 1));
    points.push(Point::new(10, 50));
    points.push(Point::new(20, 80));
    points.push(Point::new(30, 160));
    points.push(Point::new(40, 120));
    points.push(Point::new(50, 60));
    points.push(Point::new(60, -60));
    points.push(Point::new(70, -100));
    points.push(Point::new(80, 20));
    points.push(Point::new(90, 40));
    points.push(Point::new(100, 0));
  
  let pos: SvgElement = SvgElement::Polyline(Polyline::from(points));
    
    body.append_svg()
        .unwrap()
        .attr("width", &w.to_string())
        .attr("height", &h.to_string())
        .attr(
            "viewBox",
            &format!("{} {} {} {}", w / 2 * -1, h / 2 * -1, w, h),
        );
  let h = body.select("svg").unwrap();
  
  h.append(&pos);
  
  Ok(())
}

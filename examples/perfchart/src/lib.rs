use wasm_bindgen::prelude::*;
use wasm_svg_lib::geometry::Circle;
use wasm_svg_lib::geometry::Point;
use wasm_svg_lib::svg::Axis;
use wasm_svg_lib::svg::Polyline;
use wasm_svg_lib::svg::SvgElement;
use wasm_svg_lib::utils::color::*;
use wasm_svg_lib::web::{Document, SvgCanvas, SvgGElement};
use wasm_svg_lib::web::{BasicElement, Element, Selection};

#[wasm_bindgen]
pub fn hello(name: String) -> String {
    let result = format!("Hello {}!", name);
    return result.into();
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let w: i32 = 500;
    let h: i32 = 500;

    let d = Document::new();
    let body = d.select("body").expect("document should have a body");

    let canvas = SvgCanvas::new(500 as f32, 500 as f32);

    let svg = body.append_generic_element(&Element::from(canvas)).unwrap();

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
    points.push(Point::new(110, 50));
    points.push(Point::new(120, 0));

    let pos: SvgElement = SvgElement::Polyline(Polyline::from(points));

    svg.append(&pos);

    svg.append_svg_element("g").unwrap();
    let mut a = svg.append(&SvgElement::Axis(&Axis::default())).unwrap();
    a.attr("stroke", "gray").attr("stroke-width", "2");

    let g = SvgGElement::new();
    g.set_id("test.id");

    svg.append_generic_element(&Element::from(g)).unwrap();
    Ok(())
}

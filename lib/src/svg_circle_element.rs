use wasm_bindgen::JsCast;

use super::Selection;

use crate::document::*;
use crate::element::Element;
use crate::nodelist::*;


// Creating a new type in a tuple struct with just one field thus
// being a thin wrapper around the type.
pub struct SvgCircleElement(web_sys::Element);

impl SvgCircleElement {
    pub fn new(cx: f32, cy: f32, r: f32) -> SvgCircleElement {
        let d = Document::new();
        let e = SvgCircleElement(d.0.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle").unwrap());
        e.0.set_attribute("cx", format!("{}", cx).as_str()).unwrap();
        e.0.set_attribute("cy", format!("{}", cy).as_str()).unwrap();
        e.0.set_attribute("r", format!("{}", r).as_str()).unwrap();
        e
    }
    /*pub fn new(n: web_sys::SvgCircleElement) -> SvgCircleElement {
        SvgCircleElement(n)
    }*/
}

impl Selection for SvgCircleElement {
    fn select (&self, s: &str) -> Option<Element> {
        Some(Element::from(self.0.query_selector(s).unwrap().unwrap()))
    }
  
  fn select_all(&self, _s: &str) -> Option<Nodes> {
    unimplemented!()
  }
  
  
  fn append(&self, s: &str) -> Option<Element> {
        let e = Element::new(s);
        
        Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
    }
}

/*
 1var jsonCircles = [
 2  { "x_axis": 30, "y_axis": 30, "radius": 20, "color" : "green" },
 3  { "x_axis": 70, "y_axis": 70, "radius": 20, "color" : "purple"},
 4  { "x_axis": 110, "y_axis": 100, "radius": 20, "color" : "red"}];
 5
 6var svgContainer = d3.select("body").append("svg")
 7                                    .attr("width", 200)
 8                                    .attr("height", 200);
 9
10var circles = svgContainer.selectAll("circle")
11                          .data(jsonCircles)
12                          .enter()
13                          .append("circle");
14
15var circleAttributes = circles
16                       .attr("cx", function (d) { return d.x_axis; })
17                       .attr("cy", function (d) { return d.y_axis; })
18                       .attr("r", function (d) { return d.radius; })
19                       .style("fill", function(d) { return d.color; });
*/

/*
1circleRadii = [40, 20, 10]
 2
 3var svgContainer = d3.select("body").append("svg")
 4                                    .attr("width", 600)
 5                                    .attr("height", 100);
 6
 7var circles = svgContainer.selectAll("circle")
 8                          .data(circleRadii)
 9                          .enter()
10                          .append("circle")
11
12var circleAttributes = circles
13                       .attr("cx", 50)
14                       .attr("cy", 50)
15                       .attr("r", function (d) { return d; })
16                       .style("fill", function(d) {
17                         var returnColor;
18                         if (d === 40) { returnColor = "green";
19                         } else if (d === 20) { returnColor = "purple";
20                         } else if (d === 10) { returnColor = "red"; }
21                         return returnColor;
22                       });
*/

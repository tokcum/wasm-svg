use wasm_bindgen::JsCast;

use crate::document::*;
use wasm_bindgen::__rt::core::f64::consts::PI;
use crate::circle::Circle;

pub struct Element(pub web_sys::Element);

impl Element {
    /*
    fn create() -> Element {
        let d = Document::new();
        Element(d.0.create_element("none").unwrap())
    }
    */
    
    pub fn new(node: &str) -> Element {
        let d = Document::new();
        Element(d.0.create_element_ns(Some("http://www.w3.org/2000/svg"), node).unwrap())
    }

    pub fn new_svg() -> Element {
        let d = Document::new();
        Element(d.0.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg").unwrap())
    }
    
    pub fn new_svg_element(s: &str) -> Element {
        let d = Document::new();
        Element(d.0.create_element_ns(Some("http://www.w3.org/2000/svg"), s).unwrap())
    }
    
    pub fn append_svg(&self) -> Option<Element> {
        let e = Element::new_svg();

        Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
    }
    
    pub fn append_svg_element(&self, s: &str) -> Option<Element> {
        let e = Element::new_svg_element(s);
        
        Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
    }
    
    pub fn append_svg_circle(&self, circle: &Circle) -> Option<Element> {
        let mut e = Element::new_svg_element("circle");
        e.attr("cx", format!("{}", circle.cx).as_str())
          .attr("cy", format!("{}", circle.cy).as_str())
          .attr("r", format!("{}", circle.r).as_str());
    
        Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
    }

    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self {
        self.0.set_attribute(name, value).unwrap();
        self
    }
    
    pub fn id (&mut self, id: &str) -> &mut Self {
        self.0.set_attribute("id", id).unwrap();
        self
    }
    
    pub fn class(&mut self, class: &str) -> &mut Self {
        // Todo: think about adding a class instead of overwriting an existing class
        self.0.set_attribute("class", class).unwrap();
        self
    }
  
    pub fn from(e: web_sys::Element) -> Element {
        Element(e)
    }

    pub fn html(&self, s: &str) {
        self.0.set_inner_html(s);
    }
}

impl super::Selection for Element {
    fn select (&self, s: &str) -> Option<Element> {
        Some(Element::from(self.0.query_selector(s).unwrap().unwrap()))
    }
    
    fn append(&self, s: &str) -> Option<Element> {
        let e = Element::new(s);

        Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
    }
}

pub fn arc(cx: f32, cy: f32, r: f32, b: f32) -> String {
    format!("M {},{} m 0,{} v {} a {},{} 0 0,0 {},{} h {} a {},{} 0 0,1 {},{} Z", cx, cy, -(r-b), -b, r, r, -r, r, b, r-b, r-b, r-b, -(r-b))
}

pub fn arc_alpha(cx: f64, cy: f64, r: f64, b: f64, a: f64) -> String {
    //f.attr("d", arc_alpha(250.0, 250.0, 100.0, 3.0, 45.0).as_str())
    //(a.cos()*(cx+r))-(cx+r)
    //(-1.0 * w.sin()*(cy))+(cy)
    
    // Rust calculates in Radiant not Gradiant
    let w = a/360.0 * 2.0 * PI;
    
    format!("M {},{} l {},{} h {} a {},{} 0 0,1 {},{} v {} Z", cx,cy, (r-b),0, b, r, r, w.cos()*r-r, w.sin()*r, -b)
}

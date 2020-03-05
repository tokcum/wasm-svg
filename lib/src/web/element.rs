use wasm_bindgen::JsCast;

use crate::geometry::Circle;
use crate::web::document::*;
use crate::web::nodelist::Nodes;
use crate::web::SvgCanvas;
use crate::web::SvgElement;
use crate::web::SvgGElement;
use std::f64::consts::PI;

pub struct Element(web_sys::Element);

impl Element {
    pub fn new(name: &str) -> Self {
        let doc = Document::new().unwrap();
        doc.create(name, None)
    }

    pub fn append(&self, element: &Element) {
        self.0.append_child(&element.0);
    }

    pub fn html(&self, s: &str) {
        self.0.set_inner_html(s);
    }

    /*
    pub fn append_svg_circle(&self, circle: &Circle) -> Option<Element> {
        let mut e = SvgElement::new("circle");
        e.attr("cx", format!("{}", circle.cx).as_str())
            .attr("cy", format!("{}", circle.cy).as_str())
            .attr("r", format!("{}", circle.r).as_str());

        Some(Element::from(
            self.0
                .append_child(&e.0)
                .unwrap()
                .dyn_into::<web_sys::Element>()
                .unwrap(),
        ))
    }
    */

    /*
    pub fn append(&self, element: &crate::svg::SvgElement) -> Option<SvgElement> {
        match element {
            crate::svg::SvgElement::Axis(axis) => {
                let mut e = SvgElement::new("g");
                //e.append(&crate::svg::SvgElement::Line(&axis.line()));
                //e.append(&crate::svg::SvgElement::Polygon(&axis.head().unwrap()));

                Some(SvgElement::from(
                    self.0
                        .append_child(&e.0)
                        .unwrap()
                        .dyn_into::<web_sys::Element>()
                        .unwrap(),
                ))
            }
            crate::svg::SvgElement::Line(line) => {
                let mut e = SvgElement::new("line");
                e.attr("x1", format!("{}", line.p1().x()).as_str())
                    .attr("y1", format!("{}", line.p1().y()).as_str())
                    .attr("x2", format!("{}", line.p2().x()).as_str())
                    .attr("y2", format!("{}", line.p2().y()).as_str());

                Some(SvgElement::from(
                    self.0
                        .append_child(&e.0)
                        .unwrap()
                        .dyn_into::<web_sys::Element>()
                        .unwrap(),
                ))
            }
            crate::svg::SvgElement::Polygon(polygon) => {
                let mut e = SvgElement::new("polygon");

                let mut points = String::new();
                for v in polygon.points().iter() {
                    points += format!("{},{} ", v.x(), v.y()).as_str();
                }

                e.attr("points", points.as_str());
                /*
                e.id(polygon.id().as_str())
                    .class(polygon.class().as_str())
                    .attr("points", points.as_str());
                    */

                Some(SvgElement::from(
                    self.0
                        .append_child(&e.0)
                        .unwrap()
                        .dyn_into::<web_sys::Element>()
                        .unwrap(),
                ))
            }
            crate::svg::SvgElement::Polyline(polyline) => {
                let mut e = SvgElement::new("polyline");

                let mut points = String::new();
                for v in polyline.points().iter() {
                    points += format!("{},{} ", v.x(), v.y()).as_str();
                }

                e.id(polyline.id().as_str())
                    .class(polyline.class().as_str())
                    .attr("points", points.as_str());

                Some(SvgElement::from(
                    self.0
                        .append_child(&e.0)
                        .unwrap()
                        .dyn_into::<web_sys::Element>()
                        .unwrap(),
                ))
            }
            crate::svg::SvgElement::String(s) => {
                let e = Element::new(s);

                Some(SvgElement::from(
                    self.0
                        .append_child(&e.0)
                        .unwrap()
                        .dyn_into::<web_sys::Element>()
                        .unwrap(),
                ))
            }
        }
    }
    */

    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self {
        self.0.set_attribute(name, value).unwrap();
        self
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.0.set_attribute("id", id).unwrap();
        self
    }

    pub fn class(&mut self, class: &str) -> &mut Self {
        // Todo: think about adding a class instead of overwriting an existing class
        self.0.set_attribute("class", class).unwrap();
        self
    }
}

/*
impl super::Selection for Element {
    fn select(&self, s: &str) -> Option<SvgElement> {
        Some(SvgElement::from(self.0.query_selector(s).unwrap().unwrap()))
    }

    fn select_all(&self, s: &str) -> Option<Nodes> {
        let mut nodes = Nodes::new();
        let nl = self.0.query_selector_all(s).unwrap();
        let mut i = 0;
        while i < nl.length() {
            nodes.list.push(Element::from(
                nl.item(i).unwrap().dyn_into::<web_sys::Element>().unwrap(),
            ));
            i += 1;
        }
        Some(nodes)
    }

    fn append(&self, s: &str) -> Option<SvgElement> {
        let e = SvgElement::new(s);

        Some(SvgElement::from(
            self.0
                .append_child(&e)
                .unwrap()
              .dyn_into::<web_sys::Element>()
              .unwrap()
        ))
    }
}
*/

impl From<web_sys::Element> for Element {
    fn from(element: web_sys::Element) -> Element {
        Element(element)
    }
}

impl From<web_sys::SvgsvgElement> for Element {
    fn from(element: web_sys::SvgsvgElement) -> Element {
        Element(element.dyn_into::<web_sys::Element>().unwrap())
    }
}

impl Into<web_sys::SvgElement> for Element {
    fn into(self) -> web_sys::SvgElement {
        self.0.dyn_into::<web_sys::SvgElement>().unwrap()
    }
}

impl Into<web_sys::SvggElement> for Element {
    fn into(self) -> web_sys::SvggElement {
        self.0.dyn_into::<web_sys::SvggElement>().unwrap()
    }
}

impl Into<web_sys::SvgsvgElement> for Element {
    fn into(self) -> web_sys::SvgsvgElement {
        self.0.dyn_into::<web_sys::SvgsvgElement>().unwrap()
    }
}

/*
impl From<SvgCanvas> for Element {
  fn from(canvas: SvgCanvas) -> Self {
    Element(canvas.into())
  }
}
*/

use wasm_bindgen::JsCast;

use crate::web::{Document, SvgCanvas, OldTypeWrapper, Elem};
use std::ops::Deref;

#[derive(Debug)]
pub struct Element {
    n: web_sys::Element,
}

impl Element {
    pub fn create(name: &str) -> Element {
        let doc = Document::new().unwrap();
        doc.create(name, None)
    }
    
    pub fn new(n: web_sys::Element) -> Element {
        Element { n }
    }
    
    pub fn append(&self, element: Element) -> Element {
        let n = self.n.append_child(&element.n).unwrap().dyn_into::<web_sys::Element>().unwrap();
        Element { n }
    }

    pub fn html(&self, s: &str) {
        self.n.set_inner_html(s);
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
        self.n.set_attribute(name, value).unwrap();
        self
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.n.set_attribute("id", id).unwrap();
        self
    }

    pub fn class(&mut self, class: &str) -> &mut Self {
        // Todo: think about adding a class instead of overwriting an existing class
        self.n.set_attribute("class", class).unwrap();
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

impl OldTypeWrapper for Element {
  //type OldType = web_sys::Element;
  
  fn get(self) -> web_sys::Element {
    self.n
  }
}

impl<T: OldTypeWrapper + Clone> Elem<T> for Element {
  fn append(self, element: T) {
    let n = self.n.append_child(&element.get()).unwrap().dyn_into::<web_sys::Element>().unwrap();
  }
}

impl Into<web_sys::SvgsvgElement> for Element {
    fn into(self) -> web_sys::SvgsvgElement {
        self.n.dyn_into::<web_sys::SvgsvgElement>().unwrap()
    }
    
}

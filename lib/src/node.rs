use wasm_bindgen::JsCast;

use crate::element::*;

#[derive(Debug)]
pub struct Node(web_sys::Node);

impl Node {
    pub fn from(n: web_sys::Node) -> Node {
        Node(n)
    }
    
    pub fn attr(self, name: &str, value: &str) -> Element {
        let mut e = Element::from(self.0.dyn_into::< web_sys::Element >().unwrap());
        e.attr(name, value);
        e
    }
}

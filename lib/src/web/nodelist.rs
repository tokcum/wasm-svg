use crate::web::element::Element;
use crate::web::node::Node;

pub struct NodeList(web_sys::NodeList);

impl NodeList {
  pub fn from(nl: web_sys::NodeList) -> NodeList {
    NodeList(nl)
  }
  
  pub fn item(&self, i: u32) -> Node {
    Node::from(self.0.item(i).unwrap())
  }
  
  pub fn length(&self) -> u32 {
    self.0.length()
  }
}

pub struct Nodes {
  pub list: Vec<Element>,
  pub data: Vec<i32>,
}

impl Nodes {
  pub fn new() -> Self {
    Nodes {list: Vec::new(), data: Vec::new()}
  }
  
  pub fn data(&mut self, array: &[i32]) -> &mut Self {
    for i in array.iter() {
      self.data.push(*i)
    }
    self
  }
  
  pub fn enter(&mut self) -> &mut Self {
    let mut i = 0;
   
    while i < self.list.len() {
      let e = self.list.get_mut(i).unwrap();
      e.attr("r", format!("{}", self.data.get(i as usize).unwrap()).as_str());
      i += 1;
    }
    self
  }
}

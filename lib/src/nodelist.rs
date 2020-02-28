use wasm_bindgen::JsCast;


pub struct Nodes {
  pub list: Option<NodeList>,
  pub data: Vec<i32>,
}

impl Nodes {
  pub fn new() -> Self {
    Nodes {list: None, data: Vec::new()}
  }
  
  pub fn data(mut self, array: &[i32]) -> Self {
    for i in array.iter() {
      self.data.push(*i)
    }
    self
  }
}

pub struct NodeList(pub web_sys::NodeList);

impl NodeList {
  pub fn from(nl: web_sys::NodeList) -> NodeList {
    NodeList(nl)
  }
}

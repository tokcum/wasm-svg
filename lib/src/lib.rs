use wasm_bindgen::JsCast;

//Using the Newtype Pattern to Implement External Traits on External Types
//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types

pub struct Document(web_sys::Document);

impl Document {
  pub fn new() -> Document {
    let w = web_sys::window().expect("no global `window` exists");
    let d = w.document().expect("should have a document on window");
    Document(d)
  }
}

pub struct Node(web_sys::Node);

impl Node {
  pub fn from(n: web_sys::Node) -> Node {
    Node(n)
  }
}

pub struct Element(web_sys::Element);

impl Element {
  fn create() -> Element {
    let d = Document::new();
    Element(d.0.create_element("none").unwrap())
  }
  
  pub fn new(node: &str) -> Element {
    let d = Document::new();
    Element(d.0.create_element_ns(Some("http://www.w3.org/2000/svg"), node).unwrap())
  }
  
  pub fn new_svg() -> Element {
    let d = Document::new();
    Element(d.0.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg").unwrap())
  }
  
  pub fn append_svg(&self, s: &str) -> Option<Element> {
    let e = Element::new_svg();
    
    Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
  }
  
  pub fn attr(&mut self, name: &str, value: &str) -> &mut Element {
    self.0.set_attribute(name, value);
    self
  }
  
  pub fn from(e: web_sys::Element) -> Element {
    Element(e)
  }
  
  pub fn html(&self, s: &str) {
    self.0.set_inner_html(s);
  }
}

pub trait Selection {
  fn select(&self, s: &str) -> Option<Element>;
  fn append(&self, s: &str) -> Option<Element>;
}

impl Selection for Document {
  fn select (&self, s: &str) -> Option<Element> {
    Some(Element::from(self.0.query_selector(s).unwrap().unwrap()))
  }
  fn append(&self, s: &str) -> Option<Element> {
    let e = Element::new(s);
    
    Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
  }
}

impl Selection for Element {
  fn select (&self, s: &str) -> Option<Element> {
    Some(Element::from(self.0.query_selector(s).unwrap().unwrap()))
  }
  fn append(&self, s: &str) -> Option<Element> {
    let e = Element::new(s);
    
    Some(Element::from(self.0.append_child(&e.0).unwrap().dyn_into::< web_sys::Element >().unwrap()))
  }
}

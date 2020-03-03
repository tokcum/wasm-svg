use crate::web::BasicElement;
use crate::web::SvgElement;

pub struct SvgGElement(pub web_sys::Element);

impl SvgGElement {
    pub fn new() -> SvgGElement {
        let mut e = SvgElement::new("g");
        SvgGElement(e.0)
    }
}

impl BasicElement for SvgGElement {
  fn id(&self) -> String {
    self.0.id()
  }
  
  fn set_id(&self, id: &str) -> &Self {
    self.0.set_id(id);
    self
  }
  
  fn class(&self) -> String {
    self.0.class_name()
  }
  
  fn set_class(&self, class: &str) -> &Self {
    self.0.set_class_name(class);
    self
  }
}

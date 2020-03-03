// Define web module's public interface
pub use document::Document;
pub use element::Element;
pub use node::Node;
pub use nodelist::{NodeList, Nodes};
pub use svg_canvas::SvgCanvas;
pub use svg_g_element::SvgGElement;
pub use svg_element::SvgElement;

// Define geometry's sub modules
mod document;
mod element;
mod node;
mod nodelist;
mod svg_canvas;
mod svg_element;
mod svg_g_element;
mod svg_polyline_element;

pub trait Selection {
  fn select(&self, s: &str) -> Option<Element>;
  fn select_all(&self, s: &str) -> Option<Nodes>;
  fn append(&self, s: &str) -> Option<Element>;
}

pub trait BasicElement {
  fn id(&self) -> String;
  
  fn set_id(&self, id: &str) -> &Self;
  
  fn class(&self) -> String;
  
  fn set_class(&self, class: &str) -> &Self;
}

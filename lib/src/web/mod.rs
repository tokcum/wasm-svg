// Define web module's public interface
pub use document::Document;
pub use element::Element;
pub use node::Node;
pub use nodelist::{NodeList, Nodes};

// Define geometry's sub modules
mod document;
mod element;
mod node;
mod nodelist;

pub trait Selection {
  fn select(&self, s: &str) -> Option<Element>;
  fn select_all(&self, s: &str) -> Option<Nodes>;
  fn append(&self, s: &str) -> Option<Element>;
}

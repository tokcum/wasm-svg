// Declare mod as public so example code has access to it
pub mod document;
pub mod element;
pub mod node;
pub mod nodelist;

// Shorten path to objects defined in it.
use crate::web::element::Element;
use crate::web::nodelist::Nodes;

pub trait Selection {
  fn select(&self, s: &str) -> Option<Element>;
  fn select_all(&self, s: &str) -> Option<Nodes>;
  fn append(&self, s: &str) -> Option<Element>;
}

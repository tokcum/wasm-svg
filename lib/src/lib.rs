// Declare mod as public so example code has access to it
pub mod document;
pub mod element;
pub mod node;
pub mod svg_circle_element;
pub mod color;
pub mod circle;

// Shorten path to objects defined in it.
use crate::element::*;

pub trait Selection {
  fn select(&self, s: &str) -> Option<Element>;
  fn append(&self, s: &str) -> Option<Element>;
}

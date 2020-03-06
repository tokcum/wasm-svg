// Define web module's public interface
pub use document::Document;
pub use element::Element;
//pub use node::Node;
//pub use nodelist::{NodeList, Nodes};
pub use svg_canvas::SvgCanvas;
use std::ops::Deref;
//pub use svg_g_element::SvgGElement;
//pub use svg_element::SvgElement;

// Define geometry's sub modules
mod document;
mod element;
//mod node;
//mod nodelist;
mod svg_canvas;
//mod svg_element;
//mod svg_g_element;
//mod svg_polyline_element;

pub enum Namespace {
    SVG,
    XHTML,
}

// Trait to access the real type behind a New Type (see new type pattern)
trait OldTypeWrapper {
  //type OldType;
  //fn get(self) -> Self::OldType;
  fn get(self) -> web_sys::Element;
}

trait Elem<T: OldTypeWrapper + Clone> {
  fn append(self, element: T);
}

/*
pub fn append(&self, element: Element) -> Element {
  let n = self.n.append_child(&element.n).unwrap().dyn_into::<web_sys::Element>().unwrap();
  Element { n }
}
*/
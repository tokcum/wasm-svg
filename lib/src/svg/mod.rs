// Define SVG module's public interface
pub use polygon::Polygon;
pub use position::Position;

mod class;
mod polygon;
mod position;

pub enum SvgElement {
  Polygon(Polygon),
  String(String),
}

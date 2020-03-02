// Define SVG module's public interface
pub use axis::Axis;
pub use line::Line;
pub use polygon::Polygon;
pub use polyline::Polyline;
pub use pos::Pos;

mod axis;
mod class;
mod interval;
mod line;
mod polygon;
mod polyline;
mod pos;
mod scale;

mod test;

pub enum SvgElement<'a> {
  Axis(&'a Axis),
  Line(&'a Line),
  Polyline(Polyline),
  Polygon(&'a Polygon),
  String(String),
}

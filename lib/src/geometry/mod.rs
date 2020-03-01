// Define geometry module's public interface
pub use circle::Circle;
pub use line::Line;
pub use point::Point;
pub use triangle::Triangle;

// Define geometry's sub modules
mod circle;
mod line;
mod point;
mod triangle;

mod test;

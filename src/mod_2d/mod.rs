//! 2D Geometry
//!
//! This module contains basic methods to manipulate 2D Geometry.

mod point;
mod line;
mod shape;
mod circle;

pub use point::Point2D;
pub use line::Line2D;
pub use circle::Circle2D;

pub use shape::HasArea;
use crate::mod_2d::Point2D;

/// Shape can calculate it's own area
pub trait HasArea{
    /// Gets area of shape
    fn area(&self) -> f32;
}

/// Shape can calculate it's own perimeter
pub trait HasPerimeter {
    /// Gets perimeter of shape
    fn perimeter(&self) -> f32;
}

/// Shape contains a point
pub trait ContainsPoint {
    /// Determines if point is within the shape
    fn contains(&self, other: &Point2D) -> bool;
}

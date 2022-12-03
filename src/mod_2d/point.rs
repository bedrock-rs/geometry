//! 2D Point
//!
//! This module contains basic methods to manipulate a 2D point

/// Generic 2D Point
///
/// Can be used with [`f32`] or [`f64`]
///
/// # Examples
///
/// ```no_run
/// use bdrk_geometry::mod_2d::Point2D;
///
/// let new_point = Point2D::new(0, 0);
/// ```
#[derive(Debug, Default)]
pub struct Point2D<T> {
    x: T,
    y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2D { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }
}

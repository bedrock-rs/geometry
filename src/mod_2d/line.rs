//! 2D Line
//!
//! This module contains basic methods to manipulate 2D lines

use crate::mod_2d::point::Point2D;

/// Generic 2D Line
#[derive(Debug, Default)]
pub struct Line2D {
    start: Point2D,
    end: Point2D,
}

impl Line2D {
    /// Create 2D Line
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::{Line2D, Point2D};
    ///
    /// let point_one = Point2D::new(0f32, 0f32);
    /// let point_two = Point2D::new(1f32, 1f32);
    ///
    /// let new_line = Line2D::new(point_one, point_two);
    /// ```
    pub fn new(start: Point2D, end: Point2D) -> Self {
        Line2D { start, end }
    }

    /// Gets distance of 2D Line
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f32::consts::SQRT_2;
    /// use bdrk_geometry::mod_2d::{Line2D, Point2D};
    ///
    /// let point_one = Point2D::new(0f32, 0f32);
    /// let point_two = Point2D::new(1f32, 1f32);
    ///
    /// let new_line = Line2D::new(point_one, point_two);
    ///
    /// assert_eq!(new_line.distance(), SQRT_2)
    /// ```
    pub fn distance(&self) -> f32 {
        let diff_x = *self.end.x() - *self.start.x();
        let diff_y = *self.end.y() - *self.start.y();

        (diff_x.powi(2) + diff_y.powi(2)).sqrt()
    }
}

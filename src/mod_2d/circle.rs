use std::f32::consts::PI;
use crate::mod_2d::{HasArea, Point2D};

/// Generic 2D Circle
pub struct Circle2D {
    center: Point2D,
    radius: f32
}

impl Circle2D {
    /// Create 2D Circle
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::{Circle2D, Point2D};
    ///
    /// let center_point = Point2D::new(0f32, 0f32);
    ///
    /// let new_circle = Circle2D::new(center_point, 2f32);
    /// ```
    pub fn new(center: Point2D, radius: f32) -> Self {
        Circle2D {
            center,
            radius
        }
    }

    /// Gets coordinate reference of circle center point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::{Circle2D, Point2D};
    ///
    /// let center_point = Point2D::new(0f32, 0f32);
    ///
    /// let new_circle = Circle2D::new(center_point, 2f32);
    ///
    /// assert_eq!(*new_circle.center(), center_point);
    /// ```
    pub fn center(&self) -> &Point2D {
        &self.center
    }

    /// Gets mutable coordinate reference of circle center point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::{Circle2D, Point2D};
    ///
    /// let center_point = Point2D::new(0f32, 0f32);
    ///
    /// let mut new_circle = Circle2D::new(center_point, 2f32);
    ///
    /// new_circle.center_mut().set(1f32, 1f32);
    ///
    /// assert_eq!(new_circle.center(), &Point2D::new(1f32, 1f32));
    /// ```
    pub fn center_mut(&mut self) -> &mut Point2D {
        &mut self.center
    }

    /// Gets reference to circle radius
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::{Circle2D, Point2D};
    ///
    /// let center_point = Point2D::new(0f32, 0f32);
    ///
    /// let new_circle = Circle2D::new(center_point, 2f32);
    ///
    /// assert_eq!(*new_circle.radius(), 2f32);
    /// ```
    pub fn radius(&self) -> &f32 {
        &self.radius
    }

    /// Gets mutable reference to circle radius
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::{Circle2D, Point2D};
    ///
    /// let center_point = Point2D::new(0f32, 0f32);
    ///
    /// let mut new_circle = Circle2D::new(center_point, 2f32);
    ///
    /// *new_circle.radius_mut() = 3f32;
    ///
    /// assert_eq!(*new_circle.radius(), 3f32);
    /// ```
    pub fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
}

impl HasArea for Circle2D {
    /// Gets area of circle
    fn area(&self) -> f32 {
        PI * self.radius.powi(2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        use std::f32::consts::PI;

        let center_point = Point2D::new(0f32, 0f32);
        let radius = 1f32;

        let new_circle = Circle2D::new(center_point, radius);

        assert_eq!(new_circle.area(), PI * radius.powi(2));
    }
}
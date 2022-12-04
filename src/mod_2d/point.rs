//! 2D Point
//!
//! This module contains basic methods to manipulate a 2D point

/// Generic 2D Point
#[derive(Copy, Clone, Debug, Default)]
pub struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    /// Create Generic 2D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::Point2D;
    ///
    /// let new_point = Point2D::new(0f32, 0f32);
    ///
    /// assert_eq!(new_point.x(), &0f32);
    /// assert_eq!(new_point.y(), &0f32);
    /// ```
    pub fn new(x: f32, y: f32) -> Self {
        Point2D { x, y }
    }

    /// Create Generic 2D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::Point2D;
    ///
    /// let mut new_point = Point2D::new(0f32, 0f32);
    ///
    /// new_point.set(1f32, 1f32);
    ///
    /// assert_eq!(new_point.x(), &1f32);
    /// assert_eq!(new_point.y(), &1f32);
    /// ```
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    /// Gets X coordinate reference of 2D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::Point2D;
    ///
    /// let new_point = Point2D::new(0f32, 0f32);
    ///
    /// assert_eq!(new_point.x(), &0f32);
    /// ```
    pub fn x(&self) -> &f32 {
        &self.x
    }

    /// Gets Y coordinate reference of 2D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::Point2D;
    ///
    /// let new_point = Point2D::new(0f32, 0f32);
    ///
    /// assert_eq!(new_point.y(), &0f32);
    /// ```
    pub fn y(&self) -> &f32 {
        &self.y
    }

    /// Gets mutable X coordinate reference of 2D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::Point2D;
    ///
    /// let mut new_point = Point2D::new(0f32, 0f32);
    ///
    /// *new_point.x_mut() = 2f32;
    ///
    /// assert_eq!(new_point.x(), &2f32);
    /// ```
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }

    /// Gets mutable Y coordinate reference of 2D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_2d::Point2D;
    ///
    /// let mut new_point = Point2D::new(0f32, 0f32);
    ///
    /// *new_point.y_mut() = 2f32;
    ///
    /// assert_eq!(new_point.y(), &2f32);
    /// ```
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
}

impl PartialEq<Self> for Point2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point2D { }
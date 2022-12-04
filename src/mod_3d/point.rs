//! 3D Point
//!
//! This module contains basic methods to manipulate a 3D point

/// Generic 3D Point
#[derive(Debug, Default)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32
}

impl Point3D {
    /// Create Generic 3D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_3d::Point3D;
    ///
    /// let new_point = Point3D::new(0f32, 0f32, 0f32);
    ///
    /// assert_eq!(new_point.x(), &0f32);
    /// assert_eq!(new_point.y(), &0f32);
    /// assert_eq!(new_point.z(), &0f32);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3D { x, y, z }
    }

    /// Gets X coordinate reference of 3D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_3d::Point3D;
    ///
    /// let new_point = Point3D::new(0f32, 0f32, 0f32);
    ///
    /// assert_eq!(new_point.x(), &0f32);
    /// ```
    pub fn x(&self) -> &f32 {
        &self.x
    }

    /// Gets Y coordinate reference of 3D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_3d::Point3D;
    ///
    /// let new_point = Point3D::new(0f32, 0f32, 0f32);
    ///
    /// assert_eq!(new_point.y(), &0f32);
    /// ```
    pub fn y(&self) -> &f32 {
        &self.y
    }

    /// Gets Z coordinate reference of 3D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_3d::Point3D;
    ///
    /// let new_point = Point3D::new(0f32, 0f32, 0f32);
    ///
    /// assert_eq!(new_point.z(), &0f32);
    /// ```
    pub fn z(&self) -> &f32 {
        &self.z
    }

    /// Gets mutable X coordinate reference of 3D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_3d::Point3D;
    ///
    /// let mut new_point = Point3D::new(0f32, 0f32, 0f32);
    ///
    /// *new_point.x_mut() = 2f32;
    ///
    /// assert_eq!(new_point.x(), &2f32);
    /// ```
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }


    /// Gets mutable Y coordinate reference of 3D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_3d::Point3D;
    ///
    /// let mut new_point = Point3D::new(0f32, 0f32, 0f32);
    ///
    /// *new_point.y_mut() = 2f32;
    ///
    /// assert_eq!(new_point.y(), &2f32);
    /// ```
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }


    /// Gets mutable Z coordinate reference of 3D Point
    ///
    /// # Examples
    ///
    /// ```
    /// use bdrk_geometry::mod_3d::Point3D;
    ///
    /// let mut new_point = Point3D::new(0f32, 0f32, 0f32);
    ///
    /// *new_point.z_mut() = 2f32;
    ///
    /// assert_eq!(new_point.z(), &2f32);
    /// ```
    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
    }
}
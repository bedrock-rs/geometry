//! 3D Point
//!
//! This module contains basic methods to manipulate a 3D point

//! 3D Point
//!
//! This module contains basic methods to manipulate a 3D point

/// Generic 3D Point
///
/// Can be used with [`f32`] or [`f64`]
///
/// # Examples
///
/// ```no_run
/// use bdrk_geometry::mod_3d::Point3D;
///
/// let new_point = Point3D::new(0, 0, 0);
/// ```
#[derive(Debug, Default)]
pub struct Point3D<T> {
    x: T,
    y: T,
    z: T
}

impl<T> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3D { x, y, z }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn z(&self) -> &T {
        &self.z
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self.z
    }
}
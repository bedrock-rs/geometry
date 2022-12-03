//! 3D Point
//!
//! This module contains basic methods to manipulate a 3D point

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
//! 3D Line
//!
//! This module contains basic methods to manipulate 3D lines

use crate::mod_3d::point::Point3D;

use num_traits::Float;

#[derive(Debug, Default)]
pub struct Line3D<T> {
    start: Point3D<T>,
    end: Point3D<T>,
}

impl<T: Float> Line3D<T> {
    pub fn new(start: Point3D<T>, end: Point3D<T>) -> Self {
        Line3D { start, end }
    }

    pub fn distance(&self) -> T {
        let diff_x: T = *self.end.x() - *self.start.x();
        let diff_y: T = *self.end.y() - *self.start.y();
        let diff_z: T = *self.end.z() - *self.start.z();

        (diff_x.powi(2) + diff_y.powi(2) + diff_z.powi(2)).sqrt()
    }
}

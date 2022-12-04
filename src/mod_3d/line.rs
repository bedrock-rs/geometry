//! 3D Line
//!
//! This module contains basic methods to manipulate 3D lines

use crate::mod_3d::point::Point3D;

#[derive(Debug, Default)]
pub struct Line3D {
    start: Point3D,
    end: Point3D,
}

impl Line3D {
    pub fn new(start: Point3D, end: Point3D) -> Self {
        Line3D { start, end }
    }

    pub fn distance(&self) -> f32 {
        let diff_x = *self.end.x() - *self.start.x();
        let diff_y = *self.end.y() - *self.start.y();
        let diff_z = *self.end.z() - *self.start.z();

        (diff_x.powi(2) + diff_y.powi(2) + diff_z.powi(2)).sqrt()
    }
}

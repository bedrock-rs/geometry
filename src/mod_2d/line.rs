//! 2D Line
//!
//! This module contains basic methods to manipulate 2D lines

use crate::mod_2d::point::Point2D;

#[derive(Debug, Default)]
pub struct Line2D {
    start: Point2D,
    end: Point2D
}

impl Line2D {
    pub fn new(start: Point2D, end: Point2D ) -> Self {
        Line2D {
            start,
            end
        }
    }

    pub fn distance(&self) -> f32 {
        let diff_x: f32 = *self.end.x() - *self.start.x();
        let diff_y: f32 = *self.end.y() - *self.start.y();

        (diff_x.powi(2) + diff_y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let point_1 = Point2D::new(0.0, 0.0);
        let point_2 = Point2D::new(1.0, 1.0);

        let line = Line2D::new(point_1, point_2);

        assert_eq!(line.distance(), 2.0_f32.sqrt());
    }
}
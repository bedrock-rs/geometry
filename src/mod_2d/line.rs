//! 2D Line
//!
//! This module contains basic methods to manipulate 2D lines

use crate::mod_2d::point::Point2D;

use num_traits::Float;

#[derive(Debug, Default)]
pub struct Line2D<T> {
    start: Point2D<T>,
    end: Point2D<T>,
}

impl<T: Float> Line2D<T> {
    pub fn new(start: Point2D<T>, end: Point2D<T>) -> Self {
        Line2D { start, end }
    }

    pub fn distance(&self) -> T {
        let diff_x: T = *self.end.x() - *self.start.x();
        let diff_y: T = *self.end.y() - *self.start.y();

        (diff_x.powi(2) + diff_y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let point_1: Point2D<f32> = Point2D::new(0.0, 0.0);
        let point_2: Point2D<f32> = Point2D::new(1.0, 1.0);

        let line: Line2D<f32> = Line2D::new(point_1, point_2);

        assert_eq!(line.distance(), 2.0_f32.sqrt());
    }
}

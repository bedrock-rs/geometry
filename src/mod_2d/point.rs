//! 2D Point
//!
//! This module contains basic methods to manipulate a 2D point

#[derive(Debug, Default)]
pub struct Point2D {
    x: f32,
    y: f32
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Point2D {
            x,
            y
        }
    }

    pub fn x(&self) -> &f32 {
        &self.x
    }

    pub fn y(&self) -> &f32 {
        &self.y
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x() {
        let new_point = Point2D::default();

        assert_eq!(new_point.x, 0.0);
    }

    #[test]
    fn y() {
        let new_point = Point2D::default();

        assert_eq!(new_point.y, 0.0);
    }

    #[test]
    fn x_mut() {
        let mut new_point = Point2D::default();

        *new_point.x_mut() = 2.0;

        assert_eq!(new_point.x, 2.0);
    }

    #[test]
    fn y_mut() {
        let mut new_point = Point2D::default();

        *new_point.y_mut() = 2.0;

        assert_eq!(new_point.y, 2.0);
    }
}
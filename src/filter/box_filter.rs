use super::Filter;
use cgmath::Point2;

/// A box filter that simply returns 1 for all values inside the filter and 0
/// for all values outside the filter.
pub struct BoxFilter {
    half_width: f32,
    half_height: f32,
}

impl BoxFilter {
    pub fn new(half_width: f32, half_height: f32) -> Self {
        Self {
            half_width,
            half_height,
        }
    }
}

impl Filter for BoxFilter {
    fn eval_at(&self, p: Point2<f32>) -> f32 {
        if p.x.abs() <= self.half_width && p.y.abs() <= self.half_height {
            1.0
        } else {
            0.0
        }
    }

    fn half_width(&self) -> f32 {
        self.half_width
    }

    fn half_height(&self) -> f32 {
        self.half_height
    }
}

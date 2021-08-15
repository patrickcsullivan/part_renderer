use super::Filter;
use cgmath::Point2;

pub struct MitchellFilter {
    half_width: f32,
    half_height: f32,

    inv_half_width: f32,
    inv_half_height: f32,

    b: f32,
    c: f32,
}

impl MitchellFilter {
    pub fn new(half_width: f32, half_height: f32, b: f32, c: f32) -> Self {
        Self {
            half_width,
            half_height,
            inv_half_width: 1.0 / half_width,
            inv_half_height: 1.0 / half_height,
            b,
            c,
        }
    }

    fn mitchell_1d(&self, x: f32) -> f32 {
        let x = (2.0 * x).abs();
        if x > 1.0 {
            ((-1.0 * self.b - 6.0 * self.c) * x * x * x
                + (6.0 * self.b + 30.0 * self.c) * x * x
                + (-12.0 * self.b - 48.0 * self.c) * x
                + (8.0 * self.b + 24.0 * self.c))
                * (1.0 / 6.0)
        } else {
            ((12.0 - 9.0 * self.b - 6.0 * self.c) * x * x * x
                + (-18.0 + 12.0 * self.b + 6.0 * self.c) * x * x
                + (6.0 - 2.0 * self.b))
                * (1.0 / 6.0)
        }
    }
}

impl Filter for MitchellFilter {
    fn eval_at(&self, p: Point2<f32>) -> f32 {
        if p.x.abs() <= self.half_width && p.y.abs() <= self.half_height {
            self.mitchell_1d(p.x * self.inv_half_width)
                * self.mitchell_1d(p.y * self.inv_half_height)
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

use cgmath::Point2;

pub trait Filter {
    /// Takes a point whose position is relative to the center of the filter at
    /// (0, 0) and returns the weight of that point.
    fn evaluate(&self, p: &Point2<f32>) -> f32;
}

pub struct BoxFilter {
    x_radius: f32,
    y_radius: f32,
}

impl BoxFilter {
    pub fn new(x_radius: f32, y_radius: f32) -> Self {
        Self { x_radius, y_radius }
    }
}

impl Filter for BoxFilter {
    fn evaluate(&self, p: &Point2<f32>) -> f32 {
        if p.x.abs() < self.x_radius && p.y.abs() < self.y_radius {
            1.0
        } else {
            0.0
        }
    }
}

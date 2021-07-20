use cgmath::Vector2;

/// Models the sensing device in a simulated camera. Acts as a 2D plane of
/// pixels onto which a final image is rendered.
pub struct Film {
    // The images resoltion in pixels.
    pub resolution: Vector2<f32>,
}

impl Film {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            resolution: Vector2::new(x as f32, y as f32),
        }
    }
}

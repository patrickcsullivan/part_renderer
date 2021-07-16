use cgmath::{Point2, Vector2};

/// Container for all the information needed to generate a ray from a cameraa.
#[derive(Debug, Clone, Copy)]
pub struct CameraSample {
    /// The point on the film in raster space to which a generated ray will
    /// carry radiance.
    pub p_film: Point2<f32>,

    /// The point on the lense that a generated ray will pass through. This is
    /// only relevent for cameras models that include lenses.
    pub p_lens: Point2<f32>,

    /// The time at which a ray should sample the scene.
    pub time: f32,
}

impl CameraSample {
    pub fn new(p_film: Point2<f32>, p_lens: Point2<f32>, time: f32) -> Self {
        Self {
            p_film,
            p_lens,
            time,
        }
    }

    pub fn from_film_shift(&self, film_shift: Vector2<f32>) -> Self {
        Self {
            p_film: self.p_film + film_shift,
            p_lens: self.p_lens,
            time: self.time,
        }
    }
}

use cgmath::{Point2, Vector2};

/// Container for all the information needed to generate a ray from a cameraa.
#[derive(Debug, Clone, Copy)]
pub struct CameraSample {
    /// The point on the film in raster space to which a generated ray will
    /// carry radiance.
    pub film_point: Point2<f32>,

    /// The point on the lense that a generated ray will pass through. This is
    /// only relevent for cameras models that include lenses.
    pub lens_point: Point2<f32>,

    /// The time at which a ray should sample the scene.
    pub time: f32,
}

impl CameraSample {
    pub fn new(film_point: Point2<f32>, lens_point: Point2<f32>, time: f32) -> Self {
        Self {
            film_point,
            lens_point,
            time,
        }
    }

    pub fn from_film_shift(&self, film_shift: Vector2<f32>) -> Self {
        Self {
            film_point: self.film_point + film_shift,
            lens_point: self.lens_point,
            time: self.time,
        }
    }

    pub fn at_pixel_center(pixel: Point2<i32>) -> Self {
        Self {
            film_point: Point2::new(pixel.x as f32 + 0.5, pixel.y as f32 + 0.5),
            lens_point: Point2::new(0.0, 0.0), // TODO
            time: 0.0,                         // TODO
        }
    }
}

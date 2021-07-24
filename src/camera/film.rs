use super::pixel::Pixel;
use crate::{filter::Filter, geometry::bounds::Bounds2};
use cgmath::{Point2, Vector2};

/// Models the sensing device in a simulated camera. Acts as a 2D plane of
/// pixels onto which a final image is rendered.
pub struct Film {
    // The images resoltion in pixels.
    pub resolution: Vector2<usize>,

    /// The bounds of the image in raster space.
    ///
    /// Note that `raster_bounds.min` is the point `(0, 0)` at the top-left
    /// corner of the top-left pixel, whose index is `(0, 0)`.
    /// `raster_bounds.max` is the point `(resolution.x, resolution.y)` at the
    /// bottom right corner of the bottom-right pixel, whose index is
    /// `(resolution.x - 1, resolution.y - 1)`. The potentially confusing part
    /// here is that `raster_bounds.max` is a point in raster space, and it is
    /// NOT the index of the bottom-right pixel, as one might mistakenly expect.
    pub raster_bounds: Bounds2<usize>,

    pub pixels: Vec<Pixel>,

    pub filter: Box<dyn Filter>,
}

impl Film {
    pub fn new(x: usize, y: usize, filter: Box<dyn Filter>) -> Self {
        // TODO: If I'm not supporting cropped areas, maybe I don't even need
        // `raster_bounds`.
        let raster_bounds = Bounds2::new(Point2::new(0, 0), Point2::new(x, y));

        let pixels = vec![Pixel::default(); x * y];

        Self {
            resolution: Vector2::new(x, y),
            raster_bounds,
            pixels,
            filter,
        }
    }

    /// Return the range of pixel (raster space) values for the image for which
    /// a `Sampler` will be responsible for generating samples.
    ///
    /// Since the pixel reconstruction performed by a `Filter` takes values from
    /// a kernel, it's important that we generate samples for "pixels" that are
    /// outside of the image's bounds. If we don't, then when pixels at the
    /// image edge are reconstructed by a `Filter`, they will be biased towards
    /// the inner pixels.
    ///
    /// Note that the returned bounds are points in raster space; they are not
    /// pixel indices.
    pub fn image_sample_bounds(&self) -> Bounds2<usize> {
        let top_left_pixel_center = Point2::new(
            self.raster_bounds.min.x as f32 + 0.5,
            self.raster_bounds.min.y as f32 + 0.5,
        );
        let bottom_right_pixel_center = Point2::new(
            self.raster_bounds.max.x as f32 - 0.5,
            self.raster_bounds.max.y as f32 - 0.5,
        );

        let min = Point2::new(
            (top_left_pixel_center.x - self.filter.half_width()).floor() as usize,
            (top_left_pixel_center.y - self.filter.half_width()).floor() as usize,
        );
        let max = Point2::new(
            (bottom_right_pixel_center.x + self.filter.half_width()).ceil() as usize,
            (bottom_right_pixel_center.y + self.filter.half_width()).ceil() as usize,
        );

        Bounds2::new(min, max)
    }
}

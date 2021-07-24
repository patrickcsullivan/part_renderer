mod tile;

pub use tile::FilmTile;

use crate::{color::Xyz, geometry::bounds::Bounds2};
use cgmath::{Point2, Vector2};

/// Models the sensing device in a simulated camera. Acts as a 2D plane of
/// pixels onto which a final image is rendered.
pub struct Film {
    // The images resoltion in pixels.
    pub resolution: Vector2<usize>,

    /// A bounding box around the film's pixels in raster space.
    ///
    /// Note that `raster_bounds.min` is the point `(0, 0)` at the top-left
    /// corner of the top-left pixel, whose index is `(0, 0)`.
    /// `raster_bounds.max` is the point `(resolution.x, resolution.y)` at the
    /// bottom right corner of the bottom-right pixel, whose index is
    /// `(resolution.x - 1, resolution.y - 1)`. The potentially confusing part
    /// here is that `raster_bounds.max` is a point in raster space, and it is
    /// NOT the index of the bottom-right pixel, as one might mistakenly expect.
    pub pixel_bounds: Bounds2<i32>,

    pixels: Vec<FilterPixel>,
}

impl Film {
    pub fn new(x: usize, y: usize) -> Self {
        let pixel_bounds = Bounds2::new(Point2::new(0, 0), Point2::new(x as i32, y as i32));
        let pixels = vec![FilterPixel::default(); x * y];

        Self {
            resolution: Vector2::new(x, y),
            pixel_bounds,
            pixels,
        }
    }

    /// Return a bounding box around the film's pixels in raster space that a
    /// `Sampler` will be responsible for generating samples for.
    ///
    /// Since the pixel reconstruction performed by a `Filter` takes values from
    /// a kernel, it's important that we generate samples for "pixels" that are
    /// outside of the image's bounds. If we don't, then when pixels at the
    /// image edge are reconstructed by a `Filter`, they will be biased towards
    /// the inner pixels.
    pub fn sample_bounds(&self, filter_half_width: f32, filter_half_height: f32) -> Bounds2<i32> {
        let top_left_pixel_center = Point2::new(
            self.pixel_bounds.min.x as f32 + 0.5,
            self.pixel_bounds.min.y as f32 + 0.5,
        );
        let bottom_right_pixel_center = Point2::new(
            self.pixel_bounds.max.x as f32 - 0.5,
            self.pixel_bounds.max.y as f32 - 0.5,
        );

        let min = Point2::new(
            (top_left_pixel_center.x - filter_half_width).floor() as i32,
            (top_left_pixel_center.y - filter_half_height).floor() as i32,
        );
        let max = Point2::new(
            (bottom_right_pixel_center.x + filter_half_width).ceil() as i32,
            (bottom_right_pixel_center.y + filter_half_height).ceil() as i32,
        );

        Bounds2::new(min, max)
    }

    /// Return a film tile containing the subset of the film's pixels that
    /// samples taken from `sample_bounds` will contribute to.
    ///
    /// * sample_bounds - Bounding box of a pixel area (in raster space) that
    ///   samples will be generated in.
    /// * filter_half_width
    /// * filter_half_height
    pub fn tile(
        &self,
        sample_bounds: Bounds2<i32>,
        filter_half_width: f32,
        filter_half_height: f32,
    ) -> Option<FilmTile> {
        self.pixel_bounds_for_sample_bounds(sample_bounds, filter_half_width, filter_half_height)
            .map(FilmTile::new)
    }

    /// Return a bounding box around the pixels (in raster space) that samples
    /// taken from `sample_bounds` will contribute to.
    ///
    /// Since an individual sample can contribute to multiple pixels, the
    /// returned pixel bounds can exceed `sample_bounds`. On the other hand,
    /// since the returned pixel bounds should only contain pixels that are
    /// actually on the film and since the given `sample_bounds` can extend
    /// beyond the film, the returned pixel bounds may be smaller than
    /// `sample_bounds`.
    ///
    /// * sample_bounds - Bounding box of a pixel area (in raster space) that
    ///   samples will be generated in.
    /// * filter_half_width
    /// * filter_half_height
    fn pixel_bounds_for_sample_bounds(
        &self,
        sample_bounds: Bounds2<i32>,
        filter_half_width: f32,
        filter_half_height: f32,
    ) -> Option<Bounds2<i32>> {
        let min = Point2::new(
            (sample_bounds.min.x as f32 - 0.5 - filter_half_width).ceil() as i32,
            (sample_bounds.min.y as f32 - 0.5 - filter_half_height).ceil() as i32,
        );
        let max = Point2::new(
            (sample_bounds.max.x as f32 - 0.5 + filter_half_width).floor() as i32 + 1,
            (sample_bounds.max.y as f32 - 0.5 + filter_half_height).floor() as i32 + 1,
        );
        let possible_pixel_bounds = Bounds2::new(min, max);

        // Clip the possible pixel bounds to only include pixels that are
        // actually on the film.
        possible_pixel_bounds.intersect(&self.pixel_bounds)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct FilterPixel {
    /// The color at the pixel in the XYZ color space.
    xyz: Xyz,

    filter_weight_sum: f32,
}

impl Default for FilterPixel {
    fn default() -> Self {
        Self {
            xyz: Xyz::black(),
            filter_weight_sum: 0.0,
        }
    }
}

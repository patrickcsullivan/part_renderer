use cgmath::{Point2, Vector2};

use crate::{color::RgbaSpectrum, filter::Filter, geometry::bounds::Bounds2};

/// Stores the pixel data for a subset of a larger `Film`. A `FilmTile` can be
/// merged into a `Film` to add its pixel contributions to the `Film`.
pub struct FilmTile {
    /// A bounding box around the pixels (in raster space) that the tile
    /// contains.
    pub pixel_bounds: Bounds2<i32>,

    pub pixels: Vec<FilmTilePixel>,
}

impl FilmTile {
    pub fn new(pixel_bounds: Bounds2<i32>) -> Self {
        let pixels = vec![FilmTilePixel::default(); pixel_bounds.area().max(0) as usize];
        Self {
            pixel_bounds,
            pixels,
        }
    }

    /// Add the radiance from a sample to the pixels in the tile.
    ///
    /// * sample_film_point - Location of the sample on the film in raster
    ///   space.
    /// * radiance - The incoming radiance along the ray whose origin is at the
    ///   `sample_film_point`.
    /// * sample_weight - Weight that indicates how much the radiance from the
    ///   given sample contributes to the final image relative to the radiance
    ///   from other samples.
    /// * filter - The filter used to reconstruct pixels from various samples.
    pub fn add_sample(
        &mut self,
        sample_film_point: &Point2<f32>,
        radiance: &RgbaSpectrum,
        sample_weight: f32,
        filter: &dyn Filter,
    ) {
        if let Some(bounds) = self.pixel_bounds_for_sample_point(
            sample_film_point,
            filter.half_width(),
            filter.half_height(),
        ) {
            // Loop through each pixel that the sample might contribute to.
            for pixel_min_corner in bounds.range() {
                // Find the position of the sample relative to the min corner of
                // the pixel.
                let sample_offset = sample_film_point
                    - Point2::new(pixel_min_corner.x as f32, pixel_min_corner.y as f32);
                let filter_weight = filter.eval_at(Point2::new(0.0, 0.0) + sample_offset);
                let index = self.pixel_index(&pixel_min_corner);
                self.pixels[index].filter_weight_sum += filter_weight;
                self.pixels[index].weighted_spectrum_sum +=
                    filter_weight * sample_weight * radiance;
            }
        }
    }

    /// Get the pixel whose top-left corner is at the given point.
    pub fn pixel_at(&self, pixel_min_corner: Point2<i32>) -> Option<&FilmTilePixel> {
        let index = self.pixel_index(&pixel_min_corner);
        self.pixels.get(index)
    }

    /// Return a bounding box around the pixels (in raster space) that a sample
    /// at `sample_point` could contribute to.
    ///
    /// * sample_point - The point in raster space at which a sample is taken.
    /// * filter_half_width
    /// * filter_half_height
    fn pixel_bounds_for_sample_point(
        &self,
        sample_point: &Point2<f32>,
        filter_half_width: f32,
        filter_half_height: f32,
    ) -> Option<Bounds2<i32>> {
        let shifted = sample_point - Vector2::new(0.5, 0.5);
        let min = Point2::new(
            (shifted.x - filter_half_width).ceil() as i32,
            (shifted.y - filter_half_height).ceil() as i32,
        );
        let max = Point2::new(
            (shifted.x + filter_half_width).floor() as i32 + 1,
            (shifted.y + filter_half_height).floor() as i32 + 1,
        );
        let possible_pixel_bounds = Bounds2::new(min, max);
        possible_pixel_bounds.intersect(&self.pixel_bounds)
    }

    /// Get the index into `pixels` of the pixel with the given top-left corner
    /// in raster space.
    fn pixel_index(&self, p: &Point2<i32>) -> usize {
        let relative_p = Point2::new(p.x - self.pixel_bounds.min.x, p.y - self.pixel_bounds.min.y);
        // FIXME: Be careful, especially if this is made public. Passing in an
        // out-of-bounds point could panic.
        (relative_p.y * self.pixel_bounds.diagonal().x + relative_p.x) as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FilmTilePixel {
    /// The running sum for the numerator of the pixel filtering equation (on p.
    /// 490 of PBR ed. 3). This value is the sum of the following product for
    /// each contributing sample: the sample's weight, times the filter value at
    /// the sample point, times the sample radiance.
    pub weighted_spectrum_sum: RgbaSpectrum,

    /// The running sum for the denominator of the pixel filtering equation (on
    /// p. 490 of PBR ed. 3). This value is the sum of filter values at the
    /// contributing sample points.
    pub filter_weight_sum: f32,
}

impl Default for FilmTilePixel {
    fn default() -> Self {
        Self {
            weighted_spectrum_sum: RgbaSpectrum::from_rgba(0.0, 0.0, 0.0, 0.0),
            filter_weight_sum: 0.0,
        }
    }
}

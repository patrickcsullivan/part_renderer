use cgmath::{Point2, Vector2};

use crate::{color::RgbSpectrum, filter::Filter, geometry::bounds::Bounds2};

/// Stores the pixel data for a subset of a larger `Film`. Multiple `FilmTile`s
/// can be merged together to produce a complete `Film`.
pub struct FilmTile {
    /// A bounding box around the pixels (in raster space) that the tile
    /// contains.
    pub pixel_bounds: Bounds2<i32>,

    pub pixels: Vec<FilterTilePixel>,
}

impl FilmTile {
    pub fn new(pixel_bounds: Bounds2<i32>) -> Self {
        todo!()
    }

    pub fn add_sample(
        &mut self,
        p: &Point2<f32>,
        radiance: &RgbSpectrum,
        sample_weight: f32,
        filter: Box<dyn Filter>,
    ) {
        // Determine which pixels in the tile the sample contributes to.
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
}

pub struct FilterTilePixel {
    /// The running sum for the numerator of the pixel filtering equation (on p.
    /// 490 of PBR ed. 3). This value is the sum of the following product for
    /// each contributing sample: the sample's weight, times the filter value at
    /// the sample point, times the sample radiance.
    pub weighted_spectrum_sum: RgbSpectrum,

    /// The running sum for the denominator of the pixel filtering equation (on
    /// p. 490 of PBR ed. 3). This value is the sum of filter values at the
    /// contributing sample points.
    pub filter_weight_sum: f32,
}

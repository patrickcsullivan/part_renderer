mod box_filter;
mod mitchell;
mod precomputed;

pub use {box_filter::BoxFilter, mitchell::MitchellFilter, precomputed::PrecomputedFilter};

use cgmath::Point2;

/// A rectangular filter centered at (0, 0) that, when evalued for a given
/// point, returns a weight.
pub trait Filter {
    fn eval_at(&self, p: Point2<f32>) -> f32;

    /// Return half the width of the filter.
    fn half_width(&self) -> f32;

    /// Return half the height of the filter.
    fn half_height(&self) -> f32;
}

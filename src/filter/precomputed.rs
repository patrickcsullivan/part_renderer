use super::Filter;
use cgmath::Point2;

pub struct PrecomputedFilter {
    half_width: f32,
    half_height: f32,

    /// A 16-by-16 row-major grid of pre-computed filter values for points
    /// within the source filter's `half_width`.
    values: [f32; 256],
}

impl PrecomputedFilter {
    /// Pre-compute values for various inputs to the given filter, and return a
    /// new filter that uses that those pre-computed values.
    ///
    /// * filter - A filter that is symmetric about the x and y axes. That is,
    ///   f(x, y) must be equal to f(|x|, |y|).
    pub fn new(filter: Box<dyn Filter>) -> Self {
        let mut values = [0.0; 256];

        // Divide the filter's positive quadrant into a 16-by-16 grid and
        // evaluate the filter at the center of each cell in the grid.
        for y in 0..16 {
            for x in 0..16 {
                let offset = Self::index_to_offset(x, y);
                let p = Point2::new(
                    (x as f32 + 0.5) * filter.half_width() / 16.0,
                    (y as f32 + 0.5) * filter.half_height() / 16.0,
                );
                values[offset] = filter.eval_at(p);
            }
        }

        Self {
            values,
            half_width: filter.half_width(),
            half_height: filter.half_height(),
        }
    }

    pub fn index_to_offset(x: usize, y: usize) -> usize {
        y * 16 + x
    }
}

impl Filter for PrecomputedFilter {
    fn eval_at(&self, p: Point2<f32>) -> f32 {
        todo!()
    }

    fn half_width(&self) -> f32 {
        self.half_width
    }

    fn half_height(&self) -> f32 {
        self.half_height
    }
}

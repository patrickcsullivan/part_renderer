use super::Filter;
use cgmath::Point2;

const TABLE_WIDTH: usize = 16;

pub struct PrecomputedFilter {
    half_width: f32,
    half_height: f32,

    /// A 16-by-16 row-major grid of pre-computed filter values for points
    /// within the source filter's `half_width`.
    table: [f32; 256],
}

impl PrecomputedFilter {
    /// Pre-compute values for various inputs to the given filter, and return a
    /// new filter that uses that those pre-computed values.
    ///
    /// * filter - A filter that is symmetric about the x and y axes. That is,
    ///   f(x, y) must be equal to f(|x|, |y|).
    pub fn new(filter: Box<dyn Filter>) -> Self {
        let mut table = [0.0; TABLE_WIDTH * TABLE_WIDTH];

        // Divide the filter's positive quadrant into a 16-by-16 grid and
        // evaluate the filter at the center of each cell in the grid.
        for y in 0..TABLE_WIDTH {
            for x in 0..TABLE_WIDTH {
                let index = Self::table_index(x, y);
                let p = Point2::new(
                    (x as f32 + 0.5) * filter.half_width() / TABLE_WIDTH as f32,
                    (y as f32 + 0.5) * filter.half_height() / TABLE_WIDTH as f32,
                );
                table[index] = filter.eval_at(p);
            }
        }

        Self {
            table,
            half_width: filter.half_width(),
            half_height: filter.half_height(),
        }
    }

    fn table_index(x: usize, y: usize) -> usize {
        y * TABLE_WIDTH + x
    }
}

impl Filter for PrecomputedFilter {
    fn eval_at(&self, p: Point2<f32>) -> f32 {
        let x = p.x.abs();
        let y = p.y.abs();

        if x > self.half_width || y > self.half_height {
            return 0.0;
        }

        let cell_x = (((x / self.half_width) * TABLE_WIDTH as f32) as usize).min(TABLE_WIDTH);
        let cell_y = (((y / self.half_height) * TABLE_WIDTH as f32) as usize).min(TABLE_WIDTH);
        self.table[Self::table_index(cell_x, cell_y)]
    }

    fn half_width(&self) -> f32 {
        self.half_width
    }

    fn half_height(&self) -> f32 {
        self.half_height
    }
}

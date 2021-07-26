use cgmath::{point2, Point2};

/// A data structure that maintains the internal state for a "pixel sampler", a
/// type of sampler that generates all sample vectors for a single pixel at once,
/// rather than generating sample vectors as they are requested.
pub struct PixelSamplerState {
    samples_per_pixel: usize,

    /// A table containing pre-computed dimensions for all sample vectors for
    /// the current pixel.
    ///
    /// Data is stored in "dimension-major" order (as in "row-major" order).
    /// That is, the outer vector contains one nested vector for each
    /// potentially requested dimension. Each nested vector contains one element
    /// for each sample in the pixel. So `precomputed[i][j]` refers to the `i`th
    /// 1D request for the pixel's `j`th sample.
    precomputed_1d: Vec<Vec<f32>>,

    /// A table containing pre-computed dimension pairs for all sample vectors
    /// for the current pixel.
    ///
    /// The table has a size of `max_dimension_requests`-by-`samples_per_pixel`.
    /// Data is stored in "dimension-major" order (as in "row-major" order).
    /// That is, the outer vector contains one nested vector for each
    /// potentially requested dimension pair. Each nested vector contains one
    /// element for each sample in the pixel. So `precomputed[i][j]` refers to
    /// the `i`th 2D request for the pixel's `j`th sample.
    precomputed_2d: Vec<Vec<Point2<f32>>>,

    current_sample_index: usize,
    current_1d_index: usize,
    current_2d_index: usize,
}

impl PixelSamplerState {
    pub fn new(samples_per_pixel: usize, max_dimension_requests: usize) -> Self {
        let precomputed_1d = vec![vec![0.0; samples_per_pixel]; max_dimension_requests];
        let precomputed_2d =
            vec![vec![point2(0.0, 0.0); samples_per_pixel]; max_dimension_requests];
        Self {
            samples_per_pixel,
            precomputed_1d,
            precomputed_2d,
            current_sample_index: 0,
            current_1d_index: 0,
            current_2d_index: 0,
        }
    }

    pub fn start_pixel(
        &mut self,
        precomputed_1d: Vec<Vec<f32>>,
        precomputed_2d: Vec<Vec<Point2<f32>>>,
    ) {
        self.current_sample_index = 0;
        self.current_1d_index = 0;
        self.current_2d_index = 0;
        self.precomputed_1d = precomputed_1d;
        self.precomputed_2d = precomputed_2d;
    }

    pub fn get_1d(&mut self) -> f32 {
        let val = self
            .precomputed_1d
            .get(self.current_1d_index)
            .and_then(|vals_for_dim| vals_for_dim.get(self.current_sample_index))
            .copied()
            .unwrap_or(0.5); // TODO: Use a random number instead of 0.5.
        self.current_1d_index += 1;
        val
    }

    pub fn get_2d(&mut self) -> Point2<f32> {
        let val = self
            .precomputed_2d
            .get(self.current_2d_index)
            .and_then(|vals_for_dim| vals_for_dim.get(self.current_sample_index))
            .copied()
            .unwrap_or_else(|| point2(0.5, 0.5)); // TODO: Use a random number instead of 0.5.
        self.current_1d_index += 1;
        val
    }

    pub fn start_next_sample(&mut self) -> bool {
        self.current_sample_index += 1;
        self.current_1d_index = 0;
        self.current_2d_index = 0;
        self.current_sample_index < self.samples_per_pixel
    }
}

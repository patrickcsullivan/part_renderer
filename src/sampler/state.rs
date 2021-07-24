use cgmath::Point2;

/// Provides state management and book-keeping that is useful in the
/// implementation of samplers.
pub struct InnerSamplerState {
    samples_per_pixel: usize,
    current_pixel: Point2<i32>,
    current_pixel_sample_index: usize,

    /// Pre-computed 1D values for each sample in the pixel and for each 1D
    /// dimension request.
    ///
    /// The outer vector contains an element for each vector request. Each of
    /// these elements corresponds to a distinct dimension.
    ///
    /// The next vector contains an element for each sample in a pixel.
    ///
    /// The inner vector contains the requested number of 1D values for each
    /// sample.
    vecs_1d: Vec<Vec<Vec<f32>>>,

    /// Pre-computed 2D values for each sample in the pixel and for each 2D
    /// dimension request.
    ///
    /// The outer vector contains an element for each vector request. Each of
    /// these elements corresponds to a distinct pair of adjacent dimensions.
    ///
    /// The next vector contains an element for each sample in a pixel.
    ///
    /// The inner vector contains the requested number of 2D values for each
    /// sample.
    vecs_2d: Vec<Vec<Vec<Point2<f32>>>>,

    /// The current index into `vecs_1d`. This is equivalent to the number of
    /// requests to `get_1d_vec` for the current sample.
    vecs_1d_current_index: usize,

    /// The current index into `vecs_2d`. This is equivalent to the number of
    /// requests to `get_2d_vec` for the current sample.
    vecs_2d_current_index: usize,
}

impl InnerSamplerState {
    pub fn new(samples_per_pixel: usize) -> Self {
        Self {
            samples_per_pixel,
            current_pixel: Point2::new(0, 0),
            current_pixel_sample_index: 0,

            vecs_1d: vec![],
            vecs_2d: vec![],
            vecs_1d_current_index: 0,
            vecs_2d_current_index: 0,
        }
    }

    pub fn samples_per_pixel(&self) -> usize {
        self.samples_per_pixel
    }

    pub fn current_pixel(&self) -> Point2<i32> {
        self.current_pixel
    }

    pub fn current_pixel_sample_index(&self) -> usize {
        self.current_pixel_sample_index
    }

    pub fn start_pixel(&mut self, pixel: Point2<i32>) {
        self.current_pixel = pixel;
        self.current_pixel_sample_index = 0;
        self.reset_current_sample_vecs_indices();
    }

    pub fn prepare_1d_array(&mut self, count: usize) {
        // Initialize a `count`-length vec for each sample in the pixel.
        self.vecs_1d
            .push(vec![vec![0.0; count]; self.samples_per_pixel]);
    }

    pub fn prepare_2d_array(&mut self, count: usize) {
        // Initialize a `count`-length vec for each sample in the pixel.
        self.vecs_2d.push(vec![
            vec![Point2::new(0.0, 0.0); count];
            self.samples_per_pixel
        ]);
    }

    pub fn get_1d_vec(&mut self) -> Option<&Vec<f32>> {
        self.vecs_1d
            .get(self.vecs_1d_current_index)
            .and_then(|vec_per_sample| vec_per_sample.get(self.current_pixel_sample_index))
    }

    pub fn get_2d_vec(&mut self) -> Option<&Vec<Point2<f32>>> {
        self.vecs_2d
            .get(self.vecs_2d_current_index)
            .and_then(|vec_per_sample| vec_per_sample.get(self.current_pixel_sample_index))
    }

    pub fn start_next_sample(&mut self) -> bool {
        self.reset_current_sample_vecs_indices();
        self.current_pixel_sample_index += 1;
        self.current_pixel_sample_index < self.samples_per_pixel
    }

    pub fn start_nth_sample(&mut self, sample_index: usize) -> bool {
        self.reset_current_sample_vecs_indices();
        self.current_pixel_sample_index = sample_index;
        self.current_pixel_sample_index < self.samples_per_pixel
    }

    fn reset_current_sample_vecs_indices(&mut self) {
        self.vecs_1d_current_index = 0;
        self.vecs_2d_current_index = 0;
    }
}

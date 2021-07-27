use super::{pixel::PixelSamplerState, Sampler, MAX_SAMPLE};
use cgmath::{point2, Point2};
use rand::{prelude::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct StratifiedSampler {
    x_strata_count: usize,
    y_strata_count: usize,
    max_dimension_requests: usize,
    pixel_sampler_state: PixelSamplerState,
    rng: ChaCha8Rng,
    jitter: bool,
}

impl Sampler for StratifiedSampler {
    fn clone_with_seed(&self, seed: u64) -> Self {
        let samples_per_pixel = self.x_strata_count * self.y_strata_count;
        Self {
            x_strata_count: self.x_strata_count,
            y_strata_count: self.y_strata_count,
            max_dimension_requests: self.max_dimension_requests,
            pixel_sampler_state: PixelSamplerState::new(
                samples_per_pixel,
                self.max_dimension_requests,
            ),
            rng: ChaCha8Rng::seed_from_u64(seed),
            jitter: self.jitter,
        }
    }

    fn samples_per_pixel(&self) -> usize {
        self.x_strata_count * self.y_strata_count
    }

    fn start_pixel(&mut self, _pixel: Point2<i32>) {
        let mut precomputed_1d: Vec<Vec<f32>> = (0..self.max_dimension_requests)
            .map(|_| {
                self.stratified_samples_1d(self.x_strata_count * self.y_strata_count, self.jitter)
            })
            .collect();
        let mut precomputed_2d: Vec<Vec<Point2<f32>>> = (0..self.max_dimension_requests)
            .map(|_| {
                self.stratified_samples_2d(self.x_strata_count, self.y_strata_count, self.jitter)
            })
            .collect();

        // Shuffle the samples in each dimension to eliminate undesirable
        // correlations between sample values in the same sample vector. (For
        // example, if we don't do this then two 2D samples in the same sample
        // vector will be selected from the exact same strata.)
        for dim in precomputed_1d.iter_mut() {
            dim.shuffle(&mut self.rng);
        }
        for dim in precomputed_2d.iter_mut() {
            dim.shuffle(&mut self.rng);
        }

        self.pixel_sampler_state
            .start_pixel(precomputed_1d, precomputed_2d);
    }

    fn get_1d(&mut self) -> f32 {
        self.pixel_sampler_state.get_1d()
    }

    fn get_2d(&mut self) -> Point2<f32> {
        self.pixel_sampler_state.get_2d()
    }

    fn start_next_sample(&mut self) -> bool {
        self.pixel_sampler_state.start_next_sample()
    }
}

impl StratifiedSampler {
    /// Create a new stratified sampler.
    ///
    /// When two dimensions of a pixel are sampled together (using a call to
    /// `get_2d`), the sample space in the pixel is divided into
    /// `x_strata_count` for the first dimension and `y_strata_count` for the
    /// second dimension. This results in `x_strata_count` times
    /// `y_strata_count` strata and 2D samples for each pixel.
    ///
    /// When a single dimension of a pixel is sampled separately (using a call
    /// to `get_1d`), the sample space in the pixel is divided into
    /// `x_strata_count` times `y_strata_count` strata for that single
    /// dimension. This results in `x_strata_count` times `y_strata_count` 1D
    /// samples for each pixel.
    ///
    /// We choose to divide a 1D sample space into `x_strata_count` times
    /// `y_strata_count` strata, rather than just `x_strata_count` strata, so
    /// that we're always generating the same number of samples per pixel,
    /// regardless of whether we're sample one dimension or two dimensions at a
    /// time.
    pub fn new(
        x_strata_count: usize,
        y_strata_count: usize,
        max_dimension_requests: usize,
        seed: u64,
        jitter: bool,
    ) -> Self {
        let samples_per_pixel = x_strata_count * y_strata_count;
        Self {
            x_strata_count,
            y_strata_count,
            max_dimension_requests,
            pixel_sampler_state: PixelSamplerState::new(samples_per_pixel, max_dimension_requests),
            rng: ChaCha8Rng::seed_from_u64(seed),
            jitter,
        }
    }

    /// Generate a 1D sample for each strata of the dimension being sampled.
    ///
    /// * strata_count - The number of strata that divide dimension being
    ///   sampled.
    /// * jitter - Set to `true` to randomly place each sample in its strata.
    ///   Set to `false` to place each sample in the mid-point of its strata.
    ///   This should be set to `true` to generate high-quality samples. Setting
    ///   to `false` can be useful for debugging.
    fn stratified_samples_1d(&mut self, strata_count: usize, jitter: bool) -> Vec<f32> {
        let inv_strata_count = 1.0 / strata_count as f32;
        (0..strata_count)
            .map(|strata_index| {
                let delta = if jitter { self.rng.gen() } else { 0.5 };
                // The random number could be 1.0, so we need to clamp to
                // `MAX_SAMPLE`.
                ((strata_index as f32 + delta) * inv_strata_count).min(MAX_SAMPLE)
            })
            .collect()
    }

    /// Generate a 2D sample for each strata of the two dimensions being
    /// sampled.
    ///
    /// The sample space between (0, 0) inclusive and (1, 1) exclusive, will be
    /// divided into a total of `x_strata_count` time `y_strata_count` 2D
    /// strata. This method will return a 2D sample from each of those strata.
    ///
    /// * x_strata_count - The number of strata that divide the first dimension
    ///   being sampled.
    /// * y_strata_count - The number of strata that divide the second dimension
    ///   being sampled.
    /// * jitter - Set to `true` to randomly place each sample in its strata.
    ///   Set to `false` to place each sample in the mid-point of its strata.
    ///   This should be set to `true` to generate high-quality samples. Setting
    ///   to `false` can be useful for debugging.
    fn stratified_samples_2d(
        &mut self,
        x_strata_count: usize,
        y_strata_count: usize,
        jitter: bool,
    ) -> Vec<Point2<f32>> {
        let inv_x_strata_count = 1.0 / x_strata_count as f32;
        let inv_y_strata_count = 1.0 / y_strata_count as f32;
        let xs = 0..x_strata_count;
        let ys = 0..y_strata_count;
        ys.flat_map(|y| xs.clone().map(move |x| (x, y)))
            .map(|(x_strata_index, y_strata_index)| {
                let x_delta = if jitter { self.rng.gen() } else { 0.5 };
                let y_delta = if jitter { self.rng.gen() } else { 0.5 };
                point2(
                    ((x_strata_index as f32 + x_delta) * inv_x_strata_count).min(MAX_SAMPLE),
                    ((y_strata_index as f32 + y_delta) * inv_y_strata_count).min(MAX_SAMPLE),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod stratified_sampler_tests {
    use cgmath::{point2, Point2};
    use super::super::Sampler;
    use super::StratifiedSampler;

    #[test]
    fn generates_stratified_samples() {
        let mut sampler = StratifiedSampler::new(2, 3, 5, 0, true);

        sampler.start_pixel(point2(3, 4)); // The specific pixel doesn't matter.

        // First sample vector.
        let s0_xy = sampler.get_2d();
        let s0_t = sampler.get_1d();
        let s0_uv = sampler.get_2d();

        // Second sample vector.
        assert!(sampler.start_next_sample());
        let s1_xy = sampler.get_2d();
        let s1_t = sampler.get_1d();
        let s1_uv = sampler.get_2d();

        // Third sample vector.
        assert!(sampler.start_next_sample());
        let s2_xy = sampler.get_2d();
        let s2_t = sampler.get_1d();
        let s2_uv = sampler.get_2d();

        // Fourth sample vector.
        assert!(sampler.start_next_sample());
        let s3_xy = sampler.get_2d();
        let s3_t = sampler.get_1d();
        let s3_uv = sampler.get_2d();

        // Fifth sample vector.
        assert!(sampler.start_next_sample());
        let s4_xy = sampler.get_2d();
        let s4_t = sampler.get_1d();
        let s4_uv = sampler.get_2d();

        // Sixth sample vector.
        assert!(sampler.start_next_sample());
        let s5_xy = sampler.get_2d();
        let s5_t = sampler.get_1d();
        let s5_uv = sampler.get_2d();

        assert!(!sampler.start_next_sample());

        // Collect all the xy samples, t samples, and uv samples from the
        // different sample vectors.
        let xy_samples = vec![s0_xy, s1_xy, s2_xy, s3_xy, s4_xy, s5_xy];
        let t_samples = vec![s0_t, s1_t, s2_t, s3_t, s4_t, s5_t];
        let uv_samples = vec![s0_uv, s1_uv, s2_uv, s3_uv, s4_uv, s5_uv];

        // The six 1D strata divide 1D sample space into six strata.
        let strata0 = (0.0, 1.0 / 6.0);
        let strata1 = (1.0 / 6.0, 2.0 / 6.0);
        let strata2 = (2.0 / 6.0, 3.0 / 6.0);
        let strata3 = (3.0 / 6.0, 4.0 / 6.0);
        let strata4 = (4.0 / 6.0, 5.0 / 6.0);
        let strata5 = (5.0 / 6.0, 1.0);
        let strata_1d = vec![strata0, strata1, strata2, strata3, strata4, strata5];

        // The six 2D strata divide 2D sample space into two strata in the first
        // dimension and three strata in the second dimension for a total of six
        // strata.
        let strata00 = (point2(0.0, 0.0), point2(1.0 / 2.0, 1.0 / 3.0));
        let strata10 = (point2(1.0 / 2.0, 0.0), point2(1.0, 1.0 / 3.0));
        let strata01 = (point2(0.0, 1.0 / 3.0), point2(1.0 / 2.0, 2.0 / 3.0));
        let strata11 = (point2(1.0 / 2.0, 1.0 / 3.0), point2(1.0, 2.0 / 3.0));
        let strata02 = (point2(0.0, 2.0 / 3.0), point2(1.0 / 2.0, 1.0));
        let strata12 = (point2(1.0 / 2.0, 2.0 / 3.0), point2(1.0, 1.0));
        let strata_2d = vec![strata00, strata10, strata01, strata11, strata02, strata12];

        // Check that samples across each sampled dimension are stratified. We
        // only check that each strata has AT LEAST one sample, rather than
        // EXACTLY one, since a sample could be on the border between two
        // strata, and strata bounds are inclusive.

        for strata in strata_2d.iter() {
            let sample_count = xy_samples
                .iter()
                .filter(|sample| in_strata_2d(*sample, strata))
                .count();
            assert!(
                sample_count >= 1, 
                "Expected the 2D strata spanning ({}, {}) to ({}, {}) to contain 1 or 2 samples, but it contained {}.", 
                strata.0.x, 
                strata.0.y, 
                strata.1.x, 
                strata.1.y, 
                sample_count 
            );
        }

        for strata in strata_1d.iter() {
            let sample_count = t_samples
                .iter()
                .filter(|sample| in_strata_1d(*sample, strata))
                .count();
            assert!(
                sample_count >= 1, 
                "Expected the 2D strata spanning {} to {} contain 1 or 2 samples, but it contained {}.", 
                strata.0, 
                strata.1,
                sample_count 
            );
        }

        for strata in strata_2d.iter() {
            let sample_count = uv_samples
                .iter()
                .filter(|sample| in_strata_2d(*sample, strata))
                .count();
            assert!(
                sample_count >= 1, 
                "Expected the 2D strata spanning ({}, {}) to ({}, {}) to contain 1 or 2 samples, but it contained {}.", 
                strata.0.x, 
                strata.0.y, 
                strata.1.x, 
                strata.1.y, 
                sample_count 
            );
        }
    }

    /// Check if the sample is in the strata defined by the given min and max
    /// bounds. Bounds are inclusive.
    fn in_strata_1d(sample: &f32, min_max: &(f32, f32)) -> bool {
        *sample >= min_max.0 && *sample <= min_max.1
    }

    /// Check if the sample is in the strata defined by the given min and max
    /// bounds. Bounds are inclusive.
    fn in_strata_2d(sample: &Point2<f32>, min_max: &(Point2<f32>, Point2<f32>)) -> bool {
        sample.x >= min_max.0.x
            && sample.x <= min_max.1.x
            && sample.y >= min_max.0.y
            && sample.y <= min_max.1.y
    }
}

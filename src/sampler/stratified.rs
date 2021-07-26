use super::{pixel::PixelSamplerState, Sampler, MAX_SAMPLE};
use cgmath::{point2, Point2};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct StratifiedSampler {
    samples_per_pixel: usize,
    max_dimension_requests: usize,
    pixel_sampler_state: PixelSamplerState,
    rng: ChaCha8Rng,
}

impl Sampler for StratifiedSampler {
    fn clone_with_seed(&self, seed: u64) -> Self {
        Self {
            samples_per_pixel: self.samples_per_pixel,
            max_dimension_requests: self.max_dimension_requests,
            pixel_sampler_state: PixelSamplerState::new(
                self.samples_per_pixel,
                self.max_dimension_requests,
            ),
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    fn samples_per_pixel(&self) -> usize {
        todo!()
    }

    fn start_pixel(&mut self, pixel: Point2<i32>) {
        todo!()
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
    pub fn new(samples_per_pixel: usize, max_dimension_requests: usize, seed: u64) -> Self {
        Self {
            samples_per_pixel,
            max_dimension_requests,
            pixel_sampler_state: PixelSamplerState::new(samples_per_pixel, max_dimension_requests),
            rng: ChaCha8Rng::seed_from_u64(seed),
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

use cgmath::Point2;

use super::Sampler;

pub struct ConstantSampler {}

impl Sampler for ConstantSampler {
    fn clone_with_seed(&self, _seed: usize) -> Self {
        Self {}
    }

    fn samples_per_pixel(&self) -> usize {
        1
    }

    fn start_pixel(&mut self, _pixel: Point2<i32>) {}

    fn get_1d(&mut self) -> f32 {
        0.5
    }

    fn get_2d(&mut self) -> Point2<f32> {
        Point2::new(0.5, 0.5)
    }

    fn start_next_sample(&mut self) -> bool {
        // Only one sample per pixel.
        false
    }

    // fn prepare_1d_array(&mut self, count: usize) {}

    // fn prepare_2d_array(&mut self, count: usize) {}

    // fn get_1d_vec(&mut self) -> Option<Vec<f32>> {
    //     None
    // }

    // fn get_2d_vec(&mut self) -> Option<Vec<Point2<f32>>> {
    //     None
    // }

    // fn start_nth_sample(&mut self, sample_index: usize) -> bool {
    //     false
    // }
}

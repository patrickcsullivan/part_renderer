use crate::{camera::Camera, sampler::Sampler, scene::Scene};

pub trait Integrator {
    fn render(&self, scene: &Scene);
}

/// An integrator that renders an image of a scene from a stream of samples.
/// Each sample identifies a point on the image at which the integrator should
/// compute arriving light.
pub struct SampleIntegrator<S: Sampler> {
    /// A sampler that is responsible for (1) choosing points on the image from
    /// which rays are traced and (2) supplying sample positions used by the
    /// integrator to estimate the value of the light transport integral.
    sampler: S,

    /// Controls how the scene is viewed and contains the `Film` onto which the
    /// scene is rendered.
    camera: Camera,
}

impl<S: Sampler> SampleIntegrator<S> {
    pub fn new(sampler: S, camera: Camera) -> Self {
        Self { sampler, camera }
    }
}

impl<S: Sampler> Integrator for SampleIntegrator<S> {
    fn render(&self, scene: &Scene) {
        todo!()
    }
}

pub fn main() {
    let scene: Scene = todo!();
    let sampler = MockSampler;
    let camera = todo!();
    let integrator = SampleIntegrator::new(sampler, camera);
    ()
}

struct MockSampler;

impl Sampler for MockSampler {
    fn samples_per_pixel(&self) -> usize {
        todo!()
    }
    fn start_pixel(&mut self, pixel: cgmath::Point2<usize>) {
        todo!()
    }
    fn get_1d(&mut self) -> f32 {
        todo!()
    }
    fn get_2d(&mut self) -> cgmath::Point2<f32> {
        todo!()
    }
    fn prepare_1d_array(&mut self, count: usize) {
        todo!()
    }
    fn prepare_2d_array(&mut self, count: usize) {
        todo!()
    }
    fn get_1d_vec(&mut self) -> Option<&Vec<f32>> {
        todo!()
    }
    fn get_2d_vec(&mut self) -> Option<&Vec<cgmath::Point2<f32>>> {
        todo!()
    }
    fn start_next_sample(&mut self) -> bool {
        todo!()
    }
    fn start_nth_sample(&mut self, sample_index: usize) -> bool {
        todo!()
    }
}

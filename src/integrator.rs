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

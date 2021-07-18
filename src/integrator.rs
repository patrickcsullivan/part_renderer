use cgmath::Point2;
use typed_arena::Arena;

use crate::{camera::Camera, color::RgbSpectrum, ray::Ray, sampler::Sampler, scene::Scene};

pub trait Integrator {
    fn render(&mut self, scene: &Scene);
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

    fn preprocess(&mut self) {
        todo!();
    }

    /// Determine the incident radiance that arrives at the ray origin along the
    /// ray.
    ///
    /// * `ray` - The ray along which incident radience is caluclated.
    /// * `scene` - The scene being rendered.
    /// * `sampler` - The sampler that is used to solve the light transport equation
    ///   using Monte Carlo integration.
    /// * `spectrum_arena` - An arena that will be used for efficient memory
    ///   allocation of temporary spectrums that are used in the incident radience
    ///   calculation.
    /// * `depth` - The number of ray bounces from the camera that have occured up until the current call to this method.
    fn incident_radiance(
        ray: &Ray,
        scene: &Scene,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
    ) -> RgbSpectrum {
        todo!();
    }
}

impl<S: Sampler> Integrator for SampleIntegrator<S> {
    fn render(&mut self, scene: &Scene) {
        self.preprocess();

        // Render tiles in paralle.
        // >>> For each tile...
        // >>>>>> For each series of positions on the image plane.
        let raster_point: Point2<usize> = todo!();
        let sample = self.sampler.get_camera_sample(raster_point);
        let (ray, contrib) = self.camera.generate_ray(&sample);

        let sampler_clone: S = todo!();
        let spectrum_arena = Arena::new();
        let light =
            Self::incident_radiance(&ray, &scene, &mut sampler_clone, &mut spectrum_arena, 0);
        // TODO: Check for spectrums containing non-numbers.
        // Add light sample to tile.
        // >>>>>>
        // Merge tile.
        // >>>
        // Save final image.
    }
}

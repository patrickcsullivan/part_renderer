use crate::{
    camera::Camera, color::RgbSpectrum, interaction::SurfaceInteraction, ray::Ray,
    sampler::Sampler, scene::Scene,
};
use cgmath::{InnerSpace, Point2};
use typed_arena::Arena;

/// An ray tracer based on Whitted's ray tracing algorithm. This can accurately
/// compute reflected and transmitted light from specular surfaces like glass,
/// mirrors, and water. It does not account for indirect lighting effects.
pub struct WhittedIntegrator<S: Sampler> {
    max_depth: usize,

    /// A sampler that is responsible for (1) choosing points on the image from
    /// which rays are traced and (2) supplying sample positions used by the ray
    /// tracer to estimate the value of the light transport integral.
    sampler: S,

    /// Controls how the scene is viewed and contains the `Film` onto which the
    /// scene is rendered.
    camera: Box<dyn Camera>,
}

impl<S: Sampler> WhittedIntegrator<S> {
    pub fn new(camera: Box<dyn Camera>, sampler: S) -> Self {
        Self {
            max_depth: 5,
            camera,
            sampler,
        }
    }

    pub fn render(&mut self, scene: &Scene) {
        // Render tiles in paralle.
        // >>> For each tile...
        // >>>>>> For each series of positions on the image plane.
        let raster_point: Point2<usize> = todo!();
        let sample = self.sampler.get_camera_sample(raster_point);
        let (ray, contrib) = self.camera.generate_ray(&sample);

        let sampler_clone: S = todo!();
        let spectrum_arena = Arena::new();
        let light =
            self.incoming_radiance(&ray, &scene, &mut sampler_clone, &mut spectrum_arena, 0);
        // TODO: Check for spectrums containing non-numbers.
        // Add light sample to tile.
        // >>>>>>
        // Merge tile.
        // >>>
        // Save final image.
    }

    /// Determine the incoming radiance that arrives along the ray at the ray
    /// origin.
    ///
    /// * `ray` - The ray along which incoming radiance is caluclated.
    /// * `scene` - The scene being rendered.
    /// * `sampler` - The sampler that is used to solve the light transport
    ///   equation using Monte Carlo integration.
    /// * `spectrum_arena` - An arena that will be used for efficient memory
    ///   allocation of temporary spectrums used in the incoming radiance
    ///   calculation.
    /// * `depth` - The number of ray bounces from the camera that have occured
    ///   up until the current call to this method.
    fn incoming_radiance(
        &self,
        // TODO: Change to ray differential.
        ray: &Ray,
        scene: &Scene,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
    ) -> RgbSpectrum {
        if let Some((_t, _prim, interaction)) = scene.ray_intersection(ray) {
            // We will calculate the outgoing radiance along the ray at the
            // surface. Since we ignore all particpating media (like smoke or
            // fog), the outgoing radiance at the intersected surface will equal
            // the incoming radiance at the ray origin.
            let mut outgoing_radiance = RgbSpectrum::constant(0.0);

            // Initialize the normal and outgoing direction of light at the
            // surface.
            let normal = interaction.shading_geometry.normal;
            let point_to_ray_origin_direction = interaction.neg_ray_direction;

            // Compute scattering functions for surface interaction.
            interaction.compute_scattering_functions(ray, spectrum_arena);

            // Compute emitted light if ray hit an area light source.
            outgoing_radiance += interaction.emitted_radiance(&point_to_ray_origin_direction);

            // Add the contribution of each light source.
            for light in &scene.lights {
                let sample = sampler.get_2d();
                let (radiance_from_light, point_to_light_direction, pdf, visibility) =
                    light.sample_incoming_radiance_at_surface(&interaction, sample);
                if radiance_from_light.is_black() || pdf == 0.0 {
                    continue;
                }
                let f = interaction.bsdf(&point_to_light_direction, &point_to_ray_origin_direction);
                if !f.is_black() && visibility.unocculuded(scene) {
                    outgoing_radiance += f
                        * radiance_from_light
                        * (point_to_light_direction.dot(normal).abs() / pdf);
                }
            }

            if depth + 1 < self.max_depth {
                // Trace rays for specular reflection and refraction.
            }

            outgoing_radiance
        } else {
            let mut ray_origin_incoming_radiance = RgbSpectrum::constant(0.0);
            for light in &scene.lights {
                ray_origin_incoming_radiance += light.outgoing_radiance_onto_ray(ray);
            }
            ray_origin_incoming_radiance
        }
    }

    fn specular_reflect(
        &self,
        ray: &Ray,
        interaction: &SurfaceInteraction,
        scene: &Scene,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
    ) -> RgbSpectrum {
        todo!()
    }

    fn specular_transmit(
        &self,
        ray: &Ray,
        interaction: &SurfaceInteraction,
        scene: &Scene,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
    ) -> RgbSpectrum {
        todo!()
    }
}

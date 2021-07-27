use crate::{
    camera::Camera, color::RgbSpectrum, filter::Filter, geometry::bounds::Bounds2,
    interaction::SurfaceInteraction, ray::Ray, sampler::IncrementalSampler, scene::Scene,
};
use cgmath::InnerSpace;
use typed_arena::Arena;

use super::RayTracer;

/// An ray tracer based on Whitted's ray tracing algorithm. This can accurately
/// compute reflected and transmitted light from specular surfaces like glass,
/// mirrors, and water. It does not account for indirect lighting effects.
pub struct WhittedRayTracer {}

impl WhittedRayTracer {
    fn specular_reflect<'msh, 'mtrx, 'mtrl, S: IncrementalSampler>(
        &self,
        ray: &Ray,
        interaction: &SurfaceInteraction,
        scene: &Scene<'msh, 'mtrx, 'mtrl>,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
    ) -> RgbSpectrum {
        todo!()
    }

    fn specular_transmit<'msh, 'mtrx, 'mtrl, S: IncrementalSampler>(
        &self,
        ray: &Ray,
        interaction: &SurfaceInteraction,
        scene: &Scene<'msh, 'mtrx, 'mtrl>,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
    ) -> RgbSpectrum {
        todo!()
    }
}

impl<'msh, 'mtrx, 'mtrl, S: IncrementalSampler> RayTracer<'msh, 'mtrx, 'mtrl, S>
    for WhittedRayTracer
{
    fn incoming_radiance(
        &self,
        // TODO: Change to ray differential.
        ray: &Ray,
        scene: &Scene,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
        max_depth: usize,
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

            // // Compute scattering functions for surface interaction.
            // interaction.compute_scattering_functions(ray, spectrum_arena);

            // // Compute emitted light if ray hit an area light source.
            // outgoing_radiance += interaction.emitted_radiance(&point_to_ray_origin_direction);

            // Add the contribution of each light source.
            for light in &scene.lights {
                // CONTINUE HERE. <<<------
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

            if depth + 1 < max_depth {
                // Trace rays for specular reflection and refraction.
            }

            outgoing_radiance
        } else {
            // TODO: Add back in after rest of Whitted is working.
            // let mut ray_origin_incoming_radiance = RgbSpectrum::constant(0.0);
            // for light in &scene.lights {
            //     ray_origin_incoming_radiance += light.outgoing_radiance_onto_ray(ray);
            // }
            // ray_origin_incoming_radiance
            RgbSpectrum::black()
        }
    }
}

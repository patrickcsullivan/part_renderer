use crate::{
    bsdf::BxdfType, camera::Camera, color::RgbaSpectrum, filter::Filter, geometry::bounds::Bounds2,
    interaction::SurfaceInteraction, ray::Ray, sampler::IncrementalSampler, scene::Scene,
};
use cgmath::InnerSpace;
use typed_arena::Arena;

use super::RayTracer;

/// An ray tracer based on Whitted's ray tracing algorithm. This can accurately
/// compute reflected and transmitted light from specular surfaces like glass,
/// mirrors, and water. It does not account for indirect lighting effects.
pub struct WhittedRayTracer {}

impl<'msh, 'mtrl, S: IncrementalSampler> RayTracer<'msh, 'mtrl, S> for WhittedRayTracer {
    fn incoming_radiance(
        &self,
        // TODO: Change to ray differential.
        ray: &Ray,
        scene: &Scene,
        sampler: &mut S,
        spectrum_arena: &mut Arena<RgbaSpectrum>,
        depth: usize,
        max_depth: usize,
    ) -> RgbaSpectrum {
        if let Some((_t, prim, interaction)) = scene.ray_intersection(ray) {
            // We will calculate the outgoing radiance along the ray at the
            // surface. Since we ignore all particpating media (like smoke or
            // fog), the outgoing radiance at the intersected surface will equal
            // the incoming radiance at the ray origin.
            let mut outgoing_radiance = RgbaSpectrum::constant(0.0);

            // Initialize the normal and outgoing direction of light at the
            // surface.
            let normal = interaction.shading_geometry.normal;
            let wo = interaction.neg_ray_direction;

            // Compute scattering functions for surface interaction.
            let bsdf = prim.material.scattering_functions(&interaction);

            // // Compute emitted light if ray hit an area light source.
            // outgoing_radiance += interaction.emitted_radiance(&point_to_ray_origin_direction);

            // Add the contribution of each light source.
            for light in &scene.lights {
                let sample = sampler.get_2d();
                let (incident_light, wi, vis, pdf) = light.sample_li(&interaction, &sample);
                if incident_light.is_black() || pdf == 0.0 {
                    continue;
                }

                let f = bsdf.f(&wo, &wi, BxdfType::ALL);
                outgoing_radiance += f * incident_light * (wi.dot(normal).abs() / 1.0);
                // if !f.is_black() && vis.unocculuded(scene) {
                //     outgoing_radiance += f * incident_light * (wi.dot(normal).abs() / 1.0);
                // }
            }

            if depth + 1 < max_depth {
                // Trace rays for specular reflection and refraction.
            }

            outgoing_radiance.set_a(1.0);
            outgoing_radiance
        } else {
            // TODO: Add back in after rest of Whitted is working.
            // let mut ray_origin_incoming_radiance = RgbSpectrum::constant(0.0);
            // for light in &scene.lights {
            //     ray_origin_incoming_radiance += light.outgoing_radiance_onto_ray(ray);
            // }
            // ray_origin_incoming_radiance
            if depth == 0 {
                RgbaSpectrum::from_rgba(0.0, 0.0, 0.0, 0.0)
            } else {
                RgbaSpectrum::black()
            }
        }
    }
}

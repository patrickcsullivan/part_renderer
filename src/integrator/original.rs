use crate::{
    camera::Camera, color::RgbSpectrum, filter::Filter, geometry::bounds::Bounds2,
    interaction::SurfaceInteraction, light::LightSource, material::Material, ray::Ray,
    sampler::IncrementalSampler, scene::Scene,
};
use cgmath::{InnerSpace, Point3};
use typed_arena::Arena;

use super::RayTracer;

pub struct OriginalRayTracer {}

impl<'msh, 'mtrx, 'mtrl, S: IncrementalSampler> RayTracer<'msh, 'mtrx, 'mtrl, S>
    for OriginalRayTracer
{
    fn incoming_radiance(
        &self,
        // TODO: Change to ray differential.
        ray: &Ray,
        scene: &Scene,
        _sampler: &mut S,
        _spectrum_arena: &mut Arena<RgbSpectrum>,
        depth: usize,
        max_depth: usize,
    ) -> RgbSpectrum {
        Self::color_at(scene, ray, max_depth - depth)
    }
}

impl OriginalRayTracer {
    pub fn color_at(scene: &Scene, ray: &Ray, remaining: usize) -> RgbSpectrum {
        if let Some((_t, primitive, interaction)) = scene.primitives.ray_intersection(&ray) {
            Self::shade_surface_interaction(scene, &interaction, primitive.material, remaining)
        } else {
            RgbSpectrum::constant(0.0)
        }
    }

    pub fn shade_surface_interaction(
        scene: &Scene,
        interaction: &SurfaceInteraction,
        material: &Material,
        remaining: usize,
    ) -> RgbSpectrum {
        scene
            .lights
            .iter()
            .fold(RgbSpectrum::constant(0.0), |color, light| {
                // Shift the interaction point away from the surface slightly, so that
                // the occlusion check doesn't accidentally intersect the surface.
                let in_shadow = Self::is_occluded(scene, interaction.over_point(), light);

                let surface = crate::light::phong_shading(
                    material,
                    light,
                    &interaction.point,
                    &interaction.neg_ray_direction,
                    &interaction.original_geometry.normal,
                    in_shadow,
                );

                let reflected = Self::reflected_color(scene, material, interaction, remaining);

                color + surface + reflected
            })
    }

    /// Returns true if the specified point is occluded from the light.
    pub fn is_occluded(scene: &Scene, p: Point3<f32>, light: &LightSource) -> bool {
        match light {
            LightSource::PointLight(point_light) => {
                let to_light = point_light.position - p;
                let distance = to_light.magnitude();

                let ray = Ray::new(p, to_light.normalize());
                if let Some((t, _, _)) = scene.primitives.ray_intersection(&ray) {
                    t < distance
                } else {
                    false
                }
            }
        }
    }

    fn reflected_color(
        scene: &Scene,
        material: &Material,
        interaction: &SurfaceInteraction,
        remaining: usize,
    ) -> RgbSpectrum {
        if remaining < 1 || material.reflective == 0.0 {
            RgbSpectrum::constant(0.0)
        } else {
            let reflect_ray = Ray::new(interaction.over_point(), interaction.reflect());
            let color = Self::color_at(scene, &reflect_ray, remaining - 1);
            color * material.reflective
        }
    }
}

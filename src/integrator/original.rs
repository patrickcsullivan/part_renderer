use crate::{
    color::RgbSpectrum, geometry::vector, interaction::SurfaceInteraction, light::Light,
    material_v1::MaterialV1, ray::Ray, sampler::IncrementalSampler, scene::Scene,
};
use cgmath::{InnerSpace, Point3, Vector3};
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
        material: &MaterialV1,
        remaining: usize,
    ) -> RgbSpectrum {
        scene
            .lights
            .iter()
            .fold(RgbSpectrum::constant(0.0), |color, light| {
                // // Shift the interaction point away from the surface slightly, so that
                // // the occlusion check doesn't accidentally intersect the surface.
                // let in_shadow = Self::is_occluded(scene, interaction.over_point(), *light);

                let surface = Self::shading(material, light, &interaction);

                let reflected = Self::reflected_color(scene, material, interaction, remaining);

                color + surface + reflected
            })
    }

    // /// Returns true if the specified point is occluded from the light.
    // pub fn is_occluded(scene: &Scene, p: Point3<f32>, light: &LightSource) -> bool {
    //     match light {
    //         LightSource::PointLight(point_light) => {
    //             let to_light = point_light.position - p;
    //             let distance = to_light.magnitude();

    //             let ray = Ray::new(p, to_light.normalize());
    //             if let Some((t, _, _)) = scene.primitives.ray_intersection(&ray) {
    //                 t < distance
    //             } else {
    //                 false
    //             }
    //         }
    //     }
    // }

    fn reflected_color(
        scene: &Scene,
        material: &MaterialV1,
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

    fn shading(
        material: &MaterialV1,
        light: &Light, // FIXME
        interaction: &SurfaceInteraction,
    ) -> RgbSpectrum {
        let (incident_light, to_light) = light.li(interaction);
        let effective_color = &material.color * incident_light;
        let ambient = &effective_color * material.ambient;

        // light_dot_normal is the cosine of the angle between the light and normal.
        // If it's negative then the light is on the other side of the surface.
        let light_dot_normal = to_light.dot(interaction.original_geometry.normal);

        let (diffuse, specular) = if light_dot_normal >= 0.0 {
            let diffuse = effective_color * material.diffuse * light_dot_normal;

            // reflect_dot_eye is the cosine of the angle between the reflection and
            // the camera. If it's negative then the reflection is not visible.
            let reflect = vector::reflect(-1.0 * to_light, interaction.original_geometry.normal);
            let reflect_dot_eye = reflect.dot(interaction.neg_ray_direction);
            let specular = if reflect_dot_eye <= 0.0 {
                RgbSpectrum::constant(0.0)
            } else {
                let factor = reflect_dot_eye.powf(material.shininess);
                incident_light * material.specular * factor
            };

            (diffuse, specular)
        } else {
            (RgbSpectrum::black(), RgbSpectrum::black())
        };

        ambient + diffuse + specular
    }
}

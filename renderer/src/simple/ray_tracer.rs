use crate::{
    color::RgbaSpectrum, geometry::vector, integrator::RayTracer, interaction::SurfaceInteraction,
    light::Light, ray::Ray, sampler::IncrementalSampler,
};
use cgmath::InnerSpace;

use super::{Material, Scene};

pub struct OriginalRayTracer {}

impl<'msh, Sampler: IncrementalSampler> RayTracer<Scene<'msh>, Sampler> for OriginalRayTracer {
    fn incoming_radiance(
        &self,
        ray: &Ray,
        scene: &Scene<'msh>,
        _sampler: &mut Sampler,
        depth: usize,
        max_depth: usize,
    ) -> RgbaSpectrum {
        Self::color_at(scene, ray, max_depth - depth)
    }
}

impl OriginalRayTracer {
    pub fn color_at(scene: &Scene, ray: &Ray, remaining: usize) -> RgbaSpectrum {
        if let Some((_t, primitive, interaction)) = scene.primitives.ray_intersection(&ray) {
            Self::shade_surface_interaction(scene, &interaction, &primitive.material, remaining)
        } else {
            RgbaSpectrum::transparent()
        }
    }

    pub fn shade_surface_interaction(
        scene: &Scene,
        interaction: &SurfaceInteraction,
        material: &Material,
        remaining: usize,
    ) -> RgbaSpectrum {
        scene
            .lights
            .iter()
            .fold(RgbaSpectrum::constant(0.0), |color, light| {
                // // Shift the interaction point away from the surface slightly, so that
                // // the occlusion check doesn't accidentally intersect the surface.
                // let in_shadow = Self::is_occluded(scene, interaction.over_point(), *light);
                let surface = Self::shading(material, light, &interaction);
                // let reflected = Self::reflected_color(scene, material, interaction, remaining);
                color + surface // + reflected
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

    // fn reflected_color(
    //     scene: &Scene,
    //     material: &Material,
    //     interaction: &SurfaceInteraction,
    //     remaining: usize,
    // ) -> RgbaSpectrum {
    //     if remaining < 1 || material.reflective == 0.0 {
    //         RgbSpectrum::constant(0.0)
    //     } else {
    //         let reflect_ray = Ray::new(interaction.over_point(), interaction.reflect());
    //         let color = Self::color_at(scene, &reflect_ray, remaining - 1);
    //         color * material.reflective
    //     }
    // }

    fn shading(
        material: &Material,
        light: &Light, // FIXME
        interaction: &SurfaceInteraction,
    ) -> RgbaSpectrum {
        let (incident_light, to_light, _) = light.li(interaction);
        let effective_color = material.color * incident_light;
        let ambient = effective_color * material.ambient;

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
                RgbaSpectrum::black()
            } else {
                let factor = reflect_dot_eye.powf(material.shininess);
                incident_light * material.specular * factor
            };

            (diffuse, specular)
        } else {
            (RgbaSpectrum::black(), RgbaSpectrum::black())
        };

        ambient + diffuse + specular
    }
}

mod point_light;

pub use point_light::PointLightSource;

use crate::{
    color::RgbSpectrum, geometry::vector, interaction::SurfaceInteraction, material_v1::MaterialV1,
    ray::Ray, scene::Scene,
};
use cgmath::{InnerSpace, Point2, Point3, Vector3};

pub enum LightSource {
    PointLight(PointLightSource),
}

impl LightSource {
    pub fn point_light(intensity: RgbSpectrum, position: Point3<f32>) -> LightSource {
        Self::PointLight(PointLightSource::new(intensity, position))
    }

    /// Calculate the radiance carried along the ray due to a light source
    /// without associated geometry (such as infinite area lights). Light
    /// sources with associated geometry will return no radiance.
    pub fn outgoing_radiance_onto_ray(&self, _ray: &Ray) -> RgbSpectrum {
        RgbSpectrum::constant(0.0)
    }

    /// Calculate the radiance from the light that falls on the surface at the
    /// point being shaded, ignoring occlusion and shadows.
    ///
    /// This method returns three values:
    /// * The incoming radiance from the light that falls on the surface at the
    ///   point being shaded.
    /// * The the normalized direction from the point being shaded to the light
    ///   source.
    /// * The probability that a sampler would have picked the returned incoming
    ///   direction when sampling this light.
    /// * A `VisibilityTester` for checking if any primitives block the surface
    ///   point from the light source.
    pub fn sample_incoming_radiance_at_surface(
        &self,
        interaction: &SurfaceInteraction,
        sample_point: Point2<f32>,
    ) -> (RgbSpectrum, Vector3<f32>, f32, VisibilityTester) {
        let radiance = RgbSpectrum::constant(0.0);
        let propbability = 1.0;
        todo!()
    }
}

pub struct VisibilityTester {}

impl VisibilityTester {
    pub fn unocculuded(&self, scene: &Scene) -> bool {
        true // TODO
    }
}

// #[cfg(test)]
// mod phong_shading_tests {
//     use super::{phong_shading, PointLightSource};
//     use crate::material_v1::MaterialV1;
//     use crate::test::ApproxEq;
//     use crate::{color::RgbSpectrum, light_v1::LightSource};
//     use cgmath::{Point3, Vector3};

//     #[test]
//     fn eye_between_light_and_surface() {
//         let material = MaterialV1::new(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             0.1,
//             0.9,
//             0.9,
//             200.0,
//             0.0,
//         );
//         let position = Point3::new(0.0, 0.0, 0.0);
//         let normal = Vector3::new(0.0, 0.0, -1.0);

//         let light = LightSource::point_light(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             Point3::new(0.0, 0.0, -10.0),
//         );
//         let eye = Vector3::new(0.0, 0.0, -1.0);
//         let result = phong_shading(&material, &light, &position, &eye, &normal, false);
//         assert!(result.approx_eq(&RgbSpectrum::from_rgb(1.9, 1.9, 1.9)));
//     }

//     #[test]
//     fn surface_in_shadow() {
//         let material = MaterialV1::new(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             0.1,
//             0.9,
//             0.9,
//             200.0,
//             0.0,
//         );
//         let position = Point3::new(0.0, 0.0, 0.0);
//         let normal = Vector3::new(0.0, 0.0, -1.0);

//         let light = LightSource::point_light(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             Point3::new(0.0, 0.0, -10.0),
//         );
//         let eye = Vector3::new(0.0, 0.0, -1.0);
//         let result = phong_shading(&material, &light, &position, &eye, &normal, true);
//         assert!(result.approx_eq(&RgbSpectrum::from_rgb(0.1, 0.1, 0.1)));
//     }

//     #[test]
//     fn eye_between_light_and_surface_and_offset_45_degrees() {
//         let material = MaterialV1::new(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             0.1,
//             0.9,
//             0.9,
//             200.0,
//             0.0,
//         );
//         let position = Point3::new(0.0, 0.0, 0.0);
//         let normal = Vector3::new(0.0, 0.0, -1.0);

//         let light = LightSource::point_light(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             Point3::new(0.0, 0.0, -10.0),
//         );
//         let eye = Vector3::new(0.0, f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / -2.0);
//         let result = phong_shading(&material, &light, &position, &eye, &normal, false);
//         assert!(result.approx_eq(&RgbSpectrum::from_rgb(1.0, 1.0, 1.0)));
//     }

//     #[test]
//     fn eye_opposite_surface_light_offset() {
//         let material = MaterialV1::new(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             0.1,
//             0.9,
//             0.9,
//             200.0,
//             0.0,
//         );
//         let position = Point3::new(0.0, 0.0, 0.0);
//         let normal = Vector3::new(0.0, 0.0, -1.0);

//         let light = LightSource::point_light(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             Point3::new(0.0, 10.0, -10.0),
//         );
//         let eye = Vector3::new(0.0, 0.0, -1.0);
//         let result = phong_shading(&material, &light, &position, &eye, &normal, false);
//         assert!(result.approx_eq(&RgbSpectrum::from_rgb(0.7364, 0.7364, 0.7364)));
//     }

//     #[test]
//     fn eye_in_reflection_path() {
//         let material = MaterialV1::new(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             0.1,
//             0.9,
//             0.9,
//             200.0,
//             0.0,
//         );
//         let position = Point3::new(0.0, 0.0, 0.0);
//         let normal = Vector3::new(0.0, 0.0, -1.0);

//         let light = LightSource::point_light(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             Point3::new(0.0, 10.0, -10.0),
//         );
//         let eye = Vector3::new(0.0, f32::sqrt(2.0) / -2.0, f32::sqrt(2.0) / -2.0);
//         let result = phong_shading(&material, &light, &position, &eye, &normal, false);
//         assert!(result.approx_eq(&RgbSpectrum::from_rgb(1.6364, 1.6364, 1.6364)));
//     }

//     #[test]
//     fn light_behind_surface() {
//         let material = MaterialV1::new(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             0.1,
//             0.9,
//             0.9,
//             200.0,
//             0.0,
//         );
//         let position = Point3::new(0.0, 0.0, 0.0);
//         let normal = Vector3::new(0.0, 0.0, -1.0);

//         let light = LightSource::point_light(
//             RgbSpectrum::from_rgb(1.0, 1.0, 1.0),
//             Point3::new(0.0, 10.0, 10.0),
//         );
//         let eye = Vector3::new(0.0, 0.0, -1.0);
//         let result = phong_shading(&material, &light, &position, &eye, &normal, false);
//         assert!(result.approx_eq(&RgbSpectrum::from_rgb(0.1, 0.1, 0.1)));
//     }
// }

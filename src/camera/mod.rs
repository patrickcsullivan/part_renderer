mod film;
mod projective;
mod sample;

pub use film::Film;

use crate::{
    geometry::axis::Axis2,
    medium::Medium,
    ray::{Ray, RayDifferential},
};
use cgmath::{
    Angle, InnerSpace, Matrix4, PerspectiveFov, Point2, Point3, Rad, Transform, Vector2, Vector3,
    Vector4,
};
use sample::CameraSample;

// pub struct Camera {
//     /// The time at which the camera shutter opens.
//     pub shutter_open: f32,

//     /// The time at which the camera shutter closes.
//     pub shutter_close: f32,

//     /// The final image.
//     pub film: Film,

//     /// The scattering medium in which the camera is positioned.
//     pub medium: Medium,
// }

pub trait GenerateRay {
    /// Generate a ray for the given sample.
    ///
    /// This method also returns a radiance contribution value that indicates
    /// how much the radiance carried by the ray contributes to the final image
    /// on the film plane. This value can be 1.0 for simple camera models, or it
    /// may vary for cameras that simulate physical lenses.
    fn generate_ray(&self, sample: CameraSample) -> (Ray, f32);

    /// Generate a ray for the given sample along with a ray differential, which
    /// contains two rays for samples that are shifted a small amount in the x
    /// and y directions on the film plane. This method also returns the
    /// radiance contribution for the primary ray.
    ///
    /// The default implementation tries to generate ray differentials from
    /// samples offset in the positive x direction and the positive y direction,
    /// respectively. However, a sample in the negative direction may be used if
    /// a sample in the positive direction generates a ray with zero radiance
    /// contribution.
    ///
    /// No ray differential is returned if the primary ray has no radiance
    /// contribution, if an x differential ray with a positive radiance
    /// contribution cannot be found, or if a y differential ray with a positive
    /// radiance contribution cannot be found.
    // TODO: Test me.
    fn generate_ray_differential(
        &self,
        sample: CameraSample,
    ) -> (Ray, Option<RayDifferential>, f32) {
        let (primary, radiance_contrib) = self.generate_ray(sample);
        if radiance_contrib == 0.0 {
            return (primary, None, 0.0);
        }

        let dx = self
            .differential_from_ray_along_axis(&sample, &primary, 0.05, Axis2::X)
            .or_else(|| self.differential_from_ray_along_axis(&sample, &primary, -0.05, Axis2::X));
        let dy = self
            .differential_from_ray_along_axis(&sample, &primary, 0.05, Axis2::Y)
            .or_else(|| self.differential_from_ray_along_axis(&sample, &primary, -0.05, Axis2::Y));

        if let (Some((dx_origin, dx_dir)), Some((dy_origin, dy_dir))) = (dx, dy) {
            let rd = RayDifferential::new(dx_origin, dx_dir, dy_origin, dy_dir);
            (primary, Some(rd), radiance_contrib)
        } else {
            (primary, None, radiance_contrib)
        }
    }

    // TODO: Test me.
    fn differential_from_ray_along_axis(
        &self,
        sample: &CameraSample,
        primary_ray: &Ray,
        epsilon: f32,
        axis: Axis2,
    ) -> Option<(Point3<f32>, Vector3<f32>)> {
        let shift = match axis {
            Axis2::X => Vector2::new(epsilon, 0.0),
            Axis2::Y => Vector2::new(0.0, epsilon),
        };
        Vector2::new(0.0, epsilon);
        let (diff_ray, radiance_contrib) = self.generate_ray(sample.from_film_shift(shift));
        if radiance_contrib != 0.0 {
            Some((
                primary_ray.origin + ((diff_ray.origin - primary_ray.origin) / epsilon),
                primary_ray.direction + ((diff_ray.direction - primary_ray.direction) / epsilon),
            ))
        } else {
            None
        }
    }
}

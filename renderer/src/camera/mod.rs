mod orthographic;
mod sample;

pub use {orthographic::OrthographicCamera, sample::CameraSample};

use crate::ray::{Ray, RayDifferential};

pub trait Camera {
    /// Generate a ray for the given sample.
    ///
    /// This method also returns a weight that indicates how much the radiance
    /// from this sample contributes to the final image relative to the radiance
    /// from other samples. This weight can be 1.0 for simple camera models, or
    /// it may vary for cameras that simulate physical lenses.
    fn generate_ray(&self, sample: &CameraSample) -> (Ray, f32);

    /// Generate a ray for the given sample along with a ray differential.
    ///
    /// This method also returns a weight that indicates how much the radiance
    /// from this sample contributes to the final image relative to the radiance
    /// from other samples. This weight can be 1.0 for simple camera models, or
    /// it may vary for cameras that simulate physical lenses.
    ///
    /// No ray differential is returned if the primary ray has no radiance
    /// contribution, if an x differential ray with a positive radiance
    /// contribution cannot be found, or if a y differential ray with a positive
    /// radiance contribution cannot be found.
    fn generate_ray_differential(
        &self,
        sample: &CameraSample,
    ) -> (Ray, Option<RayDifferential>, f32);
}

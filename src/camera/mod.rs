mod film;
mod orthographic;
mod pixel;
mod sample;

pub use {film::Film, orthographic::OrthographicCamera, sample::CameraSample};

use crate::ray::{Ray, RayDifferential};

pub trait Camera: GenerateRay + HasFilm {}

pub trait GenerateRay {
    /// Generate a ray for the given sample.
    ///
    /// This method also returns a radiance contribution weight that indicates
    /// how much the radiance carried by the ray contributes to the final image
    /// on the film plane. This weight can be 1.0 for simple camera models, or
    /// it may vary for cameras that simulate physical lenses.
    fn generate_ray(&self, sample: &CameraSample) -> (Ray, f32);

    /// Generate a ray for the given sample along with a ray differential.
    ///
    /// This method also returns a radiance contribution weight that indicates
    /// how much the radiance carried by the ray contributes to the final image
    /// on the film plane. This weight can be 1.0 for simple camera models, or
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

pub trait HasFilm {
    /// Return the film for the camera.
    fn film(&self) -> &Film;
}

use cgmath::Point3;

use crate::{interaction::OffsetRayOrigin, scene::Scene};

pub struct VisibilityTester {
    reference: Box<dyn OffsetRayOrigin>,
    light: Point3<f32>,
}

impl VisibilityTester {
    pub fn new(reference: Box<dyn OffsetRayOrigin>, light: Point3<f32>) -> Self {
        Self { reference, light }
    }

    /// Trace a shadow ray between the reference and the light, and return true
    /// if there is an unoccluded path between the two points.
    ///
    /// This ignores the effects of any scattering medium that the ray passes
    /// through. If the effects of a scattering medium need to be take n into
    /// account, `beam_transmittance` should be called instead.
    pub fn unocculuded(&self, scene: &Scene) -> bool {
        let ray = self.reference.spawn_shadow_ray_to_point(&self.light);
        scene.ray_intersection(&ray).is_none()
    }
}

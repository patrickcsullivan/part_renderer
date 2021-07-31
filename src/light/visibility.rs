use cgmath::{InnerSpace, Point3};

use crate::{ray::Ray, scene::Scene};

pub struct VisibilityTester {
    p0: Point3<f32>,
    p1: Point3<f32>,
}

impl VisibilityTester {
    pub fn new(p0: Point3<f32>, p1: Point3<f32>) -> Self {
        Self { p0, p1 }
    }

    /// Trace a shadow ray between `p0` and `p1`, and return true if there is an
    /// unoccluded path between the two points.
    ///
    /// This ignores the effects of any scattering medium that the ray passes
    /// through. If the effects of a scattering medium need to be taken into
    /// account, `beam_transmittance` should be called instead.
    pub fn unocculuded(&self, scene: &Scene) -> bool {
        let direction = (self.p0 - self.p1).normalize();
        let ray = Ray::new(self.p0, direction);
        scene.ray_intersection(&ray).is_none()
    }
}

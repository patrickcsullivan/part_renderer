use crate::math::vector;
use cgmath::{Point3, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct SurfaceInteraction {
    /// The point in space where the interaction occurs.
    pub point: Point3<f32>,

    /// The direction of the negative/outgoing ray.
    pub neg_ray_direction: Vector3<f32>,

    /// The surface normal at the interaction point.
    pub normal: Vector3<f32>,
}

impl SurfaceInteraction {
    pub fn new(point: Point3<f32>, neg_ray_direction: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self {
            point,
            neg_ray_direction,
            normal,
        }
    }

    pub fn over_point(&self) -> Point3<f32> {
        self.point + self.normal * 0.01 // FIXME: This adjustment value seems very high.
    }

    pub fn under_point(&self) -> Point3<f32> {
        self.point - self.normal * 0.01 // FIXME: This adjustment value seems very high.
    }

    pub fn reflect(&self) -> Vector3<f32> {
        vector::reflect(-1.0 * self.neg_ray_direction, self.normal)
    }
}

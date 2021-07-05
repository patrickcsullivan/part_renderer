use cgmath::{Point3, Vector3};

#[derive(Debug)]
pub struct SurfaceInteraction {
    /// The point in space where the interaction occurs.
    pub point: Point3<f32>,

    /// The direction of the negative/outgoing ray.
    pub neg_ray_direction: Vector3<f32>,

    /// The surface normal at the interaction point.
    pub normal: Vector3<f32>,
}

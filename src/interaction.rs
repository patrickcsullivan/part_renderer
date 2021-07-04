use crate::shape::Sphere;
use cgmath::{Point3, Vector3};

#[derive(Debug)]
pub struct SurfaceInteraction<'shp, 'mtrx, 'mtrl> {
    /// The point in space where the interaction occurs.
    pub point: Point3<f32>,

    /// The direction of the negative/outgoing ray.
    pub neg_ray_direction: Vector3<f32>,

    /// The shape that the point lies on.
    pub shape: &'shp Sphere<'mtrx, 'mtrl>,

    /// The surface normal at the interaction point.
    pub normal: Vector3<f32>,
}

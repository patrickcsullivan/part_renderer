use crate::interaction::SurfaceInteraction;
use crate::ray::Ray;
use cgmath::{InnerSpace, Matrix, Matrix4, Transform, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Plane<'mtrx> {
    pub object_to_world: &'mtrx Matrix4<f32>,
    pub world_to_object: &'mtrx Matrix4<f32>,
    pub reverse_orientation: bool,
}

impl<'mtrx> Plane<'mtrx> {
    /// Returns the plane's normal in world space.
    pub fn normal(&self) -> Vector3<f32> {
        let obj_n = Vector3::new(0.0, 1.0, 0.0);
        let n = self
            .world_to_object
            .transpose()
            .transform_vector(obj_n)
            .normalize();
        if self.reverse_orientation {
            n * 1.0
        } else {
            n
        }
    }

    /// Returns intersections between the ray and the plane in world space.
    pub fn ray_intersection(&self, ray: &Ray) -> Option<(f32, SurfaceInteraction)> {
        // Transforming the ray from world to object space is analagous to
        // transforming the sphere from object to world space.
        use crate::geometry::Transform;
        let obj_ray = self.world_to_object.transform(ray);

        if obj_ray.direction.y.abs() < 0.0001 {
            return None;
        }

        let t = -1.0 * ray.origin.y / ray.direction.y;
        if t <= 0.0 {
            return None;
        }

        let obj_p = obj_ray.at_t(t);
        let world_p = self.object_to_world.transform_point(obj_p);
        let interaction = SurfaceInteraction::new(world_p, -1.0 * ray.direction, self.normal());
        Some((t, interaction))
    }
}

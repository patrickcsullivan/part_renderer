use cgmath::{Matrix4, Point3, Transform, Vector3};

use crate::medium::Medium;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,

    /// The upper bound of t in the ray's parametric equation,
    /// r(t) = o + t*d, 0 < t < time_max
    /// Limits the ray to a finite segment.
    pub t_max: f32,

    pub medium: Medium,
}

/// Contains the origin and direction of two auxilary rays for some primary ray.
/// The auxilary rays are offset from the primary in the x and y directions,
/// respectively, on the film plane.
#[derive(Debug)]
pub struct RayDifferential {
    /// Origin of a ray that is offset from some primary ray in the x direction
    /// on the film plane.
    pub dx_origin: cgmath::Point3<f32>,

    /// Direction of a ray that is offset from some primary ray in the x
    /// direction on the film plane.
    pub dx_direction: cgmath::Vector3<f32>,

    /// Origin of a ray that is offset from some primary ray in the y direction
    /// on the film plane.
    pub dy_origin: cgmath::Point3<f32>,

    /// Direction of a ray that is offset from some primary ray in the x
    /// direction on the film plane.
    pub dy_direction: cgmath::Vector3<f32>,
}

impl RayDifferential {
    pub fn new(
        dx_origin: Point3<f32>,
        dx_direction: Vector3<f32>,
        dy_origin: Point3<f32>,
        dy_direction: Vector3<f32>,
    ) -> Self {
        Self {
            dx_origin,
            dx_direction,
            dy_origin,
            dy_direction,
        }
    }
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>, medium: Medium) -> Self {
        Self {
            origin,
            direction,
            t_max: f32::MAX,
            medium,
        }
    }

    /// Get the position along the ray for a given parametric value, `t`.
    pub fn at_t(&self, t: f32) -> Point3<f32> {
        self.origin + self.direction * t
    }
}

impl Into<bvh::ray::Ray> for &Ray {
    fn into(self) -> bvh::ray::Ray {
        bvh::ray::Ray::new(
            bvh::Point3::new(self.origin.x, self.origin.y, self.origin.z),
            bvh::Vector3::new(self.direction.x, self.direction.y, self.direction.z),
        )
    }
}

impl crate::transform::Transform<Ray> for Matrix4<f32> {
    fn transform(&self, ray: &Ray) -> Ray {
        Ray {
            origin: self.transform_point(ray.origin),
            // It's important to leave direction unnormalized so that the ray
            // can shink or grow when we apply transformations that are intended
            // to scale an object.
            direction: self.transform_vector(ray.direction),
            t_max: ray.t_max,
            medium: ray.medium,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::{medium::Medium, test::ApproxEq, transform::Transform};
    use cgmath::{Matrix4, Point3, Vector3};

    #[test]
    fn at_t() {
        let ray = Ray::new(
            Point3::new(2.0, 3.0, 4.0),
            Vector3::new(1.0, 0.0, 0.0),
            Medium::new(),
        );
        assert!(ray.at_t(0.0).approx_eq(&Point3::new(2.0, 3.0, 4.0)));
        assert!(ray.at_t(1.0).approx_eq(&Point3::new(3.0, 3.0, 4.0)));
        assert!(ray.at_t(-1.0).approx_eq(&Point3::new(1.0, 3.0, 4.0)));
        assert!(ray.at_t(2.5).approx_eq(&Point3::new(4.5, 3.0, 4.0)));
    }

    #[test]
    fn translating() {
        let ray = Ray::new(
            Point3::new(1.0, 2.0, 3.0),
            Vector3::new(0.0, 1.0, 0.0),
            Medium::new(),
        );
        let t: Matrix4<f32> = Matrix4::from_translation(Vector3::new(3.0, 4.0, 5.0));
        let ray = t.transform(&ray);
        assert!(ray.origin.approx_eq(&Point3::new(4.0, 6.0, 8.0)));
        assert!(ray.direction.approx_eq(&Vector3::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn scaling() {
        let ray = Ray::new(
            Point3::new(1.0, 2.0, 3.0),
            Vector3::new(0.0, 1.0, 0.0),
            Medium::new(),
        );
        let t: Matrix4<f32> = Matrix4::from_nonuniform_scale(2.0, 3.0, 4.0);
        let ray = t.transform(&ray);
        assert!(ray.origin.approx_eq(&Point3::new(2.0, 6.0, 12.0)));
        assert!(ray.direction.approx_eq(&Vector3::new(0.0, 3.0, 0.0)));
    }
}

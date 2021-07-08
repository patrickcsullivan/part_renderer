use crate::interaction::SurfaceInteraction;
use crate::ray::Ray;
use cgmath::{InnerSpace, Matrix, Matrix4, Point3, Transform, Vector3};

#[derive(Debug)]
pub struct Triangle {
    pub points: (Point3<f32>, Point3<f32>, Point3<f32>),
    pub p1_p2: Vector3<f32>,
    pub p1_p3: Vector3<f32>,
}

impl Triangle {
    pub fn new(p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) -> Self {
        Self {
            points: (p1, p2, p3),
            p1_p2: p2 - p1,
            p1_p3: p3 - p1,
        }
    }

    /// Returns the plane's normal in world space.
    pub fn normal(
        &self,
        world_to_object: &Matrix4<f32>,
        reverse_orientation: bool,
    ) -> Vector3<f32> {
        let obj_n = self.p1_p3.cross(self.p1_p2).normalize();
        let n = world_to_object
            .transpose()
            .transform_vector(obj_n)
            .normalize();
        if reverse_orientation {
            n * 1.0
        } else {
            n
        }
    }

    /// Returns intersections between the ray and the plane in world space.
    pub fn ray_intersections(
        &self,
        ray: &Ray,
        object_to_world: &Matrix4<f32>,
        world_to_object: &Matrix4<f32>,
        reverse_orientation: bool,
    ) -> Vec<(f32, SurfaceInteraction)> {
        use crate::transform::Transform;
        let obj_ray = world_to_object.transform(ray);

        let dir_cross_p1_p3 = obj_ray.direction.cross(self.p1_p3);
        let determinant = self.p1_p2.dot(dir_cross_p1_p3);
        if determinant.abs() < 0.0001 {
            return vec![];
        }

        let inv_determinant = 1.0 / determinant;
        let p1_to_origin = obj_ray.origin - self.points.0;
        let u = inv_determinant * p1_to_origin.dot(dir_cross_p1_p3);
        if !(0.0..=1.0).contains(&u) {
            return vec![];
        }

        let origin_cross_p1_p2 = p1_to_origin.cross(self.p1_p2);
        let v = inv_determinant * obj_ray.direction.dot(origin_cross_p1_p2);
        if v < 0.0 || u + v > 1.0 {
            return vec![];
        }

        let t = inv_determinant * self.p1_p3.dot(origin_cross_p1_p2);
        let obj_p = obj_ray.at_t(t);
        let world_p = object_to_world.transform_point(obj_p);
        let intr = (
            t,
            SurfaceInteraction {
                point: world_p,
                neg_ray_direction: -1.0 * ray.direction,
                normal: normal(world_to_object, reverse_orientation),
            },
        );
        vec![intr]
    }
}

/// Returns the plane's normal in world space.
pub fn normal(world_to_object: &Matrix4<f32>, reverse_orientation: bool) -> Vector3<f32> {
    let obj_n = Vector3::new(0.0, 1.0, 0.0);
    let n = world_to_object
        .transpose()
        .transform_vector(obj_n)
        .normalize();
    if reverse_orientation {
        n * 1.0
    } else {
        n
    }
}

#[cfg(test)]
mod ray_intersects_tests {
    use crate::matrix::identity4;
    use crate::ray::Ray;
    use crate::shape::triangle::Triangle;
    use crate::test::ApproxEq;
    use cgmath::{Point3, Vector3};

    #[test]
    fn ray_parallel_to_triangle() {
        let ray = Ray {
            origin: Point3::new(0.0, -1.0, -2.0),
            direction: Vector3::new(0.0, 1.0, 0.0),
        };
        let triangle = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );
        let identity = identity4();
        let intersections = triangle.ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let ray = Ray {
            origin: Point3::new(1.0, 1.0, -2.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let triangle = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );
        let identity = identity4();
        let intersections = triangle.ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let ray = Ray {
            origin: Point3::new(-1.0, 1.0, -2.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let triangle = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );
        let identity = identity4();
        let intersections = triangle.ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let ray = Ray {
            origin: Point3::new(0.0, -1.0, -2.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let triangle = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );
        let identity = identity4();
        let intersections = triangle.ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_strikes_triangle() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.5, -2.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let triangle = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );
        let identity = identity4();
        let intersections = triangle.ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 1);
        assert!(intersections[0].0.approx_eq(&2.0))
    }
}

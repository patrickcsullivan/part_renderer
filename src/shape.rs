use crate::ray::Ray;
use cgmath::{Point3, Vector3};

trait Shape {
    fn ray_intersection(&self, ray: &Ray) -> Vec<f32>;
}

pub struct Sphere {}

impl Shape for Sphere {
    fn ray_intersection(&self, ray: &Ray) -> Vec<f32> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::shape::Sphere;
    use cgmath::{Point3, Vector3};

    use super::Shape;

    #[test]
    fn ray_intersects_at_two_points() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = sphere.ray_intersection(&ray);
        assert_eq!(intersections.len(), 2);
        assert_approx_eq(intersections[0], 4.0);
        assert_approx_eq(intersections[1], 6.0);
    }

    #[test]
    fn ray_intersects_at_tangent() {
        let ray = Ray {
            origin: Point3::new(0.0, 1.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = sphere.ray_intersection(&ray);
        assert_eq!(intersections.len(), 2);
        assert_approx_eq(intersections[0], 5.0);
        assert_approx_eq(intersections[1], 5.0);
    }

    #[test]
    fn ray_misses() {
        let ray = Ray {
            origin: Point3::new(0.0, 2.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = sphere.ray_intersection(&ray);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = sphere.ray_intersection(&ray);
        assert_eq!(intersections.len(), 2);
        assert_approx_eq(intersections[0], -1.0);
        assert_approx_eq(intersections[1], 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = sphere.ray_intersection(&ray);
        assert_eq!(intersections.len(), 2);
        assert_approx_eq(intersections[0], -6.0);
        assert_approx_eq(intersections[1], -4.0);
    }

    fn assert_approx_eq(v1: f32, v2: f32) {
        assert!((v1 - v2).abs() < f32::EPSILON);
    }
}

use cgmath::{Matrix4, Point3, Transform, Vector3};

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    /// Get the position along the ray for a given parametric value, `t`.
    fn at_t(&self, t: f32) -> Point3<f32> {
        self.origin + self.direction * t
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::transform::Transform;
    use cgmath::{Matrix4, Point3, Vector3};

    #[test]
    fn at_t() {
        let ray = Ray {
            origin: Point3::new(2.0, 3.0, 4.0),
            direction: Vector3::new(1.0, 0.0, 0.0),
        };
        assert_eq!(ray.at_t(0.0), Point3::new(2.0, 3.0, 4.0));
        assert_eq!(ray.at_t(1.0), Point3::new(3.0, 3.0, 4.0));
        assert_eq!(ray.at_t(-1.0), Point3::new(1.0, 3.0, 4.0));
        assert_eq!(ray.at_t(2.5), Point3::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating() {
        let ray = Ray {
            origin: Point3::new(1.0, 2.0, 3.0),
            direction: Vector3::new(0.0, 1.0, 0.0),
        };
        let t: Matrix4<f32> = Matrix4::from_translation(Vector3::new(3.0, 4.0, 5.0));
        let ray = t.transform(&ray);
        assert_eq!(ray.origin, Point3::new(4.0, 6.0, 8.0));
        assert_eq!(ray.direction, Vector3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling() {
        let ray = Ray {
            origin: Point3::new(1.0, 2.0, 3.0),
            direction: Vector3::new(0.0, 1.0, 0.0),
        };
        let t: Matrix4<f32> = Matrix4::from_nonuniform_scale(2.0, 3.0, 4.0);
        let ray = t.transform(&ray);
        assert_eq!(ray.origin, Point3::new(2.0, 6.0, 12.0));
        assert_eq!(ray.direction, Vector3::new(0.0, 3.0, 0.0));
    }
}

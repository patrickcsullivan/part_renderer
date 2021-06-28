use cgmath::{Point3, Vector3};

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

#[cfg(test)]
mod tests {
    use super::Ray;
    use cgmath::{Point3, Vector3};

    #[test]
    fn at_t() {
        let ray = Ray {
            origin: Point3::new(2.0, 3.0, 4.0),
            direction: Vector3::new(1.0, 0.0, 0.0),
        };
        assert_approx_eq_point(ray.at_t(0.0), Point3::new(2.0, 3.0, 4.0));
        assert_approx_eq_point(ray.at_t(1.0), Point3::new(3.0, 3.0, 4.0));
        assert_approx_eq_point(ray.at_t(-1.0), Point3::new(1.0, 3.0, 4.0));
        assert_approx_eq_point(ray.at_t(2.5), Point3::new(4.5, 3.0, 4.0));
    }

    fn assert_approx_eq_point(p1: Point3<f32>, p2: Point3<f32>) {
        assert!((p1.x - p2.x).abs() < f32::EPSILON);
        assert!((p1.y - p2.y).abs() < f32::EPSILON);
        assert!((p1.z - p2.z).abs() < f32::EPSILON);
    }
}

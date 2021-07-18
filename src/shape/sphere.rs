use crate::interaction::SurfaceInteraction;
use crate::ray::Ray;
use cgmath::{InnerSpace, Matrix, Matrix4, Point3, Transform, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Sphere<'mtrx> {
    pub object_to_world: &'mtrx Matrix4<f32>,
    pub world_to_object: &'mtrx Matrix4<f32>,
    pub reverse_orientation: bool,
}

impl<'mtrx> Sphere<'mtrx> {
    /// Returns the normal in world space at a given point on the sphere in
    /// world space.
    pub fn normal_at(&self, p: Point3<f32>) -> Vector3<f32> {
        let obj_p = self.world_to_object.transform_point(p);
        let obj_n = obj_p - Point3::new(0.0, 0.0, 0.0);
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

    pub fn ray_intersection(&self, ray: &Ray) -> Option<(f32, SurfaceInteraction)> {
        // Transforming the ray from world to object space is analagous to
        // transforming the sphere from object to world space.
        use crate::geometry::Transform;
        let obj_ray = self.world_to_object.transform(ray);

        let sphere_to_ray = obj_ray.origin - Point3::new(0.0, 0.0, 0.0);
        let a = obj_ray.direction.dot(obj_ray.direction);
        let b = 2.0 * obj_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-1.0 * b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-1.0 * b + discriminant.sqrt()) / (2.0 * a);
        if t1 <= 0.0 && t2 <= 0.0 {
            return None;
        }

        let t = if t1 <= 0.0 {
            t2
        } else if t2 <= 0.0 {
            t1
        } else {
            t1.min(t2)
        };
        let obj_p = obj_ray.at_t(t);
        let world_p = self.object_to_world.transform_point(obj_p);
        let world_neg_ray_direction = ray.direction * -1.0;
        let interaction = SurfaceInteraction {
            point: world_p,
            neg_ray_direction: world_neg_ray_direction,
            normal: self.normal_at(world_p),
        };
        Some((t, interaction))
    }
}

#[cfg(test)]
mod ray_intersects_tests {
    use crate::geometry::matrix::identity4;
    use crate::ray::Ray;
    use crate::shape::sphere::Sphere;
    use crate::test::ApproxEq;
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    #[test]
    fn interactions_includes_surface_calculation() -> Result<(), String> {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &identity,
            world_to_object: &identity,
            reverse_orientation,
        };

        if let Some((t, interaction)) = sphere.ray_intersection(&ray) {
            assert!(t.approx_eq(&4.0));
            assert!(interaction.point.approx_eq(&Point3::new(0.0, 0.0, -1.0)));
            assert!(interaction.normal.approx_eq(&Vector3::new(0.0, 0.0, -1.0)));
            assert!(interaction
                .neg_ray_direction
                .approx_eq(&Vector3::new(0.0, 0.0, -1.0)));
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }

        // assert!(intersections[1].0.approx_eq(&6.0));
        // assert!(intersections[1]
        //     .1
        //     .point
        //     .approx_eq(&Point3::new(0.0, 0.0, 1.0)));
        // assert!(intersections[1]
        //     .1
        //     .normal
        //     .approx_eq(&Vector3::new(0.0, 0.0, 1.0)));
        // assert!(intersections[1]
        //     .1
        //     .neg_ray_direction
        //     .approx_eq(&Vector3::new(0.0, 0.0, -1.0)));
    }

    #[test]
    fn intersects_at_two_points() -> Result<(), String> {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &identity,
            world_to_object: &identity,
            reverse_orientation,
        };

        if let Some((t, _)) = sphere.ray_intersection(&ray) {
            assert!(t.approx_eq(&4.0));
            // Next point would be at 6.0.
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }
    }

    #[test]
    fn intersects_at_tangent_of_untransformed_sphere() -> Result<(), String> {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &identity,
            world_to_object: &identity,
            reverse_orientation,
        };

        if let Some((t, _)) = sphere.ray_intersection(&ray) {
            assert!(t.approx_eq(&5.0));
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }
    }

    #[test]
    fn intersects_at_tangent_of_transformed_sphere() -> Result<(), String> {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let obj_to_world = Matrix4::from_translation(Vector3::new(0.0, -1.0, 0.0));
        let world_to_obj = Matrix4::from_translation(Vector3::new(0.0, 1.0, 0.0));
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &obj_to_world,
            world_to_object: &world_to_obj,
            reverse_orientation,
        };

        if let Some((t, _)) = sphere.ray_intersection(&ray) {
            assert!(t.approx_eq(&5.0));
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }
    }

    #[test]
    fn misses() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &identity,
            world_to_object: &identity,
            reverse_orientation,
        };

        assert!(
            sphere.ray_intersection(&ray).is_none(),
            "Expected to not find intersection."
        );
    }

    #[test]
    fn ray_originates_inside_sphere() -> Result<(), String> {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &identity,
            world_to_object: &identity,
            reverse_orientation,
        };

        if let Some((t, _)) = sphere.ray_intersection(&ray) {
            assert!(t.approx_eq(&-1.0)); // FIXME: Ray shouldn't find negative t's. It should find t=1.0 instead.
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }
    }

    #[test]
    fn sphere_is_behind_ray() -> Result<(), String> {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &identity,
            world_to_object: &identity,
            reverse_orientation,
        };

        if let Some((t, _)) = sphere.ray_intersection(&ray) {
            assert!(t.approx_eq(&-4.0)); // FIXME: Ray shouldn't find negative t's. It should get None instead.
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }
    }

    #[test]
    fn intersects_scaled_sphere() -> Result<(), String> {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let obj_to_world = Matrix4::from_scale(2.0);
        let world_to_obj = obj_to_world.inverse_transform().unwrap();
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &obj_to_world,
            world_to_object: &world_to_obj,
            reverse_orientation,
        };

        if let Some((t, _)) = sphere.ray_intersection(&ray) {
            assert!(t.approx_eq(&3.0));
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }
    }

    #[test]
    fn misses_translated_sphere() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let obj_to_world = Matrix4::from_translation(Vector3::new(5.0, 0.0, 0.0));
        let world_to_obj = obj_to_world.inverse_transform().unwrap();
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &obj_to_world,
            world_to_object: &world_to_obj,
            reverse_orientation,
        };

        assert!(
            sphere.ray_intersection(&ray).is_none(),
            "Expected to not find intersection."
        );
    }
}

#[cfg(test)]
mod normal_tests {
    use crate::geometry::matrix::identity4;
    use crate::shape::sphere::Sphere;
    use crate::test::ApproxEq;
    use cgmath::{Matrix4, Point3, Rad, Transform, Vector3};

    #[test]
    fn at_nonaxial_point() {
        let point = Point3::new(
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
        );
        let identity = identity4();
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &identity,
            world_to_object: &identity,
            reverse_orientation,
        };
        let normal = sphere.normal_at(point);
        let expected = Vector3::new(
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
        );
        assert!(normal.approx_eq(&expected));
    }

    #[test]
    fn on_transformed_sphere() {
        let point = Point3::new(0.0, f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / -2.0);
        let obj_to_world = Matrix4::from_nonuniform_scale(1.0, 0.5, 1.0)
            * Matrix4::from_angle_z(Rad(std::f32::consts::PI / 5.0));
        let world_to_obj = obj_to_world.inverse_transform().unwrap();
        let reverse_orientation = false;
        let sphere = Sphere {
            object_to_world: &obj_to_world,
            world_to_object: &world_to_obj,
            reverse_orientation,
        };
        let normal = sphere.normal_at(point);
        let expected = Vector3::new(0.0, 0.97014, -0.24254);
        assert!(normal.approx_eq(&expected));
    }
}

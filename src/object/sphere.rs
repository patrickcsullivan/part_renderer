use crate::interaction::SurfaceInteraction;
use crate::ray::Ray;
use cgmath::{InnerSpace, Matrix, Matrix4, Point3, Transform, Vector3};

/// Returns the normal in world space at a given point on the sphere in
/// world space.
pub fn normal_at(
    p: Point3<f32>,
    world_to_object: &Matrix4<f32>,
    reverse_orientation: bool,
) -> Vector3<f32> {
    let obj_p = world_to_object.transform_point(p);
    let obj_n = obj_p - Point3::new(0.0, 0.0, 0.0);
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

pub fn ray_intersections(
    ray: &Ray,
    object_to_world: &Matrix4<f32>,
    world_to_object: &Matrix4<f32>,
    reverse_orientation: bool,
) -> Vec<(f32, SurfaceInteraction)> {
    // Transforming the ray from world to object space is analagous to
    // transforming the sphere from object to world space.
    use crate::transform::Transform;
    let obj_ray = world_to_object.transform(ray);

    let sphere_to_ray = obj_ray.origin - Point3::new(0.0, 0.0, 0.0);
    let a = obj_ray.direction.dot(obj_ray.direction);
    let b = 2.0 * obj_ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        vec![]
    } else {
        let t1 = (-1.0 * b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-1.0 * b + discriminant.sqrt()) / (2.0 * a);

        let obj_p1 = obj_ray.at_t(t1);
        let obj_p2 = obj_ray.at_t(t2);

        let world_p1 = object_to_world.transform_point(obj_p1);
        let world_p2 = object_to_world.transform_point(obj_p2);

        let world_neg_ray_direction = ray.direction * -1.0;

        let intr1 = (
            t1,
            SurfaceInteraction {
                point: world_p1,
                neg_ray_direction: world_neg_ray_direction,
                normal: normal_at(world_p1, world_to_object, reverse_orientation),
            },
        );
        let intr2 = (
            t2,
            SurfaceInteraction {
                point: world_p2,
                neg_ray_direction: world_neg_ray_direction,
                normal: normal_at(world_p2, world_to_object, reverse_orientation),
            },
        );

        vec![intr1, intr2]
    }
}

#[cfg(test)]
mod ray_intersects_tests {
    use crate::color::Rgb;
    use crate::material::Material;
    use crate::matrix::identity4;
    use crate::object::sphere::ray_intersections;
    use crate::ray::Ray;
    use crate::test::ApproxEq;
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    fn test_material() -> Material {
        Material {
            color: Rgb::new(0.0, 0.0, 0.0),
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
            reflective: 0.0,
        }
    }

    #[test]
    fn interactions_includes_surface_calculation() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let intersections = ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 2);

        assert!(intersections[0].0.approx_eq(&4.0));
        assert!(intersections[0]
            .1
            .point
            .approx_eq(&Point3::new(0.0, 0.0, -1.0)));
        assert!(intersections[0]
            .1
            .normal
            .approx_eq(&Vector3::new(0.0, 0.0, -1.0)));
        assert!(intersections[0]
            .1
            .neg_ray_direction
            .approx_eq(&Vector3::new(0.0, 0.0, -1.0)));

        assert!(intersections[1].0.approx_eq(&6.0));
        assert!(intersections[1]
            .1
            .point
            .approx_eq(&Point3::new(0.0, 0.0, 1.0)));
        assert!(intersections[1]
            .1
            .normal
            .approx_eq(&Vector3::new(0.0, 0.0, 1.0)));
        assert!(intersections[1]
            .1
            .neg_ray_direction
            .approx_eq(&Vector3::new(0.0, 0.0, -1.0)));
    }

    #[test]
    fn intersects_at_two_points() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let intersections = ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 2);
        assert!(intersections[0].0.approx_eq(&4.0));
        assert!(intersections[1].0.approx_eq(&6.0));
    }

    #[test]
    fn intersects_at_tangent_of_untransformed_sphere() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let intersections = ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 2);

        assert!(intersections[0].0.eq(&5.0));
        assert!(intersections[0]
            .1
            .normal
            .approx_eq(&Vector3::new(0.0, 1.0, 0.0)));

        assert!(intersections[1].0.eq(&5.0));
        assert!(intersections[1]
            .1
            .normal
            .approx_eq(&Vector3::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn intersects_at_tangent_of_transformed_sphere() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let obj_to_world = Matrix4::from_translation(Vector3::new(0.0, -1.0, 0.0));
        let world_to_obj = Matrix4::from_translation(Vector3::new(0.0, 1.0, 0.0));
        let reverse_orientation = false;
        let intersections = ray_intersections(&ray, &obj_to_world, &world_to_obj, false);

        assert_eq!(intersections.len(), 2);

        assert!(intersections[0].0.eq(&5.0));
        assert!(intersections[0]
            .1
            .normal
            .approx_eq(&Vector3::new(0.0, 1.0, 0.0)));

        assert!(intersections[1].0.eq(&5.0));
        assert!(intersections[1]
            .1
            .normal
            .approx_eq(&Vector3::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn misses() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let intersections = ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let intersections = ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 2);
        assert!(intersections[0].0.eq(&-1.0));
        assert!(intersections[1].0.eq(&1.0));
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let identity = identity4();
        let reverse_orientation = false;
        let intersections = ray_intersections(&ray, &identity, &identity, false);
        assert_eq!(intersections.len(), 2);
        assert!(intersections[0].0.eq(&-6.0));
        assert!(intersections[1].0.eq(&-4.0));
    }

    #[test]
    fn intersects_scaled_sphere() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let obj_to_world = Matrix4::from_scale(2.0);
        let world_to_obj = obj_to_world.inverse_transform().unwrap();
        let reverse_orientation = false;
        let intersections = ray_intersections(&ray, &obj_to_world, &world_to_obj, false);
        assert_eq!(intersections.len(), 2);
        assert!(intersections[0].0.eq(&3.0));
        assert!(intersections[1].0.eq(&7.0));
    }

    #[test]
    fn misses_translated_sphere() {
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let obj_to_world = Matrix4::from_translation(Vector3::new(5.0, 0.0, 0.0));
        let world_to_obj = obj_to_world.inverse_transform().unwrap();
        let reverse_orientation = false;
        let intersections = ray_intersections(&ray, &obj_to_world, &world_to_obj, false);
        assert_eq!(intersections.len(), 0);
    }
}

#[cfg(test)]
mod normal_tests {
    use crate::color::Rgb;
    use crate::material::Material;
    use crate::matrix::identity4;
    use crate::object::sphere::normal_at;
    use crate::test::ApproxEq;
    use cgmath::{Matrix4, Point3, Rad, Transform, Vector3};

    fn test_material() -> Material {
        Material {
            color: Rgb::new(0.0, 0.0, 0.0),
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
            reflective: 0.0,
        }
    }

    #[test]
    fn at_nonaxial_point() {
        let point = Point3::new(
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
        );
        let identity = identity4();
        let reverse_orientation = false;
        let normal = normal_at(point, &identity, reverse_orientation);
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
        let normal = normal_at(point, &world_to_obj, reverse_orientation);
        let expected = Vector3::new(0.0, 0.97014, -0.24254);
        assert!(normal.approx_eq(&expected));
    }
}

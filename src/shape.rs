use crate::interaction::SurfaceInteraction;
use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::ray::Ray;
use cgmath::{InnerSpace, Matrix, Matrix4, Point3, Vector3};
use std::fmt::Debug;

/// Describes the geometric properties of a primitive and provides a ray
/// intersection function.
pub trait Shape<'shp, 'mtrx, 'mtrl>: Debug {
    /// Returns a reference to the matrix that transforms the shape from object
    /// space to world space.
    fn object_to_world(&self) -> &'mtrx cgmath::Matrix4<f32>;

    /// Returns a reference to the matrix that transforms the shape from world
    /// space to object space.
    fn world_to_object(&self) -> &'mtrx cgmath::Matrix4<f32>;

    fn ray_intersections(&'shp self, ray: &Ray) -> Intersections<'shp, 'mtrx, 'mtrl>;

    // TODO: Eventually, store the material/shading properties in a Primitive
    // instead of in the Shape.
    fn material(&self) -> &'mtrl Material;
}

impl<'shp, 'mtrx, 'mtrl, T> Shape<'shp, 'mtrx, 'mtrl> for &T
where
    T: Shape<'shp, 'mtrx, 'mtrl>,
{
    fn object_to_world(&self) -> &'mtrx cgmath::Matrix4<f32> {
        (*self).object_to_world()
    }

    fn world_to_object(&self) -> &'mtrx cgmath::Matrix4<f32> {
        (*self).world_to_object()
    }

    fn ray_intersections(&'shp self, ray: &Ray) -> Intersections<'shp, 'mtrx, 'mtrl> {
        (*self).ray_intersections(ray)
    }

    fn material(&self) -> &'mtrl Material {
        (*self).material()
    }
}

#[derive(Debug)]
pub struct Sphere<'mtrx, 'mtrl> {
    object_to_world: &'mtrx Matrix4<f32>,
    world_to_object: &'mtrx Matrix4<f32>,
    material: &'mtrl Material,
}

impl<'mtrx, 'mtrl> Sphere<'mtrx, 'mtrl> {
    pub fn new(
        object_to_world: &'mtrx Matrix4<f32>,
        world_to_object: &'mtrx Matrix4<f32>,
        material: &'mtrl Material,
    ) -> Self {
        Sphere {
            object_to_world,
            world_to_object,
            material,
        }
    }

    pub fn normal_at(&self, p: Point3<f32>) -> Vector3<f32> {
        use cgmath::Transform;
        let obj_p = self.world_to_object.transform_point(p);
        let obj_n = obj_p - Point3::new(0.0, 0.0, 0.0);
        self.world_to_object
            .transpose()
            .transform_vector(obj_n)
            .normalize()
    }
}

impl<'shp, 'mtrx, 'mtrl> Shape<'shp, 'mtrx, 'mtrl> for Sphere<'mtrx, 'mtrl> {
    fn object_to_world(&self) -> &'mtrx cgmath::Matrix4<f32> {
        self.object_to_world
    }

    fn world_to_object(&self) -> &'mtrx cgmath::Matrix4<f32> {
        self.world_to_object
    }

    fn ray_intersections(&'shp self, ray: &Ray) -> Intersections<'shp, 'mtrx, 'mtrl> {
        // Transforming the ray from world to object space is analagous to
        // transforming the sphere from object to world space.
        use crate::transform::Transform;
        let ray = self.world_to_object.transform(ray);

        let sphere_to_ray = ray.origin - Point3::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersections::empty()
        } else {
            let t1 = (-1.0 * b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-1.0 * b + discriminant.sqrt()) / (2.0 * a);

            let intr1 = Intersection {
                t: t1,
                interaction: SurfaceInteraction {
                    shape: Box::new(self),
                },
            };
            let intr2 = Intersection {
                t: t2,
                interaction: SurfaceInteraction {
                    shape: Box::new(self),
                },
            };

            Intersections::new(vec![intr1, intr2])
        }
    }

    fn material(&self) -> &'mtrl Material {
        self.material
    }
}

#[cfg(test)]
mod tests {
    use crate::material::Material;
    use crate::matrix::identity4;
    use crate::ray::Ray;
    use crate::shape::Sphere;
    use crate::test::ApproxEq;
    use cgmath::{Matrix4, Point3, Rad, Transform, Vector3};

    use super::Shape;

    #[test]
    fn ray_intersects_at_two_points() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let identity = identity4();
        let material = Material::default();
        let sphere = Sphere::new(&identity, &identity, &material);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 2);
        assert!(intersections.values[0].t.approx_eq(&4.0));
        assert!(intersections.values[1].t.approx_eq(&6.0));
    }

    #[test]
    fn ray_intersects_at_tangent() {
        let ray = Ray {
            origin: Point3::new(0.0, 1.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let identity = identity4();
        let material = Material::default();
        let sphere = Sphere::new(&identity, &identity, &material);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 2);
        assert!(intersections.values[0].t.eq(&5.0));
        assert!(intersections.values[1].t.eq(&5.0));
    }

    #[test]
    fn ray_misses() {
        let ray = Ray {
            origin: Point3::new(0.0, 2.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let identity = identity4();
        let material = Material::default();
        let sphere = Sphere::new(&identity, &identity, &material);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let identity = identity4();
        let material = Material::default();
        let sphere = Sphere::new(&identity, &identity, &material);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 2);
        assert!(intersections.values[0].t.eq(&-1.0));
        assert!(intersections.values[1].t.eq(&1.0));
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let identity = identity4();
        let material = Material::default();
        let sphere = Sphere::new(&identity, &identity, &material);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 2);
        assert!(intersections.values[0].t.eq(&-6.0));
        assert!(intersections.values[1].t.eq(&-4.0));
    }

    #[test]
    fn ray_intersects_scaled_sphere() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let obj_to_world = Matrix4::from_scale(2.0);
        let world_to_obj = obj_to_world.inverse_transform().unwrap();
        let material = Material::default();
        let sphere = Sphere::new(&obj_to_world, &world_to_obj, &material);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 2);
        assert!(intersections.values[0].t.eq(&3.0));
        assert!(intersections.values[1].t.eq(&7.0));
    }

    #[test]
    fn ray_misses_translated_sphere() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let obj_to_world = Matrix4::from_translation(Vector3::new(5.0, 0.0, 0.0));
        let world_to_obj = obj_to_world.inverse_transform().unwrap();
        let material = Material::default();
        let sphere = Sphere::new(&obj_to_world, &world_to_obj, &material);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 0);
    }

    #[test]
    fn normal_at_nonaxial_point() {
        let identity = identity4();
        let material = Material::default();
        let sphere = Sphere::new(&identity, &identity, &material);
        let point = Point3::new(
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
        );

        let normal = sphere.normal_at(point);
        let expected = Vector3::new(
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
        );
        assert!(normal.approx_eq(&expected));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let obj_to_world = Matrix4::from_nonuniform_scale(1.0, 0.5, 1.0)
            * Matrix4::from_angle_z(Rad(std::f32::consts::PI / 5.0));
        let world_to_obj = obj_to_world.inverse_transform().unwrap();
        let material = Material::default();
        let sphere = Sphere::new(&obj_to_world, &world_to_obj, &material);
        let point = Point3::new(0.0, f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / -2.0);

        let normal = sphere.normal_at(point);
        let expected = Vector3::new(0.0, 0.97014, -0.24254);
        assert!(normal.approx_eq(&expected));
    }
}

mod sphere;

pub use sphere::Sphere;

use crate::interaction::SurfaceInteraction;
use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::ray::Ray;
use cgmath::{InnerSpace, Matrix, Matrix4, Point3, Transform, Vector3};
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

    /// Returns a flag that indicates whether the shape's normals should be
    /// flipped from their original directions in order to point to the outside
    /// of the shape.
    fn reverse_orientation(&self) -> bool;

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

    fn reverse_orientation(&self) -> bool {
        (*self).reverse_orientation()
    }

    fn ray_intersections(&'shp self, ray: &Ray) -> Intersections<'shp, 'mtrx, 'mtrl> {
        (*self).ray_intersections(ray)
    }

    fn material(&self) -> &'mtrl Material {
        (*self).material()
    }
}

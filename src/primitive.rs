use crate::intersection::Intersections;
use crate::material::Material;
use crate::ray::Ray;

/// Describes the geometric and appearance properties of a primitive. Acts as a
/// bridge between geometry processing and shading logic.
pub trait Primitive<'shp, 'mtrx, 'mtrl> {
    // /// Returns a reference to the geometric properties of the primitive.
    // fn shape(&self) -> &'shp Sphere<'mtrx>;

    // /// Returns a reference to the appearance properties of the primitive.
    // fn material(&self) -> &();

    fn ray_intersections(&'shp self, ray: &Ray) -> Intersections<'shp, 'mtrx>;

    fn material(&self) -> &'mtrl Material;
}

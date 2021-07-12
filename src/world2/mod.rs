//! Contains data structures that can be used to construct a renderable 3D
//! world.
use crate::{interaction::SurfaceInteraction, material::Material, object::Object, ray::Ray};

/// This is the core trait that any data structure must implement in order to be
/// a renderable world. The trait allows a caller to cast a ray into the data
/// structure and find the shape, material, and surface geometry details at the
/// intersection.
pub trait RayIntersect<'msh, 'shp, 'mtrx, 'mtrl> {
    fn ray_intersection(
        &self,
        ray: &mut Ray,
    ) -> Option<(Primitive<'msh, 'shp, 'mtrx, 'mtrl>, SurfaceInteraction)>;
}

/// Combines a reference to a shape and a reference to a material. This is the
/// basic primitive used in the construction of any 3D world. Other data
/// structures may combine primitives to construct more complex worlds.
#[derive(Debug, Clone)]
pub struct Primitive<'msh, 'shp, 'mtrx, 'mtrl> {
    pub shape: &'shp Object<'msh, 'mtrx>,
    pub material: &'mtrl Material,
}

impl<'msh, 'shp, 'mtrx, 'mtrl> Primitive<'msh, 'shp, 'mtrx, 'mtrl> {
    pub fn new(shape: &'shp Object<'msh, 'mtrx>, material: &'mtrl Material) -> Self {
        Self { shape, material }
    }
}

impl<'msh, 'shp, 'mtrx, 'mtrl> RayIntersect<'msh, 'shp, 'mtrx, 'mtrl>
    for Primitive<'msh, 'shp, 'mtrx, 'mtrl>
{
    fn ray_intersection(
        &self,
        ray: &mut Ray,
    ) -> Option<(Primitive<'msh, 'shp, 'mtrx, 'mtrl>, SurfaceInteraction)> {
        todo!()
    }
}

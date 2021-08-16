mod aggregate;

pub use aggregate::PrimitiveAggregate;

use crate::{material::Material, triangle::Triangle};
use bvh::{aabb::Bounded, bounding_hierarchy::BHShape};

/// Combines a shape and a reference to a material. This is the basic primitive
/// used in the construction of primitives aggregates.
#[derive(Clone, Copy)]
pub struct Primitive<'msh, 'mtrl> {
    pub shape: Triangle<'msh>,
    pub material: &'mtrl (dyn Material + Send + Sync),

    /// Tracks the index of the primitives in a bounding volume
    /// hierarchy if it is stored in one.
    bvh_node_index: usize,
}

impl<'msh, 'mtrl> Primitive<'msh, 'mtrl> {
    pub fn new(shape: Triangle<'msh>, material: &'mtrl (dyn Material + Send + Sync)) -> Self {
        Self {
            shape,
            material,
            bvh_node_index: 0,
        }
    }
}

impl<'msh, 'mtrl> Bounded for Primitive<'msh, 'mtrl> {
    fn aabb(&self) -> bvh::aabb::AABB {
        self.shape.aabb()
    }
}

impl<'msh, 'mtrl> BHShape for Primitive<'msh, 'mtrl> {
    fn set_bh_node_index(&mut self, index: usize) {
        self.bvh_node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.bvh_node_index
    }
}

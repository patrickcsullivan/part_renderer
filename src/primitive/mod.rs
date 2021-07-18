mod aggregate;

pub use aggregate::PrimitiveAggregate;

use crate::{material::Material, shape::Shape};
use bvh::{aabb::Bounded, bounding_hierarchy::BHShape};

/// Combines a shape and a reference to a material. This is the basic primitive
/// used in the construction of primitives aggregates.
#[derive(Debug, Clone, Copy)]
pub struct Primitive<'msh, 'mtrx, 'mtrl> {
    pub shape: Shape<'msh, 'mtrx>,
    pub material: &'mtrl Material,

    /// Tracks the index of the primitives in a bounding volume
    /// hierarchy if it is stored in one.
    bvh_node_index: usize,
}

impl<'msh, 'mtrx, 'mtrl> Primitive<'msh, 'mtrx, 'mtrl> {
    pub fn new(shape: Shape<'msh, 'mtrx>, material: &'mtrl Material) -> Self {
        Self {
            shape,
            material,
            bvh_node_index: 0,
        }
    }
}

impl<'msh, 'mtrx, 'mtrl> Bounded for Primitive<'msh, 'mtrx, 'mtrl> {
    fn aabb(&self) -> bvh::aabb::AABB {
        self.shape.aabb()
    }
}

impl<'msh, 'mtrx, 'mtrl> BHShape for Primitive<'msh, 'mtrx, 'mtrl> {
    fn set_bh_node_index(&mut self, index: usize) {
        self.bvh_node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.bvh_node_index
    }
}

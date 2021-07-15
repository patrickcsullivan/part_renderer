mod builder_node;
mod linear_node;

use crate::{bounding_box::Bounds3, math::axis::Axis3, renderable::Primitive};
use builder_node::BuilderNode;
use cgmath::Point3;
use typed_arena::Arena;

pub struct BoundingVolumeHierarchy<'msh, 'mtrx, 'mtrl> {
    primitives: Vec<Primitive<'msh, 'mtrx, 'mtrl>>,
}

struct PrimitiveInfo {
    /// Index of the primitive in the original list of primitives.
    primitive_index: usize,

    /// An axis-aligned bounding box in world space.
    bounds: Bounds3<f32>,

    /// The bounding box centroid in world space.
    centroid: Point3<f32>,
}

impl PrimitiveInfo {
    fn new(index: usize, primitive: &Primitive) -> Self {
        let bounds = primitive.shape.world_bounds();

        Self {
            primitive_index: index,
            bounds,
            centroid: bounds.cetroid(),
        }
    }
}

impl<'msh, 'mtrx, 'mtrl> BoundingVolumeHierarchy<'msh, 'mtrx, 'mtrl> {
    pub fn new(
        max_primitives_in_node: usize,
        primitives: Vec<Primitive<'msh, 'mtrx, 'mtrl>>,
    ) -> Self {
        let primitives_info: Vec<PrimitiveInfo> = primitives
            .iter()
            .enumerate()
            .map(|(i, p)| PrimitiveInfo::new(i, p))
            .collect();

        let mut total_nodes: usize = 0;
        let node_arena: Arena<BuilderNode> = Arena::new();

        let mut ordered_primitive_indices: Vec<usize> = vec![];
        let ordered_primitives = ordered_primitive_indices
            .into_iter()
            .map(|i| primitives[i].clone());

        todo!();
    }
}

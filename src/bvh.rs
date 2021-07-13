use crate::renderable::Primitive;

pub struct BoundingVolumeHierarchy {
    max_primitives_in_node: usize,
    primitives: Vec<Primitive>,
}

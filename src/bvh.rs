use crate::{bounding_box::Bounds3, math::axis::Axis3, renderable::Primitive};
use cgmath::Point3;
use typed_arena::Arena;

pub struct BoundingVolumeHierarchy<'msh, 'mtrx, 'mtrl> {
    max_primitives_in_node: usize,
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

/// A node in a bounding volume hierarchy tree. The structure of this nodes is
/// convenient for building a bounding volume hierarchy, but it is less
/// condensed than the structure `FlatNode`. Therefore, when building a bounding
/// volume hierarchy, we first construct the tree out of `BuilderNode`s before
/// transforming the tree into `FlatNode`s.
enum BuilderNode<'arena> {
    Interior {
        /// An bounding box in world space of all children beneath the node.
        bounds: Bounds3<f32>,

        /// The axis along which primitives were partitioned into the two child
        /// nodes.
        partition_axis: Axis3,

        left_child: &'arena BuilderNode<'arena>,

        right_child: &'arena BuilderNode<'arena>,
    },
    Leaf {
        /// An bounding box in world space of all primitives in the node.
        bounds: Bounds3<f32>,

        /// An index into the ordered vector of primitive references of the
        /// first primitive stored in the leaf.
        first_index: usize,

        /// The number of primitives stored in the leaf.
        num_primitives: usize,
    },
}

impl<'arena, 'msh, 'mtrx, 'mtrl> BuilderNode<'arena> {
    fn leaf(first_index: usize, num_primitives: usize, bounds: Bounds3<f32>) -> Self {
        Self::Leaf {
            bounds,
            first_index,
            num_primitives,
        }
    }

    fn interior(
        partition_axis: Axis3,
        left_child: &'arena BuilderNode<'arena>,
        right_child: &'arena BuilderNode<'arena>,
        bounds: Bounds3<f32>,
    ) -> Self {
        Self::Interior {
            bounds,
            partition_axis,
            left_child,
            right_child,
        }
    }

    /// Constructs a tree of `BuilderNode`s for the subset of `PrimitiveInfo`
    /// structs in the index range [`start`, `end`] and returns the root of the
    /// constructed tree and the total number of nodes in the tree. A reference
    /// to each primitive in the constructed tree is added to
    /// `ordered_primitives`. All nodes are allocated in the given arena.
    ///
    /// The range of primitives specified by `start` and `end` must contain at
    /// least one primitive; this will panic otherwise.
    fn build_subtree(
        arena: &'arena Arena<BuilderNode<'arena>>,
        primitives_info: &[PrimitiveInfo],
        start: usize,
        end: usize,
        ordered_primitive_indices: &mut Vec<usize>,
    ) -> (&'arena Self, usize) {
        let num_primitives = end - start;

        // If there is only one primitive, so just create a leaf containing a
        // simgle primitive.
        if num_primitives == 1 {
            let node = Self::build_leaf(
                arena,
                primitives_info,
                start,
                end,
                ordered_primitive_indices,
            );
            return (node, 1);
        }
        // At this point we can assume there are least two primitives.

        let centroid_bounds = Self::centroid_bounds(primitives_info, start, end);

        // If all primitives have the same centroid (resulting in centroid
        // bounds with zero volume), then there's no good way to partition
        // them, so put all primitives in a single leaf.
        if centroid_bounds.min() == centroid_bounds.max() {
            let node = Self::build_leaf(
                arena,
                primitives_info,
                start,
                end,
                ordered_primitive_indices,
            );
            return (node, 1);
        }

        // We will partition the primitives along the axis for which
        // primitive centroids have the greatest range.
        let partition_axis = centroid_bounds.maximum_extend();

        let mid = (start + end) / 2;
        // TODO: Partition primitives.

        let (left_child, left_size) = Self::build_subtree(
            arena,
            primitives_info,
            start,
            mid,
            ordered_primitive_indices,
        );
        let (right_child, right_size) =
            Self::build_subtree(arena, primitives_info, mid, end, ordered_primitive_indices);
        let bounds = Self::primitives_bounds(primitives_info, start, end);
        let parent = arena.alloc(Self::interior(
            partition_axis,
            left_child,
            right_child,
            bounds,
        ));
        (parent, left_size + right_size + 1)
    }

    /// Constructs a leaf node that contains the primitives identified by the
    /// subset of `PrimitiveInfo` structs in the index range [`start`, `end`].
    /// Returns a reference to the new leaf node and adds a reference to each
    /// primitive in the leaf to `ordered_primitives`. The new leaf node is
    /// allocated in the given arena.
    ///
    /// The range of primitives specified by `start` and `end` must contain at
    /// least one primitive; this will panic otherwise.
    fn build_leaf(
        arena: &'arena Arena<BuilderNode<'arena>>,
        primitives_info: &[PrimitiveInfo],
        start: usize,
        end: usize,
        ordered_primitive_indices: &mut Vec<usize>,
    ) -> &'arena Self {
        let bounds = Self::primitives_bounds(primitives_info, start, end);
        let node = arena.alloc(Self::leaf(
            ordered_primitive_indices.len(),
            end - start,
            bounds,
        ));
        let mut new_ordered_primitive_indices: Vec<usize> = (start..end)
            .map(|i| primitives_info[i].primitive_index)
            .collect();
        ordered_primitive_indices.append(&mut new_ordered_primitive_indices);
        return node;
    }

    /// Returns a bounding box for the primitives in the
    /// specified range.
    ///
    /// The range of primitives specified by `start` and `end` must contain at
    /// least one primitive; this will panic otherwise.
    fn primitives_bounds(
        primitives_info: &[PrimitiveInfo],
        start: usize,
        end: usize,
    ) -> Bounds3<f32> {
        let init_bounds = primitives_info[start].bounds;
        primitives_info[start + 1..end]
            .iter()
            .fold(init_bounds, |b, p| b.union(&p.bounds))
    }

    /// Returns a bounding box for the centroids of the primitives in the
    /// specified range.
    ///
    /// The range of primitives specified by `start` and `end` must contain at
    /// least two primitives; this will panic otherwise.
    fn centroid_bounds(
        primitives_info: &[PrimitiveInfo],
        start: usize,
        end: usize,
    ) -> Bounds3<f32> {
        let init_bounds = Bounds3::from_corners(
            primitives_info[start].centroid,
            primitives_info[start + 1].centroid,
        );
        primitives_info[start + 2..end]
            .iter()
            .fold(init_bounds, |b, p| b.union(&p.bounds))
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

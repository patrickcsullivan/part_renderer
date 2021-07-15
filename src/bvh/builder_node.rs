use super::PrimitiveInfo;
use crate::{bounding_box::Bounds3, math::axis::Axis3, math::point, number};
use cgmath::Point3;
use typed_arena::Arena;

/// A node in a bounding volume hierarchy tree. The structure of this nodes is
/// convenient for building a bounding volume hierarchy, but it is less
/// condensed than the structure `FlatNode`. Therefore, when building a bounding
/// volume hierarchy, we first construct the tree out of `BuilderNode`s before
/// transforming the tree into `FlatNode`s.
pub enum BuilderNode<'arena> {
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
    /// Returns a leaf node.
    fn leaf(first_index: usize, num_primitives: usize, bounds: Bounds3<f32>) -> Self {
        Self::Leaf {
            bounds,
            first_index,
            num_primitives,
        }
    }

    /// Returns an interior node.
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
    /// constructed tree and the total number of nodes in the tree.
    ///
    /// All nodes are allocated in the given arena.
    ///
    /// Elements in the range [`start`, `end`) will be reordered as the tree
    /// construction algorithm partitions subsets of primitives.
    ///
    /// An index reference to each primitive in the constructed tree is added to
    /// `ordered_primitive_indices`.
    ///
    /// The range of primitives specified by `start` and `end` must contain at
    /// least one primitive; this will panic otherwise.
    fn build_subtree(
        arena: &'arena Arena<BuilderNode<'arena>>,
        primitives_info: &mut [PrimitiveInfo],
        start: usize,
        end: usize,
        ordered_primitive_indices: &mut Vec<usize>,
    ) -> (&'arena Self, usize) {
        let num_primitives = end - start;

        // If there is only one primitive, then just create a leaf containing
        // the primitive.
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

        // We will partition the primitives along the axis along which the
        // primitive centroids have the greatest range.
        let partition_axis = centroid_bounds.maximum_extend();
        let second_partition_start = if num_primitives <= 4 {
            // Using surface area heuristic to partition isn't worth the effort
            // when the subset of primitives is small enough.
            Self::even_split_partition(primitives_info, start, end, partition_axis)
        } else {
            Self::surface_area_heuristic_partition(primitives_info, start, end, partition_axis)
        };

        let (left_child, left_size) = Self::build_subtree(
            arena,
            primitives_info,
            start,
            second_partition_start,
            ordered_primitive_indices,
        );
        let (right_child, right_size) = Self::build_subtree(
            arena,
            primitives_info,
            second_partition_start,
            end,
            ordered_primitive_indices,
        );
        let bounds = Self::primitives_bounds(primitives_info, start, end);
        let parent = arena.alloc(Self::interior(
            partition_axis,
            left_child,
            right_child,
            bounds,
        ));
        (parent, left_size + right_size + 1)
    }

    /// Reorders `primitives_info` so that for each primitive in the first half
    /// of the slice the primitive's centroid position along `axis` is less than
    /// that of the centroid positions for primitives in the second half. This
    /// method returns the index of the first element in the second partition.
    ///
    /// Note that this does not necessarily order the slice by centroid position.
    fn even_split_partition(
        primitives_info: &mut [PrimitiveInfo],
        start: usize,
        end: usize,
        axis: Axis3,
    ) -> usize {
        let mid_offset = (end - start) / 2;
        let subset = &mut primitives_info[start..end];
        subset.select_nth_unstable_by(mid_offset, |p1, p2| {
            number::f32::total_cmp(
                point::component(p1.centroid, axis),
                point::component(p2.centroid, axis),
            )
        });
        start + mid_offset
    }

    /// Constructs a leaf node that contains the primitives identified by the
    /// subset of `PrimitiveInfo` structs in the index range [`start`, `end`].
    /// Returns a reference to the new leaf node and adds an index reference to
    /// each primitive in the leaf to `ordered_primitive_indices`.
    ///
    /// The new leaf node is allocated in the given arena.
    ///
    /// An index reference to each primitive in the leaf is added to
    /// `ordered_primitive_indices`.
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
    /// least one primitive; this will panic otherwise.
    fn centroid_bounds(
        primitives_info: &[PrimitiveInfo],
        start: usize,
        end: usize,
    ) -> Bounds3<f32> {
        let init_bounds = Bounds3::from_point(primitives_info[start].centroid);
        primitives_info[start + 1..end]
            .iter()
            .fold(init_bounds, |b, p| b.union(&p.bounds))
    }

    /// Reorders `primitives_info` such that elements are partitioned using a
    /// "surface area heuristic" that attempts to minimize the cost of ray
    /// intersection tests.
    fn surface_area_heuristic_partition(
        primitives_info: &mut [PrimitiveInfo],
        start: usize,
        end: usize,
        axis: Axis3,
    ) -> usize {
        todo!()
    }
}

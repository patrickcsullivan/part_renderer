use super::PrimitiveInfo;
use crate::{
    bounding_box::{Bounds3, Union},
    math::axis::Axis3,
    math::point,
    number,
};
use cgmath::Point3;
use typed_arena::Arena;

/// A node in a bounding volume hierarchy tree. The structure of this nodes is
/// convenient for building a bounding volume hierarchy, but it is not a
/// memory-efficient representation of the tree.
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

/// A bucket of primitives, used in the surface area heuristic partitioning
/// algorithm.
#[derive(Debug, Clone, Copy)]
struct Bucket {
    /// The number of primitives in the bucket.
    primitives_count: usize,

    /// The bounding box of all primitives in the bucket.
    bounds: Option<Bounds3<f32>>,
}

impl Default for Bucket {
    fn default() -> Self {
        Self {
            primitives_count: 0,
            bounds: None,
        }
    }
}

impl Bucket {
    fn add_primitive(&mut self, primitive_info: &PrimitiveInfo) {
        self.primitives_count += 1;
        self.bounds = if let Some(bounds) = self.bounds {
            Some(bounds.union(&primitive_info.bounds))
        } else {
            Some(primitive_info.bounds)
        }
    }
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
        max_primitives_per_leaf: usize,
        ordered_primitive_indices: &mut Vec<usize>,
    ) -> (&'arena Self, usize) {
        let primitives_count = end - start;

        // If there is only one primitive, then just create a leaf containing
        // the primitive.
        if primitives_count == 1 {
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

        // We will partition the primitives along the axis on which the
        // centroids have the greatest range.
        let partition_axis = centroid_bounds.maximum_extend();
        let primitives_bounds = Self::primitives_bounds(primitives_info, start, end);

        // Using surface area heuristic to partition isn't worth the effort when
        // the subset of primitives is small enough, so just evenly partition
        // the primitives along the axis.
        if primitives_count <= 4 {
            let mid = Self::even_split_partition(primitives_info, start, end, partition_axis);
            return Self::build_interior(
                arena,
                primitives_info,
                start,
                mid,
                end,
                partition_axis,
                max_primitives_per_leaf,
                primitives_bounds,
                ordered_primitive_indices,
            );
        }

        // Split the axis into buckets
        let (split_rel_pos, split_cost) = Self::find_min_sah_cost_split(
            primitives_info,
            start,
            end,
            partition_axis,
            primitives_bounds,
            centroid_bounds,
        );

        // Estimate the ray intersection test cost if we just put all primitives
        // in a leaf. If that's less than the cost associated with a partitioned
        // interior node, and if the leaf wouldn't exceed the max size, then
        // just build a leaf.
        let leaf_cost = primitives_count as f32;
        if leaf_cost < split_cost && primitives_count < max_primitives_per_leaf {
            let node = Self::build_leaf(
                arena,
                primitives_info,
                start,
                end,
                ordered_primitive_indices,
            );
            return (node, 1);
        }

        todo!();
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
        node
    }

    fn build_interior(
        arena: &'arena Arena<BuilderNode<'arena>>,
        primitives_info: &mut [PrimitiveInfo],
        start: usize,
        mid: usize,
        end: usize,
        partition_axis: Axis3,
        max_primitives_per_leaf: usize,
        primitives_bounds: Bounds3<f32>,
        ordered_primitive_indices: &mut Vec<usize>,
    ) -> (&'arena Self, usize) {
        let (left_child, left_size) = Self::build_subtree(
            arena,
            primitives_info,
            start,
            mid,
            max_primitives_per_leaf,
            ordered_primitive_indices,
        );
        let (right_child, right_size) = Self::build_subtree(
            arena,
            primitives_info,
            mid,
            end,
            max_primitives_per_leaf,
            ordered_primitive_indices,
        );
        let parent = arena.alloc(Self::interior(
            partition_axis,
            left_child,
            right_child,
            primitives_bounds,
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

    /// Reorders a subset of `primitives_info` so that primitives are
    /// partitioned by their relative positions along a given axis.
    ///
    /// This method takes a bounding box for the primitives' centroids and a
    /// `relative_partition_position` inside that bounding box along the given
    /// axis. Any primitive whose centroid relative position is less than or
    /// equal to `relative_partition_position` is moved into the first
    /// partition, and all other primitives are moved into the second
    fn partition_around_position() {}

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

    /// This method returns a relative position along the given axis of the
    /// primitives' centroid bounding box. A value of 0.0 is at the minimum end
    /// of the bounding box, and a value of 1.0 is at the maximum end of the
    /// bounding box. The relative position is the partition point that is
    /// estimated to minimize the surface area heuristic partition cost.
    ///
    /// This method also returns the relative surface area heuristic cost
    /// associated with partitioning the range of primitives at the specified
    /// point.
    ///
    /// If no valid partition can be found, this returns a cost of infinity.
    ///
    /// # Arguments
    ///
    /// * `primitives_info` - A slice of `PrimitiveInfo` structs.
    /// * `start` - Inclusive start index of the range of primitives.
    /// * `end` - Exclusive end index of the range of primitives.
    /// * `axis` - The axis along which primitives will be partitioned.
    /// * `primitives_bounds` - Pre-computed bounding box for  the primitives in
    ///   the specifed range.
    /// * `primitives_centroid_bounds` - Pre-computed bounding box for centroids
    ///   of the primitives in the specifed range.
    fn find_min_sah_cost_split(
        primitives_info: &mut [PrimitiveInfo],
        start: usize,
        end: usize,
        axis: Axis3,
        primitives_bounds: Bounds3<f32>,
        primitives_centroid_bounds: Bounds3<f32>,
    ) -> (f32, f32) {
        const BUCKET_COUNT: usize = 12;
        let buckets = Self::divide_range_into_buckets(
            primitives_info,
            start,
            end,
            axis,
            BUCKET_COUNT,
            primitives_centroid_bounds,
        );

        // `costs[i]` will contain the estimated surface area heuristic cost of
        // partitioning the primitives after the `i`th bucket. Partitioning
        // after the last bucket isn't considered since that wouldn't actually
        // split the primitives.
        let costs: Vec<f32> = (0..BUCKET_COUNT - 1)
            .map(|i| Self::estimate_sah_cost(&buckets, i, primitives_bounds))
            .collect();

        let mut split_after_bucket = 0;
        let mut min_cost = f32::INFINITY;
        for (i, &c) in costs.iter().enumerate() {
            if c < min_cost {
                split_after_bucket = i;
                min_cost = c;
            }
        }

        // Map the bucket index to a relative position.
        let split_at = (split_after_bucket + 1) as f32 / (BUCKET_COUNT + 1) as f32;

        (split_at, min_cost)
    }

    /// Divides the range of primitives into buckets along the given axis and
    /// returns the buckets.
    ///
    /// The buckets evenly divide the range of space that is occupied by the
    /// primitives' centroids along the given axis. A primitive is placed in a
    /// bucket if the primitive's centroid is in the space corresponding to the
    /// bucket.
    ///
    /// Note that the bounding boxes of different buckets can overlap.
    fn divide_range_into_buckets(
        primitives_info: &[PrimitiveInfo],
        start: usize,
        end: usize,
        axis: Axis3,
        bucket_count: usize,
        primitives_centroid_bounds: Bounds3<f32>,
    ) -> Vec<Bucket> {
        let mut buckets = vec![Bucket::default(); bucket_count];
        for p in &primitives_info[start..end] {
            let bucket_index = Self::find_bucket(p, primitives_centroid_bounds, axis, bucket_count);
            let bucket = &mut buckets[bucket_index];
            bucket.add_primitive(p);
        }
        buckets
    }

    fn relative_position_along_axis(p: Point3<f32>, bounds: Bounds3<f32>, axis: Axis3) -> f32 {
        point::component(bounds.offset(&p), axis)
    }

    /// Find the bucket that `primitive_info` belongs in when the range of space
    /// occupied by the primitives' centroid bounds is divided evently into
    /// buckets along the specified axis.
    fn find_bucket(
        primitive_info: &PrimitiveInfo,
        primitives_centroid_bounds: Bounds3<f32>,
        axis: Axis3,
        bucket_count: usize,
    ) -> usize {
        // Compute the relative position along the axis of the primitive
        // centroid, compared to the other primitive centroids. This will be
        // 0.0 at the lower end of the range and 1.0 at the upper end.
        let rel_pos = point::component(
            primitives_centroid_bounds.offset(&primitive_info.centroid),
            axis,
        );

        // Map the relative position into an index for one of the buckets.
        ((rel_pos * bucket_count as f32) as usize).min(bucket_count - 1)
    }

    /// Estimate the surface area heuristic cost of partitioning primitives by
    /// splitting them after the bucket at the index, `split_after`.
    fn estimate_sah_cost(
        buckets: &[Bucket],
        split_after: usize,
        primitives_bounds: Bounds3<f32>,
    ) -> f32 {
        // Split buckets into two partitions, `p1` and `p2`.
        let p1_buckets = &buckets[..=split_after];
        let p2_buckets = &buckets[split_after + 1..];

        let p1_primitives_count = p1_buckets
            .iter()
            .fold(0, |n, bucket| n + bucket.primitives_count);
        let p1_bounds = p1_buckets
            .iter()
            .fold(None, |bounds, bucket| bounds.union(&bucket.bounds));
        let p1_bounds_sa = p1_bounds.map_or(0.0, |b| b.surface_area());

        let p2_primitives_count = p2_buckets
            .iter()
            .fold(0, |n, bucket| n + bucket.primitives_count);
        let p2_bounds = p2_buckets
            .iter()
            .fold(None, |bounds, bucket| bounds.union(&bucket.bounds));
        let p2_bounds_sa = p2_bounds.map_or(0.0, |b| b.surface_area());

        // This follows the surface area heuristic cost function from p. 264 of
        // PBR ed. 3. We estimate that the cost of performming a node traversal
        // is 1/8 the cost of computing a ray intersection for an individual
        // primitive.
        const TRAVERSAL_RELATIVE_COST: f32 = 0.125;
        TRAVERSAL_RELATIVE_COST
            + (p1_primitives_count as f32 * p1_bounds_sa
                + p2_primitives_count as f32 * p2_bounds_sa)
                / primitives_bounds.surface_area()
    }
}

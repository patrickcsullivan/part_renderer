use std::ops::Add;

use crate::math::axis;
use crate::ray;
use crate::transform;
use cgmath::Point3;
use cgmath::Transform;
use cgmath::VectorSpace;

/// A 2D axis-aligned bounding box.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Bounds2<S> {
    min: cgmath::Point2<S>,
    max: cgmath::Point2<S>,
}

/// A 3D axis-aligned bounding box.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Bounds3<S> {
    min: cgmath::Point3<S>,
    max: cgmath::Point3<S>,
}

impl<S: cgmath::BaseNum + std::cmp::PartialOrd + std::fmt::Display> Bounds3<S> {
    pub fn min(&self) -> Point3<S> {
        self.min
    }

    pub fn max(&self) -> Point3<S> {
        self.max
    }

    /// Creates a bounding box that encloses a single point.
    pub fn from_point(p: cgmath::Point3<S>) -> Self {
        Self { min: p, max: p }
    }

    /// Creates a bounding box that encloses the given corner points.
    pub fn from_corners(p1: cgmath::Point3<S>, p2: cgmath::Point3<S>) -> Self {
        let min = cgmath::Point3::new(
            min_partial_ord(p1.x, p2.x),
            min_partial_ord(p1.y, p2.y),
            min_partial_ord(p1.z, p2.z),
        );
        let max = cgmath::Point3::new(
            max_partial_ord(p1.x, p2.x),
            max_partial_ord(p1.y, p2.y),
            max_partial_ord(p1.z, p2.z),
        );
        Self { min, max }
    }

    /// Returns the corner points of the bounding box.
    pub fn corners(&self) -> Vec<cgmath::Point3<S>> {
        vec![
            cgmath::Point3::new(self.min.x, self.min.y, self.min.z),
            cgmath::Point3::new(self.max.x, self.min.y, self.min.z),
            cgmath::Point3::new(self.min.x, self.max.y, self.min.z),
            cgmath::Point3::new(self.max.x, self.max.y, self.min.z),
            cgmath::Point3::new(self.min.x, self.min.y, self.max.z),
            cgmath::Point3::new(self.max.x, self.min.y, self.max.z),
            cgmath::Point3::new(self.min.x, self.max.y, self.max.z),
            cgmath::Point3::new(self.max.x, self.max.y, self.max.z),
        ]
    }

    /// Returns a union of two bounding boxes.
    pub fn union(&self, other: &Self) -> Self {
        let min = cgmath::Point3::new(
            min_partial_ord(self.min.x, other.min.x),
            min_partial_ord(self.min.y, other.min.y),
            min_partial_ord(self.min.z, other.min.z),
        );
        let max = cgmath::Point3::new(
            max_partial_ord(self.max.x, other.max.x),
            max_partial_ord(self.max.y, other.max.y),
            max_partial_ord(self.max.z, other.max.z),
        );
        Self { min, max }
    }

    /// Returns a union of the bounding box and an additional point..
    pub fn union_with_point(&self, p: &cgmath::Point3<S>) -> Self {
        self.union(&Bounds3::from_point(*p))
    }

    /// Returns the intersection of the bounding boxes.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let min = cgmath::Point3::new(
            max_partial_ord(self.min.x, other.min.x),
            max_partial_ord(self.min.y, other.min.y),
            max_partial_ord(self.min.z, other.min.z),
        );
        let max = cgmath::Point3::new(
            min_partial_ord(self.max.x, other.max.x),
            min_partial_ord(self.max.y, other.max.y),
            min_partial_ord(self.max.z, other.max.z),
        );
        if leq_3d(&min, &max) {
            Some(Self { min, max })
        } else {
            None
        }
    }

    /// Returns true if and only if the bounding boxes overlap inclusively.
    pub fn overlaps(&self, other: &Self) -> bool {
        let x_overlaps = self.min.x <= other.max.x && self.max.x >= other.min.x;
        let y_overlaps = self.min.y <= other.max.y && self.max.y >= other.min.y;
        let z_overlaps = self.min.z <= other.max.z && self.max.z >= other.min.z;
        x_overlaps && y_overlaps && z_overlaps
    }

    /// Returns true if and only if the point is inside the bounding box
    /// inclusively.
    pub fn inside(&self, p: &cgmath::Point3<S>) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    /// Returns true if and only if the point is inside the bounding box
    /// exclusively.
    pub fn inside_exclusive(&self, p: &cgmath::Point3<S>) -> bool {
        p.x >= self.min.x
            && p.x < self.max.x
            && p.y > self.min.y
            && p.y < self.max.y
            && p.z > self.min.z
            && p.z < self.max.z
    }

    /// Expands the bounding box by the given `delta`. If `delta` is zero or
    /// negative the bounding box is unchanged.
    pub fn expand(&mut self, delta: S) {
        if delta > S::zero() {
            let min = self.min - cgmath::Vector3::new(delta, delta, delta);
            let max = self.max + cgmath::Vector3::new(delta, delta, delta);
            self.min = min;
            self.max = max;
        }
    }

    /// Returns a vector across the diagonal of the bounding box, pointing from
    /// the miminum corner to the maximum corner.
    pub fn diagonal(&self) -> cgmath::Vector3<S> {
        self.max - self.min
    }

    /// Returns the surface area of the bounding box.
    pub fn surface_area(&self) -> S {
        let d = self.diagonal();
        let half = d.x * d.y + d.x * d.z + d.y * d.z;
        half + half
    }

    /// Returns the volume of the bounding box.
    pub fn volume(&self) -> S {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    /// Returns the longest axis of the bounding box.
    pub fn maximum_extend(&self) -> axis::Axis3 {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            axis::Axis3::X
        } else if d.y > d.z {
            axis::Axis3::Y
        } else {
            axis::Axis3::Z
        }
    }

    /// Returns the result of linearly interpolating from the bounding box's
    /// minimum corner to its maximum corner.
    pub fn lerp(&self, amount: S) -> cgmath::Point3<S> {
        let origin = cgmath::Point3::new(S::zero(), S::zero(), S::zero());
        let min = self.min - origin;
        let max = self.max - origin;
        let lerp = min.lerp(max, amount);
        cgmath::Point3::new(lerp.x, lerp.y, lerp.z)
    }

    // TODO: bounding_sphere, p. 81
}

pub trait Union<Rhs = Self> {
    type Output;

    fn union(self, other: Rhs) -> Self::Output;
}

impl<S: cgmath::BaseNum + std::cmp::PartialOrd + std::fmt::Display> Union<&Bounds3<S>>
    for &Bounds3<S>
{
    type Output = Bounds3<S>;

    fn union(self, other: &Bounds3<S>) -> Self::Output {
        let min = cgmath::Point3::new(
            min_partial_ord(self.min.x, other.min.x),
            min_partial_ord(self.min.y, other.min.y),
            min_partial_ord(self.min.z, other.min.z),
        );
        let max = cgmath::Point3::new(
            max_partial_ord(self.max.x, other.max.x),
            max_partial_ord(self.max.y, other.max.y),
            max_partial_ord(self.max.z, other.max.z),
        );
        Self::Output { min, max }
    }
}

impl<S: cgmath::BaseNum + std::cmp::PartialOrd + std::fmt::Display> Union<&Option<Bounds3<S>>>
    for &Option<Bounds3<S>>
{
    type Output = Option<Bounds3<S>>;

    fn union(self, other: &Option<Bounds3<S>>) -> Self::Output {
        match (self, other) {
            (None, None) => None,
            (None, Some(b)) => Some(*b),
            (Some(b), None) => Some(*b),
            (Some(b1), Some(b2)) => Some(b1.union(b2)),
        }
    }
}

impl Bounds3<f32> {
    pub fn cetroid(&self) -> Point3<f32> {
        self.min + self.diagonal() * 0.5
    }

    /// Returns the offset factor of the given point relative to the corners of
    /// the bounding box. If `p` is at the minimum corner this returns `(0.0,
    /// 0.0, 0.0)`. If `p` is at the maximum corner this returns `(1.0, 1.0,
    /// 1.0)`.
    pub fn offset(&self, p: &Point3<f32>) -> Point3<f32> {
        let min_to_p = p - self.min;
        let min_to_max = self.max - self.min;
        Point3::new(
            min_to_p.x / min_to_max.x,
            min_to_p.y / min_to_max.y,
            min_to_p.z / min_to_max.z,
        )
    }

    /// Returns the range of parametric values for which the given ray passes
    /// through the bounding box. If the ray's origin is inside the bounding box
    /// then the lower bound will be 0.
    pub fn ray_intersection(&self, ray: &ray::Ray) -> Option<(f32, f32)> {
        // TODO: Consider implementing the optimized version of this function
        // that takes pre-computed values described on p. 128.

        let mut result = (0.0, ray.t_max);

        for dim in 0..3 {
            let ray_direction_recip = 1.0 / ray.direction[dim];

            // Handle special cases where the ray direction is parallel to the
            // min and max planes of the bounding box.
            if ray_direction_recip.is_infinite() {
                if ray.origin[dim] < self.min[dim] || ray.origin[dim] > self.max[dim] {
                    // The ray origin does not lie between the planes, so the
                    // ray can never intersect the bounding box.
                    return None;
                } else {
                    // The ray origin is between the planes or on one of the
                    // planes, so this dimension does not shrink the bounds of
                    // the result.
                    continue;
                }
            }

            // Find the parametric value where the ray intersects each side of
            // the bounding box in the current dimension.
            let t_at_bounds_min = (self.min[dim] - ray.origin[dim]) * ray_direction_recip;
            let t_at_bounds_max = (self.max[dim] - ray.origin[dim]) * ray_direction_recip;

            // Assuming both sides of the bounding box are intersected,
            // determine which intersection is nearest to the ray origin and
            // which is furthest.
            let (t_near, t_far) = if t_at_bounds_min > t_at_bounds_max {
                (t_at_bounds_max, t_at_bounds_min)
            } else {
                (t_at_bounds_min, t_at_bounds_max)
            };

            // Shrink the bounds in the result .
            if t_near > result.0 {
                result.0 = t_near;
            }
            if t_far < result.1 {
                result.1 = t_far;
            }
            if result.0 > result.1 {
                return None;
            }
        }

        Some(result)
    }
}

impl transform::Transform<Bounds3<f32>> for cgmath::Matrix4<f32> {
    fn transform(&self, bounds: &Bounds3<f32>) -> Bounds3<f32> {
        // TODO: This could be optimized.
        let init = Bounds3::from_point(self.transform_point(bounds.min));
        bounds
            .corners()
            .into_iter()
            .map(|p| self.transform_point(p))
            .fold(init, |b, p| b.union_with_point(&p))
    }
}

/// Returns true if all dimensions of the first point are less than or equal
/// to the respective dimensions of the second point.
fn leq_3d<S: std::cmp::PartialOrd>(p1: &cgmath::Point3<S>, p2: &cgmath::Point3<S>) -> bool {
    p1.x <= p2.x && p1.y <= p2.y && p1.z <= p2.z
}

fn min_partial_ord<S: std::cmp::PartialOrd>(x: S, y: S) -> S {
    if x.le(&y) {
        x
    } else if y.lt(&x) {
        y
    } else {
        // FIXME: This isn't a good way of handling an error.
        panic!("Could not find a minimumn between the given values.");
    }
}

fn max_partial_ord<S: std::cmp::PartialOrd>(x: S, y: S) -> S {
    if x.ge(&y) {
        x
    } else if y.gt(&x) {
        y
    } else {
        // FIXME: This isn't a good way of handling an error.
        panic!("Could not find a maximum between the given values.");
    }
}

use super::SurfaceInteraction;
use crate::ray::Ray;
use cgmath::{point3, InnerSpace, Matrix4, Point3, Transform, Vector3};

const SHADOW_EPSILON: f32 = 0.0001;

/// Describes an interaction point at which a new ray can be spawned. If the new
/// ray is spawned on an intersectable surface, the ray origin is position in
/// such a way that it does not immediately intersect with the surface on which
/// it originates.
pub trait OffsetRayOrigin {
    /// The non-offset ray origin. This is the original interaction point.
    fn non_offset_ray_origin(&self) -> Point3<f32>;

    /// Offset the origin of a new ray at this point, accounting for potential
    /// floating point error in its calculation, so that the ray does not
    /// intersect with the surface on which it originates.
    fn offset_ray_origin(&self, ray_direction: &Vector3<f32>) -> Point3<f32>;

    /// Spawn a new ray leaving the interaction point in the given direction.
    fn spawn_ray(&self, ray_direction: &Vector3<f32>) -> Ray {
        let origin = self.offset_ray_origin(ray_direction);
        Ray::new(origin, *ray_direction, f32::INFINITY)
    }

    fn spawn_shadow_ray_to_point(&self, target: &Point3<f32>) -> Ray {
        let origin = self.offset_ray_origin(&(target - self.non_offset_ray_origin()));
        let direction = target - origin;
        Ray::new(origin, direction, 1.0 - SHADOW_EPSILON)
    }

    fn spawn_shadow_ray_to_offset_point(&self, target: Box<dyn OffsetRayOrigin>) -> Ray {
        let origin = self
            .offset_ray_origin(&(target.non_offset_ray_origin() - self.non_offset_ray_origin()));
        let new_target = target.offset_ray_origin(&(origin - target.non_offset_ray_origin()));
        let direction = new_target - origin;
        Ray::new(origin, direction, 1.0 - SHADOW_EPSILON)
    }
}

impl OffsetRayOrigin for SurfaceInteraction {
    fn non_offset_ray_origin(&self) -> Point3<f32> {
        self.point
    }

    fn offset_ray_origin(&self, ray_direction: &Vector3<f32>) -> Point3<f32> {
        offset_ray_origin(
            &self.point,
            &self.point_error_bound,
            &self.original_geometry.normal,
            ray_direction,
        )
    }
}

/// Offset the origin of a new ray at a surface interaction, accounting for
/// potential floating point error in its calculation, so that the ray does not
/// intersect with the surface on which it originates.
fn offset_ray_origin(
    interaction_point: &Point3<f32>,
    interaction_point_error_bound: &Vector3<f32>,
    normal: &Vector3<f32>,
    ray_direction: &Vector3<f32>,
) -> Point3<f32> {
    let offset_along_normal = normal
        .map(|comp| comp.abs())
        .dot(*interaction_point_error_bound);
    let offset = if ray_direction.dot(*normal) < 0.0 {
        // The ray is going out of the surface, so offset along normal.
        offset_along_normal * normal
    } else {
        // The ray is going into the surface, so offset against normal.
        -1.0 * offset_along_normal * normal
    };
    let offset_point = interaction_point + offset;

    // Round the offset point away from the original interaction point.
    point3(
        round_along_offset(offset_point.x, offset.x),
        round_along_offset(offset_point.y, offset.y),
        round_along_offset(offset_point.z, offset.z),
    )
}

/// Round the given number up if the offset is positive and down of the offset
/// is negative. Return the given number, unchanged, if the offset is 0.
fn round_along_offset(f: f32, offset: f32) -> f32 {
    use float_next_after::NextAfter;
    if offset > 0.0 {
        f.next_after(f32::INFINITY)
    } else if offset < 0.0 {
        f.next_after(f32::NEG_INFINITY)
    } else {
        f
    }
}

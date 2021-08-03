use crate::{color::RgbSpectrum, geometry::vector, ray::Ray};
use cgmath::{Point3, Vector3};
use typed_arena::Arena;

#[derive(Debug, Clone, Copy)]
pub struct SurfaceInteraction {
    /// The point in world space where the interaction with a surface occurs.
    pub point: Point3<f32>,

    /// The direction of the negative/outgoing ray.
    pub neg_ray_direction: Vector3<f32>,

    /// The original geometry of the surface at the intersection point.
    pub original_geometry: SurfaceGeometry,

    /// A second instance of the surface geometry. These properties are
    /// initialized to match the original surface geometry, but they may be
    /// perturbed (by bump mapping, for example) before they are used in shading
    /// calculations by the integrator.
    pub shading_geometry: SurfaceGeometry,
}

/// Represents the geometry at a specific point on a surface. Includes a normal,
/// partial derivatives of the normal with respect to UV coordinates, and
/// partial derivatives of the XYZ position with respect to UV coordinates.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceGeometry {
    pub normal: Vector3<f32>,

    /// The partial derivative of the position with respect to U.
    pub dpdu: Vector3<f32>,

    /// The partial derivative of the position with respect to V.
    pub dpdv: Vector3<f32>,
}

impl SurfaceInteraction {
    pub fn new(
        point: Point3<f32>,
        neg_ray_direction: Vector3<f32>,
        normal: Vector3<f32>,
        dpdu: Vector3<f32>,
        dpdv: Vector3<f32>,
    ) -> Self {
        Self {
            point,
            neg_ray_direction,
            original_geometry: SurfaceGeometry { normal, dpdu, dpdv },
            shading_geometry: SurfaceGeometry { normal, dpdu, dpdv },
        }
    }

    // pub fn over_point(&self) -> Point3<f32> {
    //     self.point + self.original_geometry.normal * 0.01 // FIXME: This adjustment value seems very high.
    // }

    // pub fn under_point(&self) -> Point3<f32> {
    //     self.point - self.original_geometry.normal * 0.01 // FIXME: This adjustment value seems very high.
    // }

    // pub fn reflect(&self) -> Vector3<f32> {
    //     vector::reflect(-1.0 * self.neg_ray_direction, self.original_geometry.normal)
    // }
}

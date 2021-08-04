use cgmath::{InnerSpace, Matrix4, Point3, Transform, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct SurfaceInteraction {
    /// The point in world space where the interaction with a surface occurs.
    pub point: Point3<f32>,

    /// A conservative bound on the amount of floating point error in `point`.
    pub point_error_bound: Vector3<f32>,

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
        point_error_bound: Vector3<f32>,
        neg_ray_direction: Vector3<f32>,
        dpdu: Vector3<f32>,
        dpdv: Vector3<f32>,
    ) -> Self {
        let normal = dpdu.cross(dpdv);
        Self {
            point,
            point_error_bound,
            neg_ray_direction,
            original_geometry: SurfaceGeometry { normal, dpdu, dpdv },
            shading_geometry: SurfaceGeometry { normal, dpdu, dpdv },
        }
    }

    pub fn new_with_normal(
        point: Point3<f32>,
        point_error_bound: Vector3<f32>,
        neg_ray_direction: Vector3<f32>,
        dpdu: Vector3<f32>,
        dpdv: Vector3<f32>,
        normal: Vector3<f32>,
    ) -> Self {
        Self {
            point,
            point_error_bound,
            neg_ray_direction,
            original_geometry: SurfaceGeometry { normal, dpdu, dpdv },
            shading_geometry: SurfaceGeometry { normal, dpdu, dpdv },
        }
    }
}

// TODO: When interaction transformation is needed, account for error introduced by transformation.
// impl crate::geometry::Transform<SurfaceInteraction> for Matrix4<f32> {
//     fn transform(&self, t: &SurfaceInteraction) -> SurfaceInteraction {
//         SurfaceInteraction {
//             point: self.transform_point(t.point),
//             neg_ray_direction: self.transform_vector(t.neg_ray_direction).normalize(),
//             original_geometry: self.transform(&t.original_geometry),
//             shading_geometry: self.transform(&t.shading_geometry),
//         }
//     }
// }

// impl crate::geometry::Transform<SurfaceGeometry> for Matrix4<f32> {
//     fn transform(&self, t: &SurfaceGeometry) -> SurfaceGeometry {
//         SurfaceGeometry {
//             normal: self.transform_vector(t.normal).normalize(),
//             dpdu: self.transform_vector(t.dpdu),
//             dpdv: self.transform_vector(t.dpdv),
//         }
//     }
// }

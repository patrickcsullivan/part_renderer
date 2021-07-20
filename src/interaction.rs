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
    pub normal: cgmath::Vector3<f32>,
}

impl SurfaceInteraction {
    pub fn new(point: Point3<f32>, neg_ray_direction: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self {
            point,
            neg_ray_direction,
            original_geometry: SurfaceGeometry { normal },
            shading_geometry: SurfaceGeometry { normal },
        }
    }

    /// Compute the bidrectional scattering distribution function at the point
    /// on the surface. This method determines how much of the incoming light
    /// from some source is scattered in the direction of the destination.
    ///
    /// Note that both `point_to_source_direction` and
    /// `point_to_destination_direction` should be directed outward from the
    /// point on the surface. Also, both vectors should be normalized.
    pub fn bsdf(
        &self,
        point_to_source_direction: &Vector3<f32>,
        point_to_destinatino_direction: &Vector3<f32>,
    ) -> RgbSpectrum {
        todo!()
    }

    /// Compute scattering functions for the surface interaction.
    /// TODO: What does this mean?
    pub fn compute_scattering_functions(&self, ray: &Ray, spectrum_arena: &mut Arena<RgbSpectrum>) {
        let mut radiance = RgbSpectrum::constant(0.0);

        // Add emitted light if the surface is emissive.
        radiance += self.emitted_radiance(&self.neg_ray_direction);
    }

    /// Determine the radiance emitted by the surface in the given direction if
    /// the surface is emissive (an area light source, for example). Returns no
    /// radiance if the surface is not emissive.
    pub fn emitted_radiance(&self, direction: &Vector3<f32>) -> RgbSpectrum {
        // TODO: Compute radiance for emissive surface.
        // TODO: This might have to move or incorporate a primitive parameter.
        RgbSpectrum::constant(0.0)
    }

    pub fn over_point(&self) -> Point3<f32> {
        self.point + self.original_geometry.normal * 0.01 // FIXME: This adjustment value seems very high.
    }

    pub fn under_point(&self) -> Point3<f32> {
        self.point - self.original_geometry.normal * 0.01 // FIXME: This adjustment value seems very high.
    }

    pub fn reflect(&self) -> Vector3<f32> {
        vector::reflect(-1.0 * self.neg_ray_direction, self.original_geometry.normal)
    }
}

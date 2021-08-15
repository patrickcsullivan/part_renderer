mod bxdf;
mod fresnel;
mod geometry;
mod lambertian;
mod oren_nayar;
mod scale;

pub use bxdf::{Bxdf, BxdfType};
pub use lambertian::{LambertianDiffuseReflection, LambertianDiffuseTransmission};
pub use oren_nayar::OrenNayarDiffuseReflection;

use crate::{color::RgbSpectrum, interaction::SurfaceInteraction};
use cgmath::{vec3, InnerSpace, Point2, Vector3};

/// The bidirectional scattering distribution function (BSDF). Describes the way
/// light scatters at a point on a surface. A BSDF is composed of multiple
/// different bidirectional reflectance distribution functions and bidirectional
/// transmission distribution functions.
///
/// BSDFs only model the scattering of light that enters and exits a surface at
/// a single point. To model the scattering of light that occurs as light passes
/// through a material (rather than just modeling scattering that occurs at a
/// surface interaction) a bidirectional scattering-surface reflectance
/// distribution function (BSSRDF) should be used instead.
pub struct Bsdf {
    /// The relative index of refraction over the boundry of the surface. This
    /// should be set to 1 for opaque surfaces.
    // pub relative_refraction: f32,
    bxdfs: Vec<Box<dyn Bxdf>>,

    /// The original surface normal.
    original_normal: Vector3<f32>,

    /// The surface normal after any perturbations (by bump mapping, for
    /// example). This is used as the z axis of the local shading coordinate
    /// system.
    shading_normal: Vector3<f32>,

    /// The primary surface tangent vector after any perturbations (by bump
    /// mapping, for example). This is used as the x axis of the local shading
    /// coordinate system.
    shading_primary_tangent: Vector3<f32>,

    /// The primary surface tangent vector after any perturbations (by bump
    /// mapping, for example). This is orthogonal to `shading_normal` and
    /// `shading_primary_tangent` and is used as the y axis of the local shading
    /// coordinate system.
    shading_secondary_tangent: Vector3<f32>,
}

impl Bsdf {
    /// Construct a BSDF that describes the way light scatters at point on a
    /// surface.
    ///
    /// * interaction - Contains information about the differential geometry at
    ///   the point on the surface.
    /// * relative_refraction - The relative index of refraction over the
    ///   boundry of the surface. This should be set to 1 for opaque surfaces.
    pub fn new(
        interaction: &SurfaceInteraction,
        // relative_refraction: f32
    ) -> Self {
        Self {
            // relative_refraction,
            bxdfs: vec![],
            original_normal: interaction.original_geometry.normal,
            shading_normal: interaction.shading_geometry.normal,
            shading_primary_tangent: interaction.shading_geometry.dpdu,
            shading_secondary_tangent: interaction
                .shading_geometry
                .normal
                .cross(interaction.shading_geometry.dpdu),
        }
    }

    /// Add an element to the BSDF's collection of BxDFs.
    pub fn add(&mut self, bxdf: Box<dyn Bxdf>) {
        self.bxdfs.push(bxdf)
    }

    /// Return the number of elements in the BSDF's collection of BxDFs that
    /// have the given BxDF type.
    pub fn count_with_type(&self, ty: BxdfType) -> usize {
        self.bxdfs.iter().filter(|bxdf| bxdf.has_type(ty)).count()
    }

    /// Transform the vector from world space to the local shading space.
    pub fn transform_world_to_local(&self, v: &Vector3<f32>) -> Vector3<f32> {
        vec3(
            v.dot(self.shading_primary_tangent),
            v.dot(self.shading_secondary_tangent),
            v.dot(self.shading_normal),
        )
    }

    /// Transform the vector from the local shading space to world space.
    pub fn transform_local_to_world(&self, v: &Vector3<f32>) -> Vector3<f32> {
        vec3(
            self.shading_primary_tangent.x * v.x
                + self.shading_secondary_tangent.x * v.y
                + self.shading_normal.x * v.z,
            self.shading_primary_tangent.y * v.x
                + self.shading_secondary_tangent.y * v.y
                + self.shading_normal.y * v.z,
            self.shading_primary_tangent.z * v.x
                + self.shading_secondary_tangent.z * v.y
                + self.shading_normal.z * v.z,
        )
    }

    /// Calculate the spectrum of light that is scattered in the viewing
    /// direction, `wo_world`, due to light arriving at some point on a surface
    /// from the incident light direction, `wi_world`.
    pub fn f(
        &self,
        wo_world: &Vector3<f32>,
        wi_world: &Vector3<f32>,
        flags: BxdfType,
    ) -> RgbSpectrum {
        let wo = self.transform_world_to_local(wo_world);
        let wi = self.transform_world_to_local(wi_world);

        let reflect_or_transmit =
            if wi_world.dot(self.original_normal) * wo_world.dot(self.original_normal) > 0.0 {
                BxdfType::REFLECTION
            } else {
                BxdfType::TRANSMISSION
            };

        let type_to_eval = flags | reflect_or_transmit;

        self.bxdfs
            .iter()
            .filter(|bxdf| bxdf.has_type(type_to_eval))
            .fold(RgbSpectrum::black(), |light, bxdf| light + bxdf.f(&wo, &wi))
    }

    /// Evaluate the hemispherical-directional reflectance function. This
    /// returns the total reflection in the direction `wo` due to constant
    /// illumination over the hemisphere.
    fn rho_hd(&self, wo: &Vector3<f32>, samples: &[Point2<f32>], flags: BxdfType) -> RgbSpectrum {
        self.bxdfs
            .iter()
            .filter(|bxdf| bxdf.has_type(flags))
            .fold(RgbSpectrum::black(), |light, bxdf| {
                light + bxdf.rho_hd(wo, samples)
            })
    }

    /// Evaluate the hemispherical-hemispherical reflectance function. This
    /// returns the fraction of incident light reflected by a surface when
    /// incident light is the same from all directions.
    fn rho_hh(
        &self,
        samples1: &[Point2<f32>],
        samples2: &[Point2<f32>],
        flags: BxdfType,
    ) -> RgbSpectrum {
        self.bxdfs
            .iter()
            .filter(|bxdf| bxdf.has_type(flags))
            .fold(RgbSpectrum::black(), |light, bxdf| {
                light + bxdf.rho_hh(samples1, samples2)
            })
    }
}

mod fresnel;
mod geometry;
mod lambertian;
mod scale;

use bitflags::bitflags;
use cgmath::{vec3, InnerSpace, Point2, Vector3};

use crate::{color::RgbSpectrum, interaction::SurfaceInteraction};

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
    pub relative_refraction: f32,

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
    /// Construct a BSDF that describes the way light scatters a point on a
    /// surface.
    ///
    /// * interaction - Contains information about the differential geometry at
    ///   the point on the surface.
    /// * relative_refraction - The relative index of refraction over the
    ///   boundry of the surface. This should be set to 1 for opaque surfaces.
    pub fn new(interaction: SurfaceInteraction, relative_refraction: f32) -> Self {
        Self {
            relative_refraction,
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
                BxdfType::BSDF_REFLECTION
            } else {
                BxdfType::BSDF_TRANSMISSION
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

bitflags! {
    /// A bit flag representing the different types of bidirectional relectance
    /// or transmittance distribution functions.
    pub struct BxdfType: u8 {
        const BSDF_REFLECTION = 0b00000001;
        const BSDF_TRANSMISSION = 0b00000010;
        const BSDF_DIFFUSE = 0b00000100;
        const BSDF_GLOSSY = 0b00001000;
        const BSDF_SPECULAR = 0b00010000;
        const BSDF_ALL =
            Self::BSDF_DIFFUSE.bits |
            Self::BSDF_GLOSSY.bits |
            Self::BSDF_SPECULAR.bits |
            Self::BSDF_REFLECTION.bits |
            Self::BSDF_TRANSMISSION.bits;
    }
}

/// A BxDF is a bidriectional reflectance distribution function (BRDF), a
/// bidirectional transmittance distribution function (BTDF), or some
/// combination of the two. It can be evaluated to calculate the spectrum of
/// light scattered in a given viewing direction due to light arriving at a
/// surface from a particular incident light direction.
pub trait Bxdf {
    fn bxdf_type(&self) -> BxdfType;

    fn has_type(&self, t: BxdfType) -> bool {
        self.bxdf_type() & t == self.bxdf_type()
    }

    /// Calculate the spectrum of light that is scattered in the viewing
    /// direction, `wo`, due to light arriving at some point on a surface from
    /// the incident light direction, `wi`.
    ///
    /// This method is useful for evaluating BxDFs that scatter light over a
    /// range of directions. BxDFs that scatter light in only a single
    /// direction, such as perfectly specular BxDFs, are better evaluated with
    /// `sample_f`, since it will be practically impossible to call `f` with
    /// `wo` and `wi` arguments that result in non-zero light scattering.
    ///
    /// * wo - The view direction. A normalized vector in the shading coordinate
    ///   system that points from the point on the surface to the point from
    ///   which the surface is being viewed.
    /// * wi - The incident light direction. A normalized vector in the shading
    ///   coordinate system that points from the point on the surface to the
    ///   light source.
    fn f(&self, wo: &Vector3<f32>, wi: &Vector3<f32>) -> RgbSpectrum;

    /// Given a viewing direction, `wo`, this method returns the following:
    ///
    /// * The incident light direction that would scatter light in the viewing
    ///   direction.
    /// * PDF ?
    /// * The spetrum of light that is scattered in the viewing direction due to
    ///   light arriving at the surface from the returned incident light
    ///   direction.
    ///
    /// This method is useful for evaluating BxDFs that scatter light in only a
    /// single direction, such as perfectly specular BxDFs.
    ///
    /// * wo - The view direction. A normalized vector in the shading coordinate
    ///   system that points from the point on the surface to the point from
    ///   which the surface is being viewed.
    fn sample_f(
        &self,
        wo: &Vector3<f32>,
        sample: Point2<f32>,
        sampled_type: BxdfType,
    ) -> (Vector3<f32>, f32, RgbSpectrum);

    /// Evaluate the hemispherical-directional reflectance function. This
    /// returns the total reflection in the direction `wo` due to constant
    /// illumination over the hemisphere.
    fn rho_hd(&self, wo: &Vector3<f32>, samples: &[Point2<f32>]) -> RgbSpectrum {
        // TODO: There should actually be a default implementation when I get to
        // Monte Carlo.
        todo!()
    }

    /// Evaluate the hemispherical-hemispherical reflectance function. This
    /// returns the fraction of incident light reflected by a surface when
    /// incident light is the same from all directions.
    fn rho_hh(&self, samples1: &[Point2<f32>], samples2: &[Point2<f32>]) -> RgbSpectrum {
        // TODO: There should actually be a default implementation when I get to
        // Monte Carlo.
        todo!()
    }
}

mod geometry;

use bitflags::bitflags;
use cgmath::Vector3;

use crate::color::RgbSpectrum;

/// The bidirectional scattering distribution function (BSDF). Describes the way
/// light scatters at a point on a surface. A BSDF is composed of multiple
/// different bidirectional reflectance distribution functions and bidirectional
/// transmission distribution functions.
pub struct Bsdf {}

bitflags! {
    /// A bit flag representing the types of
    pub struct BsdfType: u8 {
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

/// A bidriectional distribution function (BDF) that can be evaluated to
/// calculate the spectrum of light that is scattered in a given viewing
/// direction due to light arriving at a surface from a given incident light
/// direction.
pub trait Bdf {
    fn bsdf_type(&self) -> BsdfType;

    fn has_type(&self, t: BsdfType) -> bool {
        self.bsdf_type() & t == self.bsdf_type()
    }

    /// Calculate the spectrum of light that is scattered in the viewing
    /// direction, `wo`, due to light arriving at some point on a surface from
    /// the incident light direction, `wi`.
    ///
    /// This method is useful for evaluating BDFs that scatter light over a
    /// range of directions. BDFs that scatter light in only a single direction,
    /// such as perfectly specular BDFs, are better evaluated with `sample_f`,
    /// since it will be practically impossible to call `f` with `wo` and `wi`
    /// arguments that result in non-zero light scattering.
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
    /// * The spetrum of light that is scattered in the viewing direction due to
    ///   light arriving at the surface from the returned incident light
    ///   direction.
    ///
    /// This method is useful for evaluating BDFs that scatter light in only a
    /// single direction, such as perfectly specular BDFs.
    ///
    /// * wo - The view direction. A normalized vector in the shading coordinate
    ///   system that points from the point on the surface to the point from
    ///   which the surface is being viewed.
    fn sample_f(&self, wo: &Vector3<f32>) -> (Vector3<f32>, RgbSpectrum);
}

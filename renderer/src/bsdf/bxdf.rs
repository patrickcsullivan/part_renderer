use crate::color::RgbaSpectrum;
use bitflags::bitflags;
use cgmath::{Point2, Vector3};

bitflags! {
    /// A bit flag representing the different types of bidirectional relectance
    /// or transmittance distribution functions.
    pub struct BxdfType: u8 {
        const REFLECTION = 0b00000001;
        const TRANSMISSION = 0b00000010;
        const DIFFUSE = 0b00000100;
        const GLOSSY = 0b00001000;
        const SPECULAR = 0b00010000;
        const ALL =
            Self::DIFFUSE.bits |
            Self::GLOSSY.bits |
            Self::SPECULAR.bits |
            Self::REFLECTION.bits |
            Self::TRANSMISSION.bits;
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
    fn f(&self, wo: &Vector3<f32>, wi: &Vector3<f32>) -> RgbaSpectrum;

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
    ) -> (Vector3<f32>, f32, RgbaSpectrum) {
        // TODO: There should actually be a default implementation.
        todo!()
    }

    /// Evaluate the hemispherical-directional reflectance function. This
    /// returns the total reflection in the direction `wo` due to constant
    /// illumination over the hemisphere.
    fn rho_hd(&self, wo: &Vector3<f32>, samples: &[Point2<f32>]) -> RgbaSpectrum {
        // TODO: There should actually be a default implementation when I get to
        // Monte Carlo.
        todo!()
    }

    /// Evaluate the hemispherical-hemispherical reflectance function. This
    /// returns the fraction of incident light reflected by a surface when
    /// incident light is the same from all directions.
    fn rho_hh(&self, samples1: &[Point2<f32>], samples2: &[Point2<f32>]) -> RgbaSpectrum {
        // TODO: There should actually be a default implementation when I get to
        // Monte Carlo.
        todo!()
    }
}

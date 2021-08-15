use super::{Bxdf, BxdfType};
use crate::color::RgbSpectrum;
use cgmath::{Point2, Vector3};
use std::f32::consts::FRAC_1_PI;

/// A BRDF that models reflection off a perfectly diffuse surface, scattering
/// incident light equally in all directions.
///
/// This model is not physically realistic, but it's a useful approximation for
/// modelling matte surfaces.
pub struct LambertianDiffuseReflection {
    /// Reflectance spectrum. The fraction of incident light that is scattered.
    r: RgbSpectrum,
}

impl LambertianDiffuseReflection {
    pub fn new(r: RgbSpectrum) -> Self {
        Self { r }
    }
}

impl Bxdf for LambertianDiffuseReflection {
    fn bxdf_type(&self) -> BxdfType {
        BxdfType::DIFFUSE | BxdfType::REFLECTION
    }

    fn f(&self, _wo: &Vector3<f32>, _wi: &Vector3<f32>) -> RgbSpectrum {
        self.r * FRAC_1_PI
    }

    fn sample_f(
        &self,
        wo: &Vector3<f32>,
        sample: Point2<f32>,
        sampled_type: BxdfType,
    ) -> (Vector3<f32>, f32, RgbSpectrum) {
        todo!()
    }

    fn rho_hd(&self, _wo: &Vector3<f32>, _samples: &[Point2<f32>]) -> RgbSpectrum {
        self.r
    }

    fn rho_hh(&self, _samples1: &[Point2<f32>], _samples2: &[Point2<f32>]) -> RgbSpectrum {
        self.r
    }
}

/// A BTDF that models transmission through a perfectly diffuse surface,
/// scattering incident light equally in all directions.
///
/// This model is not physically realistic, but it's a useful approximation for
/// modelling matte surfaces.
pub struct LambertianDiffuseTransmission {
    /// Transmittance spectrum. The fraction of incident light that is scattered.
    t: RgbSpectrum,
}

impl LambertianDiffuseTransmission {
    pub fn new(r: RgbSpectrum) -> Self {
        Self { t: r }
    }
}

impl Bxdf for LambertianDiffuseTransmission {
    fn bxdf_type(&self) -> BxdfType {
        BxdfType::DIFFUSE | BxdfType::TRANSMISSION
    }

    fn f(&self, _wo: &Vector3<f32>, _wi: &Vector3<f32>) -> RgbSpectrum {
        self.t * FRAC_1_PI
    }

    fn sample_f(
        &self,
        wo: &Vector3<f32>,
        sample: Point2<f32>,
        sampled_type: BxdfType,
    ) -> (Vector3<f32>, f32, RgbSpectrum) {
        todo!()
    }

    fn rho_hd(&self, _wo: &Vector3<f32>, _samples: &[Point2<f32>]) -> RgbSpectrum {
        self.t
    }

    fn rho_hh(&self, _samples1: &[Point2<f32>], _samples2: &[Point2<f32>]) -> RgbSpectrum {
        self.t
    }
}

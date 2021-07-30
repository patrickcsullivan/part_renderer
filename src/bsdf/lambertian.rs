use super::{Bxdf, BxdfType};
use crate::color::RgbSpectrum;
use cgmath::{Point2, Vector3};
use std::f32::consts::FRAC_1_PI;

/// A BRDF that models reflection off a perfect diffuse surface that scatters
/// incident light equally in all directions.
///
/// This model is not physically realistic, but it's a useful approximation for
/// modelling matte surfaces.
pub struct LambertianBrdf {
    /// Reflectance spectrum. The fraction of incident light that is scattered.
    r: RgbSpectrum,
}

impl LambertianBrdf {
    pub fn new(r: RgbSpectrum) -> Self {
        Self { r }
    }
}

impl Bxdf for LambertianBrdf {
    fn bxdf_type(&self) -> super::BxdfType {
        BxdfType::BSDF_DIFFUSE | BxdfType::BSDF_REFLECTION
    }

    fn f(&self, _wo: &Vector3<f32>, _wi: &Vector3<f32>) -> RgbSpectrum {
        self.r * FRAC_1_PI
    }

    fn rho_hd(&self, _wo: &Vector3<f32>, _samples: &[Point2<f32>]) -> RgbSpectrum {
        self.r
    }

    fn rho_hh(&self, _samples1: &[Point2<f32>], _samples2: &[Point2<f32>]) -> RgbSpectrum {
        self.r
    }
}

/// A BTDF that models transmission through a perfect diffuse surface that
/// scatters incident light equally in all directions.
///
/// This model is not physically realistic, but it's a useful approximation for
/// modelling matte surfaces.
pub struct LambertianBtdf {
    /// Transmittance spectrum. The fraction of incident light that is scattered.
    t: RgbSpectrum,
}

impl LambertianBtdf {
    pub fn new(r: RgbSpectrum) -> Self {
        Self { t: r }
    }
}

impl Bxdf for LambertianBtdf {
    fn bxdf_type(&self) -> super::BxdfType {
        BxdfType::BSDF_DIFFUSE | BxdfType::BSDF_TRANSMISSION
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

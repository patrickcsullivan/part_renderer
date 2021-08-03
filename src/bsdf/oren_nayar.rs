use super::{
    geometry::{self, abs_cos_theta},
    Bxdf, BxdfType,
};
use crate::color::RgbSpectrum;
use cgmath::{Point2, Vector3};
use std::f32::consts::FRAC_1_PI;

pub struct OrenNayarDiffuseReflection {
    r: RgbSpectrum,
    a: f32,
    b: f32,
}

impl OrenNayarDiffuseReflection {
    /// * r - Reflectance spectrum. The fraction of incident light that is
    ///   scattered.
    /// * sigma - Standard deviation of the microfacet orientation angle in
    ///   radians.
    pub fn new(r: RgbSpectrum, sigma: f32) -> Self {
        let sigma2 = sigma * sigma;
        Self {
            r,
            a: 1.0 - (sigma2 / (2.0 * (sigma2 + 0.33))),
            b: 0.45 * sigma2 / (sigma2 + 0.09),
        }
    }
}

impl Bxdf for OrenNayarDiffuseReflection {
    fn bxdf_type(&self) -> BxdfType {
        BxdfType::BSDF_REFLECTION | BxdfType::BSDF_DIFFUSE
    }

    fn f(&self, wo: &Vector3<f32>, wi: &Vector3<f32>) -> RgbSpectrum {
        let sin_theta_i = geometry::sin_theta(wi);
        let sin_theta_o = geometry::sin_theta(wo);
        let max_cos = if sin_theta_i > 1e-4 && sin_theta_o > 1e-4 {
            let sin_phi_i = geometry::sin_phi(wi);
            let cos_phi_i = geometry::cos_phi(wi);
            let sin_phi_o = geometry::sin_phi(wo);
            let cos_phi_o = geometry::cos_phi(wo);
            let d_cos = cos_phi_i * cos_phi_o + sin_phi_i * sin_phi_o;
            d_cos.max(0.0)
        } else {
            0.0
        };
        let (sin_alpha, tan_beta) = if abs_cos_theta(wi) > abs_cos_theta(wo) {
            (sin_theta_o, sin_theta_i / abs_cos_theta(wi))
        } else {
            (sin_theta_i, sin_theta_o / abs_cos_theta(wo))
        };
        self.r * FRAC_1_PI * (self.a + self.b * max_cos * sin_alpha * tan_beta)
    }
}

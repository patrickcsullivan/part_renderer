use super::{Bxdf, BxdfType};
use crate::color::RgbSpectrum;
use cgmath::{Point2, Vector3};

pub struct ScaledBxdf {
    bxdf: Box<dyn Bxdf>,
    scale: RgbSpectrum,
}

impl ScaledBxdf {
    pub fn new(bxdf: Box<dyn Bxdf>, scale: RgbSpectrum) -> Self {
        Self { bxdf, scale }
    }
}

impl Bxdf for ScaledBxdf {
    fn bxdf_type(&self) -> super::BxdfType {
        self.bxdf.bxdf_type()
    }

    fn f(&self, wo: &Vector3<f32>, wi: &Vector3<f32>) -> RgbSpectrum {
        self.scale * self.bxdf.f(wo, wi)
    }

    fn sample_f(
        &self,
        wo: &Vector3<f32>,
        sample: Point2<f32>,
        sampled_type: BxdfType,
    ) -> (Vector3<f32>, f32, RgbSpectrum) {
        let (wi, pdf, light) = self.bxdf.sample_f(wo, sample, sampled_type);
        (wi, pdf, self.scale * light)
    }

    fn rho_hd(&self, wo: &Vector3<f32>, samples: &[Point2<f32>]) -> RgbSpectrum {
        self.scale * self.bxdf.rho_hd(wo, samples)
    }

    fn rho_hh(&self, samples1: &[Point2<f32>], samples2: &[Point2<f32>]) -> RgbSpectrum {
        self.scale * self.bxdf.rho_hh(samples1, samples2)
    }
}

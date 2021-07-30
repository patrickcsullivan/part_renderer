use super::Bxdf;
use crate::color::RgbSpectrum;

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

    fn f(&self, wo: &cgmath::Vector3<f32>, wi: &cgmath::Vector3<f32>) -> RgbSpectrum {
        self.scale * self.bxdf.f(wo, wi)
    }

    fn sample_f(
        &self,
        wo: &cgmath::Vector3<f32>,
        sample: cgmath::Point2<f32>,
        pdf: f32,
        sampled_type: super::BxdfType,
    ) -> (cgmath::Vector3<f32>, RgbSpectrum) {
        let (wi, light) = self.bxdf.sample_f(wo, sample, pdf, sampled_type);
        (wi, self.scale * light)
    }

    fn rho_hd(&self, wo: &cgmath::Vector3<f32>, samples: &[cgmath::Point2<f32>]) -> RgbSpectrum {
        self.scale * self.bxdf.rho_hd(wo, samples)
    }

    fn rho_hh(
        &self,
        samples1: &[cgmath::Point2<f32>],
        samples2: &[cgmath::Point2<f32>],
    ) -> RgbSpectrum {
        self.scale * self.bxdf.rho_hh(samples1, samples2)
    }
}

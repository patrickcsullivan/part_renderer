use cgmath::Point3;

use crate::color::RgbSpectrum;

pub struct PointLightSource {
    pub intensity: RgbSpectrum,
    pub position: Point3<f32>,
}

impl PointLightSource {
    pub fn new(intensity: RgbSpectrum, position: Point3<f32>) -> Self {
        Self {
            intensity,
            position,
        }
    }
}

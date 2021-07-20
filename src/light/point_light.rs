use cgmath::{InnerSpace, Point3, Vector3};

use crate::{color::RgbSpectrum, geometry::vector, material::Material};

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

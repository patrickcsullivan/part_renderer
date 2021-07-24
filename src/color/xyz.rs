use super::RgbSpectrum;

/// A color in the XYZ color space. XYZ colors are display-independent.
#[derive(Debug, Clone, Copy)]
pub struct Xyz {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Xyz {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn black() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl From<RgbSpectrum> for Xyz {
    fn from(rgb: RgbSpectrum) -> Self {
        let x = 0.412453 * rgb.r() + 0.357580 * rgb.g() + 0.180423 * rgb.b();
        let y = 0.212671 * rgb.r() + 0.715160 * rgb.g() + 0.072169 * rgb.b();
        let z = 0.019334 * rgb.r() + 0.119193 * rgb.g() + 0.950227 * rgb.b();
        Xyz::new(x, y, z)
    }
}

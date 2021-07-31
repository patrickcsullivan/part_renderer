use crate::color::RgbSpectrum;

/// A matte material for a perfectly diffuse surface.
pub struct MatteMaterial {}

impl MatteMaterial {
    /// * kd - Spectral diffuse reflection value.
    /// * sigma - Roughness value.
    pub fn new(kd: RgbSpectrum, sigma: f32) -> Self {
        todo!()
    }
}

//! A library of materials for testing. Eventually, I want to build up a library
//! of useful materials and move it into a separate crate.

use crate::color::RgbSpectrum;

/// A matte material for a perfectly diffuse surface.
pub struct MatteMaterial {}

impl MatteMaterial {
    /// * kd - Spectral diffuse reflection value.
    /// * sigma - Roughness value.
    pub fn new(kd: RgbSpectrum, sigma: f32) -> Self {}
}

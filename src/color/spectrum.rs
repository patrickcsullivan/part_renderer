use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use cgmath::Zero;

/// Represents a spectral power distribution (SPD), a distribution function that
/// describes the amount of light at each wavelength.
pub struct Spectrum {}

// TODO: Does spectrum length need to be hard-coded at compile time?
// If so, the what spectrum lengths do we need to support?

/// Represents a spectrum as 32 discrete samples.
#[derive(Debug, PartialEq)]
pub struct CoefficientSpectrum32 {
    samples: [f32; 32],
}

impl CoefficientSpectrum32 {
    pub fn new(value: f32) -> Self {
        Self {
            samples: [value; 32],
        }
    }

    pub fn is_black(&self) -> bool {
        self.samples.iter().all(|s| s.is_zero())
    }

    pub fn sqrt(&self) -> Self {
        let mut samples = [0.0; 32];
        for (sample, input) in samples.iter_mut().zip(&self.samples) {
            *sample = input.sqrt()
        }
        Self { samples }
    }

    pub fn powf(&self, n: f32) -> Self {
        let mut samples = [0.0; 32];
        for (sample, input) in samples.iter_mut().zip(&self.samples) {
            *sample = input.powf(n)
        }
        Self { samples }
    }

    pub fn lerp(t: f32, s1: &Self, s2: Self) -> Self {
        &((1.0 - t) * s1) + &(t * &s2)
    }

    pub fn clamp(&mut self, min: f32, max: f32) {
        for s in self.samples.iter_mut() {
            *s = s.clamp(min, max)
        }
    }

    pub fn has_nan(&self) -> bool {
        self.samples.iter().any(|s| s.is_nan())
    }
}

impl Add<&CoefficientSpectrum32> for &CoefficientSpectrum32 {
    type Output = CoefficientSpectrum32;

    fn add(self, rhs: &CoefficientSpectrum32) -> Self::Output {
        let mut samples = [0.0; 32];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl AddAssign<&CoefficientSpectrum32> for CoefficientSpectrum32 {
    fn add_assign(&mut self, rhs: &CoefficientSpectrum32) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left += right
        }
    }
}

impl Sub<&CoefficientSpectrum32> for &CoefficientSpectrum32 {
    type Output = CoefficientSpectrum32;

    fn sub(self, rhs: &CoefficientSpectrum32) -> Self::Output {
        let mut samples = [0.0; 32];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl SubAssign<&CoefficientSpectrum32> for CoefficientSpectrum32 {
    fn sub_assign(&mut self, rhs: &CoefficientSpectrum32) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left -= right
        }
    }
}

impl Mul<&CoefficientSpectrum32> for &CoefficientSpectrum32 {
    type Output = CoefficientSpectrum32;

    fn mul(self, rhs: &CoefficientSpectrum32) -> Self::Output {
        let mut samples = [0.0; 32];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl MulAssign<&CoefficientSpectrum32> for CoefficientSpectrum32 {
    fn mul_assign(&mut self, rhs: &CoefficientSpectrum32) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left *= right
        }
    }
}

impl Div<&CoefficientSpectrum32> for &CoefficientSpectrum32 {
    type Output = CoefficientSpectrum32;

    fn div(self, rhs: &CoefficientSpectrum32) -> Self::Output {
        let mut samples = [0.0; 32];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl DivAssign<&CoefficientSpectrum32> for CoefficientSpectrum32 {
    fn div_assign(&mut self, rhs: &CoefficientSpectrum32) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left /= right
        }
    }
}

impl Mul<&CoefficientSpectrum32> for f32 {
    type Output = CoefficientSpectrum32;

    fn mul(self, rhs: &CoefficientSpectrum32) -> Self::Output {
        let mut samples = [0.0; 32];
        for (sample, right) in samples.iter_mut().zip(&rhs.samples) {
            *sample = self * right
        }
        Self::Output { samples }
    }
}

impl Mul<f32> for &CoefficientSpectrum32 {
    type Output = CoefficientSpectrum32;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut samples = [0.0; 32];
        for (sample, left) in samples.iter_mut().zip(&self.samples) {
            *sample = left * rhs
        }
        Self::Output { samples }
    }
}

impl MulAssign<f32> for CoefficientSpectrum32 {
    fn mul_assign(&mut self, rhs: f32) {
        for left in self.samples.iter_mut() {
            *left *= rhs
        }
    }
}

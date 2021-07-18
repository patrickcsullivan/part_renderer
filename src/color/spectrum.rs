use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use cgmath::Zero;

/// Represents a spectral power distribution (SPD), a distribution function that
/// describes the amount of light at each wavelength.
pub struct Spectrum {}

const SAMPLE_COUNT: usize = 60;

/// Represents a spectrum as 60 discrete samples.
#[derive(Debug, PartialEq)]
pub struct CoefficientSpectrum60 {
    samples: [f32; SAMPLE_COUNT],
}

impl CoefficientSpectrum60 {
    pub fn new(value: f32) -> Self {
        Self {
            samples: [value; SAMPLE_COUNT],
        }
    }

    pub fn is_black(&self) -> bool {
        self.samples.iter().all(|s| s.is_zero())
    }

    pub fn sqrt(&self) -> Self {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, input) in samples.iter_mut().zip(&self.samples) {
            *sample = input.sqrt()
        }
        Self { samples }
    }

    pub fn powf(&self, n: f32) -> Self {
        let mut samples = [0.0; SAMPLE_COUNT];
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

// Spectrum addition

impl Add<CoefficientSpectrum60> for CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn add(self, rhs: CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl Add<&CoefficientSpectrum60> for CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn add(self, rhs: &CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl Add<CoefficientSpectrum60> for &CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn add(self, rhs: CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl Add<&CoefficientSpectrum60> for &CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn add(self, rhs: &CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl AddAssign<CoefficientSpectrum60> for CoefficientSpectrum60 {
    fn add_assign(&mut self, rhs: CoefficientSpectrum60) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left += right
        }
    }
}

impl AddAssign<&CoefficientSpectrum60> for CoefficientSpectrum60 {
    fn add_assign(&mut self, rhs: &CoefficientSpectrum60) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left += right
        }
    }
}

// Spectrum subtraction

impl Sub<CoefficientSpectrum60> for CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn sub(self, rhs: CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl Sub<&CoefficientSpectrum60> for CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn sub(self, rhs: &CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl Sub<CoefficientSpectrum60> for &CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn sub(self, rhs: CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl Sub<&CoefficientSpectrum60> for &CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn sub(self, rhs: &CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl SubAssign<CoefficientSpectrum60> for CoefficientSpectrum60 {
    fn sub_assign(&mut self, rhs: CoefficientSpectrum60) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left -= right
        }
    }
}

impl SubAssign<&CoefficientSpectrum60> for CoefficientSpectrum60 {
    fn sub_assign(&mut self, rhs: &CoefficientSpectrum60) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left -= right
        }
    }
}

// Spectrum multiplication

impl Mul<CoefficientSpectrum60> for CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn mul(self, rhs: CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl Mul<&CoefficientSpectrum60> for CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn mul(self, rhs: &CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl Mul<CoefficientSpectrum60> for &CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn mul(self, rhs: CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl Mul<&CoefficientSpectrum60> for &CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn mul(self, rhs: &CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl MulAssign<CoefficientSpectrum60> for CoefficientSpectrum60 {
    fn mul_assign(&mut self, rhs: CoefficientSpectrum60) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left *= right
        }
    }
}

impl MulAssign<&CoefficientSpectrum60> for CoefficientSpectrum60 {
    fn mul_assign(&mut self, rhs: &CoefficientSpectrum60) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left *= right
        }
    }
}

// Spectrum division

impl Div<CoefficientSpectrum60> for CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn div(self, rhs: CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl Div<&CoefficientSpectrum60> for CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn div(self, rhs: &CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl Div<CoefficientSpectrum60> for &CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn div(self, rhs: CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl Div<&CoefficientSpectrum60> for &CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn div(self, rhs: &CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl DivAssign<CoefficientSpectrum60> for CoefficientSpectrum60 {
    fn div_assign(&mut self, rhs: CoefficientSpectrum60) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left /= right
        }
    }
}

impl DivAssign<&CoefficientSpectrum60> for CoefficientSpectrum60 {
    fn div_assign(&mut self, rhs: &CoefficientSpectrum60) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left /= right
        }
    }
}

// Scalar multiplication

impl Mul<CoefficientSpectrum60> for f32 {
    type Output = CoefficientSpectrum60;

    fn mul(self, rhs: CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, right) in samples.iter_mut().zip(&rhs.samples) {
            *sample = self * right
        }
        Self::Output { samples }
    }
}

impl Mul<&CoefficientSpectrum60> for f32 {
    type Output = CoefficientSpectrum60;

    fn mul(self, rhs: &CoefficientSpectrum60) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, right) in samples.iter_mut().zip(&rhs.samples) {
            *sample = self * right
        }
        Self::Output { samples }
    }
}

impl Mul<f32> for CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, left) in samples.iter_mut().zip(&self.samples) {
            *sample = left * rhs
        }
        Self::Output { samples }
    }
}

impl Mul<f32> for &CoefficientSpectrum60 {
    type Output = CoefficientSpectrum60;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, left) in samples.iter_mut().zip(&self.samples) {
            *sample = left * rhs
        }
        Self::Output { samples }
    }
}

impl MulAssign<f32> for CoefficientSpectrum60 {
    fn mul_assign(&mut self, rhs: f32) {
        for left in self.samples.iter_mut() {
            *left *= rhs
        }
    }
}

// Division by a scalar is excluded because it's always more efficient to
// multiply by a reciprocal.

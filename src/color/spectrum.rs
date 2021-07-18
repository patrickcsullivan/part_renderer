use cgmath::Zero;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Represents a spectral power distribution (SPD), a distribution function that
/// describes the amount of light at each wavelength.
///
/// Currently, the only implementation of `Spectrum` is `RgbSpectrum`. However,
/// PBR ed. 2 describes a `SampleSpectrum` that is structured very similarly to
/// `RgbSpectrum` but is backed by a 60-element array of samples. If we were to
/// implement `SampleSpectrum`, we could easily swap out the type alias.
pub type Spectrum = RgbSpectrum;

const SAMPLE_COUNT: usize = 3;

/// Represents a spectrum as 60 discrete samples.
#[derive(Debug, PartialEq)]
pub struct RgbSpectrum {
    samples: [f32; SAMPLE_COUNT],
}

impl RgbSpectrum {
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

impl Add<RgbSpectrum> for RgbSpectrum {
    type Output = RgbSpectrum;

    fn add(self, rhs: RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl Add<&RgbSpectrum> for RgbSpectrum {
    type Output = RgbSpectrum;

    fn add(self, rhs: &RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl Add<RgbSpectrum> for &RgbSpectrum {
    type Output = RgbSpectrum;

    fn add(self, rhs: RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl Add<&RgbSpectrum> for &RgbSpectrum {
    type Output = RgbSpectrum;

    fn add(self, rhs: &RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl AddAssign<RgbSpectrum> for RgbSpectrum {
    fn add_assign(&mut self, rhs: RgbSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left += right
        }
    }
}

impl AddAssign<&RgbSpectrum> for RgbSpectrum {
    fn add_assign(&mut self, rhs: &RgbSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left += right
        }
    }
}

// Spectrum subtraction

impl Sub<RgbSpectrum> for RgbSpectrum {
    type Output = RgbSpectrum;

    fn sub(self, rhs: RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl Sub<&RgbSpectrum> for RgbSpectrum {
    type Output = RgbSpectrum;

    fn sub(self, rhs: &RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl Sub<RgbSpectrum> for &RgbSpectrum {
    type Output = RgbSpectrum;

    fn sub(self, rhs: RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl Sub<&RgbSpectrum> for &RgbSpectrum {
    type Output = RgbSpectrum;

    fn sub(self, rhs: &RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl SubAssign<RgbSpectrum> for RgbSpectrum {
    fn sub_assign(&mut self, rhs: RgbSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left -= right
        }
    }
}

impl SubAssign<&RgbSpectrum> for RgbSpectrum {
    fn sub_assign(&mut self, rhs: &RgbSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left -= right
        }
    }
}

// Spectrum multiplication

impl Mul<RgbSpectrum> for RgbSpectrum {
    type Output = RgbSpectrum;

    fn mul(self, rhs: RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl Mul<&RgbSpectrum> for RgbSpectrum {
    type Output = RgbSpectrum;

    fn mul(self, rhs: &RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl Mul<RgbSpectrum> for &RgbSpectrum {
    type Output = RgbSpectrum;

    fn mul(self, rhs: RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl Mul<&RgbSpectrum> for &RgbSpectrum {
    type Output = RgbSpectrum;

    fn mul(self, rhs: &RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl MulAssign<RgbSpectrum> for RgbSpectrum {
    fn mul_assign(&mut self, rhs: RgbSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left *= right
        }
    }
}

impl MulAssign<&RgbSpectrum> for RgbSpectrum {
    fn mul_assign(&mut self, rhs: &RgbSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left *= right
        }
    }
}

// Spectrum division

impl Div<RgbSpectrum> for RgbSpectrum {
    type Output = RgbSpectrum;

    fn div(self, rhs: RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl Div<&RgbSpectrum> for RgbSpectrum {
    type Output = RgbSpectrum;

    fn div(self, rhs: &RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl Div<RgbSpectrum> for &RgbSpectrum {
    type Output = RgbSpectrum;

    fn div(self, rhs: RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl Div<&RgbSpectrum> for &RgbSpectrum {
    type Output = RgbSpectrum;

    fn div(self, rhs: &RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl DivAssign<RgbSpectrum> for RgbSpectrum {
    fn div_assign(&mut self, rhs: RgbSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left /= right
        }
    }
}

impl DivAssign<&RgbSpectrum> for RgbSpectrum {
    fn div_assign(&mut self, rhs: &RgbSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left /= right
        }
    }
}

// Scalar multiplication

impl Mul<RgbSpectrum> for f32 {
    type Output = RgbSpectrum;

    fn mul(self, rhs: RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, right) in samples.iter_mut().zip(&rhs.samples) {
            *sample = self * right
        }
        Self::Output { samples }
    }
}

impl Mul<&RgbSpectrum> for f32 {
    type Output = RgbSpectrum;

    fn mul(self, rhs: &RgbSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, right) in samples.iter_mut().zip(&rhs.samples) {
            *sample = self * right
        }
        Self::Output { samples }
    }
}

impl Mul<f32> for RgbSpectrum {
    type Output = RgbSpectrum;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, left) in samples.iter_mut().zip(&self.samples) {
            *sample = left * rhs
        }
        Self::Output { samples }
    }
}

impl Mul<f32> for &RgbSpectrum {
    type Output = RgbSpectrum;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, left) in samples.iter_mut().zip(&self.samples) {
            *sample = left * rhs
        }
        Self::Output { samples }
    }
}

impl MulAssign<f32> for RgbSpectrum {
    fn mul_assign(&mut self, rhs: f32) {
        for left in self.samples.iter_mut() {
            *left *= rhs
        }
    }
}

// Division by a scalar is excluded because it's always more efficient to
// multiply by a reciprocal.

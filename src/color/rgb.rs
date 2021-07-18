use cgmath::Zero;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::number;

use super::Xyz;

const SAMPLE_COUNT: usize = 3;

/// Represents a spectral power distribution (SPD), a distribution function that
/// describes the amount of light at each wavelength.
///
/// This particular representation of an SPD contains only three samples, one
/// each for red, green, and blue.
#[derive(Debug, PartialEq)]
pub struct RgbSpectrum {
    samples: [f32; SAMPLE_COUNT],
}

impl RgbSpectrum {
    pub fn constant(value: f32) -> Self {
        Self {
            samples: [value; SAMPLE_COUNT],
        }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { samples: [r, g, b] }
    }

    pub fn r(&self) -> f32 {
        self.samples[0]
    }

    pub fn g(&self) -> f32 {
        self.samples[1]
    }

    pub fn b(&self) -> f32 {
        self.samples[2]
    }

    /// Creates an RGB spectrum from the given set of arbirary samples. Each
    /// sample contains a wavelength in nanometers and a sample value.
    ///
    /// This method sorts the given samples by wavelength as a side effect.
    pub fn from_sampled(samples: &mut [(f32, f32)]) -> Self {
        samples.sort_by(|(wavelength1, _), (wavelength2, _)| {
            number::f32::total_cmp(*wavelength1, *wavelength2)
        });

        todo!() // TODO: Finish implementing. See p. 333.
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

    pub fn lerp(t: f32, s1: &Self, s2: &Self) -> Self {
        (1.0 - t) * s1 + t * s2
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

impl From<Xyz> for RgbSpectrum {
    fn from(xyz: Xyz) -> Self {
        let r = 3.240479 * xyz.x - 1.53715 * xyz.y - 0.498535 * xyz.z;
        let g = -0.969256 * xyz.x + 1.875991 * xyz.y + 0.041556 * xyz.z;
        let b = 0.055648 * xyz.x - 0.204043 * xyz.y + 1.057311 * xyz.z;
        Self::from_rgb(r, g, b)
    }
}

impl From<RgbSpectrum> for image::Rgb<u8> {
    fn from(rgb: RgbSpectrum) -> Self {
        image::Rgb([
            component_f32_into_u8(rgb.r()),
            component_f32_into_u8(rgb.g()),
            component_f32_into_u8(rgb.b()),
        ])
    }
}

fn component_f32_into_u8(c: f32) -> u8 {
    if c < 0.0 {
        0
    } else if c >= 1.0 {
        255
    } else {
        (c * 256.0) as u8
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

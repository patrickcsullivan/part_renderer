use super::Xyza;
use cgmath::Zero;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

const SAMPLE_COUNT: usize = 4;

/// Represents a spectral power distribution (SPD), a distribution function that
/// describes the amount of light at each wavelength.
///
/// This particular representation of an SPD contains three samples, one each
/// for red, green, and blue.
///
/// A fourth sample, alpha, which describes transparency is also included.
/// Unlike the other samples, this does not describe a physical property of
/// light. Nonetheless, it is useful when converting an SPD into a final image
/// since it allows us to apply the non-phyically-based effect of making some
/// pixels transparent.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RgbaSpectrum {
    samples: [f32; SAMPLE_COUNT],
}

impl RgbaSpectrum {
    pub fn constant(value: f32) -> Self {
        Self {
            samples: [value, value, value, 1.0],
        }
    }

    pub fn black() -> Self {
        Self::constant(0.0)
    }

    pub fn transparent() -> Self {
        Self::from_rgba(0.0, 0.0, 0.0, 0.0)
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            samples: [r, g, b, 1.0],
        }
    }

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            samples: [r, g, b, a],
        }
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

    pub fn a(&self) -> f32 {
        self.samples[3]
    }

    pub fn set_a(&mut self, a: f32) {
        self.samples[3] = a;
    }

    // /// Creates an RGB spectrum from the given set of arbirary samples. Each
    // /// sample contains a wavelength in nanometers and a sample value.
    // ///
    // /// This method sorts the given samples by wavelength as a side effect.
    // pub fn from_sampled(samples: &mut [(f32, f32)]) -> Self {
    //     samples.sort_by(|(wavelength1, _), (wavelength2, _)| {
    //         number::f32::total_cmp(wavelength1, wavelength2)
    //     });

    //     todo!() // TODO: Finish implementing. See p. 333.
    // }

    pub fn is_black(&self) -> bool {
        self.r().is_zero() && self.g().is_zero() && self.b().is_zero()
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

impl From<Xyza> for RgbaSpectrum {
    fn from(xyz: Xyza) -> Self {
        let r = 3.240479 * xyz.x() - 1.53715 * xyz.y() - 0.498535 * xyz.z();
        let g = -0.969256 * xyz.x() + 1.875991 * xyz.y() + 0.041556 * xyz.z();
        let b = 0.055648 * xyz.x() - 0.204043 * xyz.y() + 1.057311 * xyz.z();
        let a = xyz.a();
        Self::from_rgba(r, g, b, a)
    }
}

impl From<RgbaSpectrum> for image::Rgb<u8> {
    fn from(rgb: RgbaSpectrum) -> Self {
        image::Rgb([
            component_f32_into_u8(rgb.r()),
            component_f32_into_u8(rgb.g()),
            component_f32_into_u8(rgb.b()),
        ])
    }
}

impl From<RgbaSpectrum> for image::Rgba<u8> {
    fn from(rgb: RgbaSpectrum) -> Self {
        image::Rgba([
            component_f32_into_u8(rgb.r()),
            component_f32_into_u8(rgb.g()),
            component_f32_into_u8(rgb.b()),
            component_f32_into_u8(rgb.a()),
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

impl Add<RgbaSpectrum> for RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn add(self, rhs: RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl Add<&RgbaSpectrum> for RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn add(self, rhs: &RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl Add<RgbaSpectrum> for &RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn add(self, rhs: RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl Add<&RgbaSpectrum> for &RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn add(self, rhs: &RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left + right
        }
        Self::Output { samples }
    }
}

impl AddAssign<RgbaSpectrum> for RgbaSpectrum {
    fn add_assign(&mut self, rhs: RgbaSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left += right
        }
    }
}

impl AddAssign<&RgbaSpectrum> for RgbaSpectrum {
    fn add_assign(&mut self, rhs: &RgbaSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left += right
        }
    }
}

// Spectrum subtraction

impl Sub<RgbaSpectrum> for RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn sub(self, rhs: RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl Sub<&RgbaSpectrum> for RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn sub(self, rhs: &RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl Sub<RgbaSpectrum> for &RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn sub(self, rhs: RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl Sub<&RgbaSpectrum> for &RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn sub(self, rhs: &RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left - right
        }
        Self::Output { samples }
    }
}

impl SubAssign<RgbaSpectrum> for RgbaSpectrum {
    fn sub_assign(&mut self, rhs: RgbaSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left -= right
        }
    }
}

impl SubAssign<&RgbaSpectrum> for RgbaSpectrum {
    fn sub_assign(&mut self, rhs: &RgbaSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left -= right
        }
    }
}

// Spectrum multiplication

impl Mul<RgbaSpectrum> for RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn mul(self, rhs: RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl Mul<&RgbaSpectrum> for RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn mul(self, rhs: &RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl Mul<RgbaSpectrum> for &RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn mul(self, rhs: RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl Mul<&RgbaSpectrum> for &RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn mul(self, rhs: &RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left * right
        }
        Self::Output { samples }
    }
}

impl MulAssign<RgbaSpectrum> for RgbaSpectrum {
    fn mul_assign(&mut self, rhs: RgbaSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left *= right
        }
    }
}

impl MulAssign<&RgbaSpectrum> for RgbaSpectrum {
    fn mul_assign(&mut self, rhs: &RgbaSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left *= right
        }
    }
}

// Spectrum division

impl Div<RgbaSpectrum> for RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn div(self, rhs: RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl Div<&RgbaSpectrum> for RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn div(self, rhs: &RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl Div<RgbaSpectrum> for &RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn div(self, rhs: RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl Div<&RgbaSpectrum> for &RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn div(self, rhs: &RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for ((sample, left), right) in samples.iter_mut().zip(&self.samples).zip(&rhs.samples) {
            *sample = left / right
        }
        Self::Output { samples }
    }
}

impl DivAssign<RgbaSpectrum> for RgbaSpectrum {
    fn div_assign(&mut self, rhs: RgbaSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left /= right
        }
    }
}

impl DivAssign<&RgbaSpectrum> for RgbaSpectrum {
    fn div_assign(&mut self, rhs: &RgbaSpectrum) {
        for (left, right) in self.samples.iter_mut().zip(&rhs.samples) {
            *left /= right
        }
    }
}

// Scalar multiplication

impl Mul<RgbaSpectrum> for f32 {
    type Output = RgbaSpectrum;

    fn mul(self, rhs: RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, right) in samples.iter_mut().zip(&rhs.samples) {
            *sample = self * right
        }
        Self::Output { samples }
    }
}

impl Mul<&RgbaSpectrum> for f32 {
    type Output = RgbaSpectrum;

    fn mul(self, rhs: &RgbaSpectrum) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, right) in samples.iter_mut().zip(&rhs.samples) {
            *sample = self * right
        }
        Self::Output { samples }
    }
}

impl Mul<f32> for RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, left) in samples.iter_mut().zip(&self.samples) {
            *sample = left * rhs
        }
        Self::Output { samples }
    }
}

impl Mul<f32> for &RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        for (sample, left) in samples.iter_mut().zip(&self.samples) {
            *sample = left * rhs
        }
        Self::Output { samples }
    }
}

impl MulAssign<f32> for RgbaSpectrum {
    fn mul_assign(&mut self, rhs: f32) {
        for left in self.samples.iter_mut() {
            *left *= rhs
        }
    }
}

// Scalar division.

impl Div<f32> for RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn div(self, rhs: f32) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        let inv_rhs = 1.0 / rhs;
        for (sample, left) in samples.iter_mut().zip(&self.samples) {
            *sample = left * inv_rhs
        }
        Self::Output { samples }
    }
}

impl Div<f32> for &RgbaSpectrum {
    type Output = RgbaSpectrum;

    fn div(self, rhs: f32) -> Self::Output {
        let mut samples = [0.0; SAMPLE_COUNT];
        let inv_rhs = 1.0 / rhs;
        for (sample, left) in samples.iter_mut().zip(&self.samples) {
            *sample = left * inv_rhs
        }
        Self::Output { samples }
    }
}

impl DivAssign<f32> for RgbaSpectrum {
    fn div_assign(&mut self, rhs: f32) {
        let inv_rhs = 1.0 / rhs;
        for left in self.samples.iter_mut() {
            *left *= inv_rhs
        }
    }
}

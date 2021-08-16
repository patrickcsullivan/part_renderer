use super::RgbaSpectrum;
use std::ops::{Add, AddAssign};

const COMPONENT_COUNT: usize = 4;

/// A color in the XYZ color space. XYZ colors are display-independent.
///
/// A fourth component, alpha, describes transparency is also included. Unlike
/// Although alpha is not part of the XYZ color space, it is useful when
/// converting an XYZ color into a final image since it allows us to apply the
/// non-phyically-based effect of making some pixels transparent.
#[derive(Debug, Clone, Copy)]
pub struct Xyza {
    components: [f32; COMPONENT_COUNT],
}

impl Xyza {
    pub fn new(x: f32, y: f32, z: f32, a: f32) -> Self {
        Self {
            components: [x, y, z, a],
        }
    }

    pub fn black() -> Self {
        Self {
            components: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn transparent() -> Self {
        Self {
            components: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn x(&self) -> f32 {
        self.components[0]
    }

    pub fn y(&self) -> f32 {
        self.components[1]
    }

    pub fn z(&self) -> f32 {
        self.components[2]
    }

    pub fn a(&self) -> f32 {
        self.components[3]
    }
}

impl From<RgbaSpectrum> for Xyza {
    fn from(rgb: RgbaSpectrum) -> Self {
        let x = 0.412453 * rgb.r() + 0.357580 * rgb.g() + 0.180423 * rgb.b();
        let y = 0.212671 * rgb.r() + 0.715160 * rgb.g() + 0.072169 * rgb.b();
        let z = 0.019334 * rgb.r() + 0.119193 * rgb.g() + 0.950227 * rgb.b();
        let a = rgb.a();
        Xyza::new(x, y, z, a)
    }
}

// Addition

impl Add<Xyza> for Xyza {
    type Output = Xyza;

    fn add(self, rhs: Xyza) -> Self::Output {
        let mut components = [0.0; COMPONENT_COUNT];
        for ((sample, left), right) in components
            .iter_mut()
            .zip(&self.components)
            .zip(&rhs.components)
        {
            *sample = left + right
        }
        Self::Output { components }
    }
}

impl Add<&Xyza> for Xyza {
    type Output = Xyza;

    fn add(self, rhs: &Xyza) -> Self::Output {
        let mut components = [0.0; COMPONENT_COUNT];
        for ((sample, left), right) in components
            .iter_mut()
            .zip(&self.components)
            .zip(&rhs.components)
        {
            *sample = left + right
        }
        Self::Output { components }
    }
}

impl Add<Xyza> for &Xyza {
    type Output = Xyza;

    fn add(self, rhs: Xyza) -> Self::Output {
        let mut components = [0.0; COMPONENT_COUNT];
        for ((sample, left), right) in components
            .iter_mut()
            .zip(&self.components)
            .zip(&rhs.components)
        {
            *sample = left + right
        }
        Self::Output { components }
    }
}

impl Add<&Xyza> for &Xyza {
    type Output = Xyza;

    fn add(self, rhs: &Xyza) -> Self::Output {
        let mut components = [0.0; COMPONENT_COUNT];
        for ((sample, left), right) in components
            .iter_mut()
            .zip(&self.components)
            .zip(&rhs.components)
        {
            *sample = left + right
        }
        Self::Output { components }
    }
}

impl AddAssign<Xyza> for Xyza {
    fn add_assign(&mut self, rhs: Xyza) {
        for (left, right) in self.components.iter_mut().zip(&rhs.components) {
            *left += right
        }
    }
}

impl AddAssign<&Xyza> for Xyza {
    fn add_assign(&mut self, rhs: &Xyza) {
        for (left, right) in self.components.iter_mut().zip(&rhs.components) {
            *left += right
        }
    }
}

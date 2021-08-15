use super::RgbSpectrum;
use std::ops::{Add, AddAssign};

const COMPONENT_COUNT: usize = 3;

/// A color in the XYZ color space. XYZ colors are display-independent.
#[derive(Debug, Clone, Copy)]
pub struct Xyz {
    components: [f32; COMPONENT_COUNT],
}

impl Xyz {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            components: [x, y, z],
        }
    }

    pub fn black() -> Self {
        Self {
            components: [0.0, 0.0, 0.0],
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
}

impl From<RgbSpectrum> for Xyz {
    fn from(rgb: RgbSpectrum) -> Self {
        let x = 0.412453 * rgb.r() + 0.357580 * rgb.g() + 0.180423 * rgb.b();
        let y = 0.212671 * rgb.r() + 0.715160 * rgb.g() + 0.072169 * rgb.b();
        let z = 0.019334 * rgb.r() + 0.119193 * rgb.g() + 0.950227 * rgb.b();
        Xyz::new(x, y, z)
    }
}

// Addition

impl Add<Xyz> for Xyz {
    type Output = Xyz;

    fn add(self, rhs: Xyz) -> Self::Output {
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

impl Add<&Xyz> for Xyz {
    type Output = Xyz;

    fn add(self, rhs: &Xyz) -> Self::Output {
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

impl Add<Xyz> for &Xyz {
    type Output = Xyz;

    fn add(self, rhs: Xyz) -> Self::Output {
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

impl Add<&Xyz> for &Xyz {
    type Output = Xyz;

    fn add(self, rhs: &Xyz) -> Self::Output {
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

impl AddAssign<Xyz> for Xyz {
    fn add_assign(&mut self, rhs: Xyz) {
        for (left, right) in self.components.iter_mut().zip(&rhs.components) {
            *left += right
        }
    }
}

impl AddAssign<&Xyz> for Xyz {
    fn add_assign(&mut self, rhs: &Xyz) {
        for (left, right) in self.components.iter_mut().zip(&rhs.components) {
            *left += right
        }
    }
}

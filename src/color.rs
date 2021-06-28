use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb { r, g, b }
    }
}

impl Add<Rgb> for Rgb {
    type Output = Rgb;

    fn add(self, rhs: Rgb) -> Self::Output {
        Rgb::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign<Rgb> for Rgb {
    fn add_assign(&mut self, rhs: Rgb) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<f32> for Rgb {
    type Output = Rgb;

    fn mul(self, rhs: f32) -> Self::Output {
        Rgb::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<Rgb> for f32 {
    type Output = Rgb;

    fn mul(self, rhs: Rgb) -> Self::Output {
        Rgb::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl Mul<Rgb> for Rgb {
    type Output = Rgb;

    fn mul(self, rhs: Rgb) -> Self::Output {
        Rgb::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl MulAssign<f32> for Rgb {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl MulAssign<Rgb> for Rgb {
    fn mul_assign(&mut self, rhs: Rgb) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl From<Rgb> for image::Rgb<u8> {
    fn from(rgb: Rgb) -> Self {
        image::Rgb([
            component_f32_into_u8(rgb.r),
            component_f32_into_u8(rgb.g),
            component_f32_into_u8(rgb.b),
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

use crate::color::Xyz;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    /// The color at the pixel in the XYZ color space.
    xyz: Xyz,

    filter_weight_sum: f32,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            xyz: Xyz::black(),
            filter_weight_sum: 0.0,
        }
    }
}

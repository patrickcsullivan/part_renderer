use crate::color::RgbaSpectrum;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: RgbaSpectrum,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,

    /// The degree to which the material reflects light. 0 is completely
    /// nonreflective. 1 is a perfect mirror.
    pub reflective: f32,
}

impl Material {
    pub fn new(
        color: RgbaSpectrum,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
        reflective: f32,
    ) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
        }
    }
}

use crate::color::Rgb;

#[derive(Debug)]
pub struct Material {
    pub color: Rgb,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new(color: Rgb, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

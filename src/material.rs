use crate::color::Rgb;

#[derive(Debug)]
pub struct Material {
    pub color: Rgb,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Rgb::new(0.0, 0.0, 0.0),
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
        }
    }
}

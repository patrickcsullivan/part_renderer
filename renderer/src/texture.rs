use crate::interaction::SurfaceInteraction;

pub trait Texture<T> {
    fn evaluate(&self, interaction: &SurfaceInteraction) -> T;
}

pub struct ConstantTexture<T: Clone> {
    value: T,
}

impl<T: Clone> ConstantTexture<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, _interaction: &SurfaceInteraction) -> T {
        self.value.clone()
    }
}

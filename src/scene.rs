use crate::{
    interaction::SurfaceInteraction,
    light_v1::LightSource,
    primitive::{Primitive, PrimitiveAggregate},
    ray::Ray,
};

pub struct Scene<'msh, 'mtrx, 'mtrl> {
    pub primitives: PrimitiveAggregate<'msh, 'mtrx, 'mtrl>,
    pub lights: Vec<LightSource>,
}

impl<'msh, 'mtrx, 'mtrl> Scene<'msh, 'mtrx, 'mtrl> {
    pub fn new(
        renderable: PrimitiveAggregate<'msh, 'mtrx, 'mtrl>,
        lights: Vec<LightSource>,
    ) -> Self {
        Self {
            primitives: renderable,
            lights,
        }
    }

    // Find the first primitive the ray intersects. Return the parametric value
    // at the intersection, a reference to the primitive, and a description of
    // the primitive-ray interaction.
    pub fn ray_intersection(
        &self,
        ray: &Ray,
    ) -> Option<(f32, Primitive<'msh, 'mtrx, 'mtrl>, SurfaceInteraction)> {
        self.primitives.ray_intersection(ray)
    }
}

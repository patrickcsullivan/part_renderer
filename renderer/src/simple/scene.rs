use super::primitive::{Primitive, PrimitiveAggregate};
use crate::{interaction::SurfaceInteraction, light::Light, ray::Ray};

pub struct Scene<'msh> {
    pub primitives: PrimitiveAggregate<'msh>,
    pub lights: Vec<Light>,
}

impl<'msh> Scene<'msh> {
    pub fn new(primitives: PrimitiveAggregate<'msh>, lights: Vec<Light>) -> Self {
        Self { primitives, lights }
    }

    // Find the first primitive the ray intersects. Return the parametric value
    // at the intersection, a reference to the primitive, and a description of
    // the primitive-ray interaction.
    pub fn ray_intersection(
        &self,
        ray: &Ray,
    ) -> Option<(f32, Primitive<'msh>, SurfaceInteraction)> {
        self.primitives.ray_intersection(ray)
    }
}

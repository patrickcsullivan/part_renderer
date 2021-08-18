use crate::{
    interaction::SurfaceInteraction,
    light::Light,
    primitive::{Primitive, PrimitiveAggregate},
    ray::Ray,
};

pub struct Scene<'msh, 'mtrl> {
    pub primitives: PrimitiveAggregate<'msh, 'mtrl>,
    pub lights: Vec<Light>,
}

impl<'msh, 'mtrl> Scene<'msh, 'mtrl> {
    pub fn new(primitives: PrimitiveAggregate<'msh, 'mtrl>, lights: Vec<Light>) -> Self {
        Self { primitives, lights }
    }

    // Find the first primitive the ray intersects. Return the parametric value
    // at the intersection, a reference to the primitive, and a description of
    // the primitive-ray interaction.
    pub fn ray_intersection(
        &self,
        ray: &Ray,
    ) -> Option<(f32, Primitive<'msh, 'mtrl>, SurfaceInteraction)> {
        self.primitives.ray_intersection(ray)
    }
}

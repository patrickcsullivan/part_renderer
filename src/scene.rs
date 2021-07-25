use crate::{
    camera::{Camera, CameraSample},
    film::Film,
    integrator::RayTracer,
    interaction::SurfaceInteraction,
    light::LightSource,
    primitive::{Primitive, PrimitiveAggregate},
    ray::Ray,
    sampler::Sampler,
};
use cgmath::Point2;
use image::ImageBuffer;
use typed_arena::Arena;

pub struct Scene<'msh, 'mtrx, 'mtrl> {
    pub primitives: PrimitiveAggregate<'msh, 'mtrx, 'mtrl>,
    pub lights: Vec<LightSource>,
}

pub fn render<'msh, 'mtrx, 'mtrl, S: Sampler>(
    scene: &Scene<'msh, 'mtrx, 'mtrl>,
    camera: Box<dyn Camera>,
    film: &mut Film,
    ray_tracer: Box<dyn RayTracer<'msh, 'mtrx, 'mtrl, S>>,
    max_depth: usize,
) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    ImageBuffer::from_fn(
        film.resolution.x as u32,
        film.resolution.y as u32,
        |x, y| {
            println!("At ({}, {})", x, y);
            let sample = CameraSample::at_pixel_center(Point2::new(x as i32, y as i32));
            let (ray, _) = camera.generate_ray(&sample);

            // Unused but needed for incoming_radiance call.
            let mut sampler = S::new(0);
            let mut spectrum_arena = Arena::new();

            let color = ray_tracer.incoming_radiance(
                &ray,
                scene,
                &mut sampler,
                &mut spectrum_arena,
                0,
                max_depth,
            );
            let pixel: image::Rgb<u8> = color.into();
            pixel
        },
    )
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

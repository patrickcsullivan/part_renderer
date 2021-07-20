mod camera;
mod color;
mod demo;
mod geometry;
mod integrator;
mod interaction;
mod light;
mod material;
mod number;
mod primitive;
mod ray;
mod sampler;
mod scene;
mod shape;

#[cfg(test)]
mod test;

use integrator::WhittedIntegrator;
use sampler::Sampler;
use scene::Scene;

fn main() {
    println!("Hello, world!");
    crate::demo::simple();
    crate::demo::complex();
}

pub fn main_todo() {
    let scene: Scene = todo!();
    let sampler = MockSampler;
    let camera = todo!();
    let integrator = WhittedIntegrator::new(camera, sampler);
    integrator.render(&scene);
    ()
}

struct MockSampler;

impl Sampler for MockSampler {
    fn samples_per_pixel(&self) -> usize {
        todo!()
    }
    fn start_pixel(&mut self, pixel: cgmath::Point2<usize>) {
        todo!()
    }
    fn get_1d(&mut self) -> f32 {
        todo!()
    }
    fn get_2d(&mut self) -> cgmath::Point2<f32> {
        todo!()
    }
    fn prepare_1d_array(&mut self, count: usize) {
        todo!()
    }
    fn prepare_2d_array(&mut self, count: usize) {
        todo!()
    }
    fn get_1d_vec(&mut self) -> Option<&Vec<f32>> {
        todo!()
    }
    fn get_2d_vec(&mut self) -> Option<&Vec<cgmath::Point2<f32>>> {
        todo!()
    }
    fn start_next_sample(&mut self) -> bool {
        todo!()
    }
    fn start_nth_sample(&mut self, sample_index: usize) -> bool {
        todo!()
    }
}

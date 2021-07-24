mod camera;
mod color;
mod demo;
mod filter;
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

fn main() {
    println!("Hello, world!");
    crate::demo::simple_ortho();
    crate::demo::complex_ortho();
}

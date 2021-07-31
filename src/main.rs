mod bsdf;
mod camera;
mod color;
mod demo;
mod film;
mod filter;
mod geometry;
mod integrator;
mod interaction;
mod light;
mod material;
mod material_v1;
mod materials;
mod number;
mod primitive;
mod ray;
mod sampler;
mod scene;
mod shape;

#[cfg(test)]
mod test;

fn main() {
    println!("Starting...");
    crate::demo::bunny_orth();
}

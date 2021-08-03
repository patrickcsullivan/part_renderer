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
mod number;
mod primitive;
mod ray;
mod sampler;
mod scene;
mod shape;
mod texture;

#[cfg(test)]
mod test;

// TODO: Figure out where to put this.
/// Indicates whether a ray that found a surface interaction was found along a
/// path starting from a camera or a path starting from a light source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportMode {
    Radiance,
    Importance,
}

fn main() {
    println!("Starting...");
    crate::demo::teapot_orth();
}

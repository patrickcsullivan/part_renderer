mod bsdf;
pub mod camera;
pub mod color;
mod demo;
pub mod film;
pub mod filter;
mod geometry;
pub mod integrator;
mod interaction;
pub mod light;
pub mod material;
mod number;
pub mod primitive;
mod ray;
pub mod sampler;
pub mod scene;
mod texture;
mod triangle;

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

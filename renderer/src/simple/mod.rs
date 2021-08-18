mod material;
mod primitive;
mod ray_tracer;
mod scene;

pub use material::Material;
pub use primitive::{Primitive, PrimitiveAggregate};
pub use ray_tracer::OriginalRayTracer;
pub use scene::Scene;

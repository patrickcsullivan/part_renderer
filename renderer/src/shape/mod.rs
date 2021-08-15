mod mesh;

pub use mesh::{Mesh, MeshBuilder, Triangle};

use crate::{interaction::SurfaceInteraction, ray::Ray};
use bvh::aabb::Bounded;
use cgmath::Matrix4;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum Shape<'msh, 'mtrx> {
    Triangle(Triangle<'msh, 'mtrx>),
}

impl<'msh, 'mtrx> Shape<'msh, 'mtrx> {
    pub fn triangle(mesh: &'mtrx Mesh, index_in_mesh: usize) -> Self {
        Self::Triangle(mesh.triangle_at(index_in_mesh))
    }

    /// Returns information about the first ray-shape intersection, if any, in
    /// the (0, `ray.t_max`) parametric range along the ray.
    ///
    /// `ray` is in world space, and the returned surface interaction is in
    /// world space.
    pub fn ray_intersection(&self, ray: &Ray) -> Option<(f32, SurfaceInteraction)> {
        match self {
            Self::Triangle(triangle) => triangle.ray_intersection(ray),
        }
    }
}

impl<'msh, 'mtrx> Bounded for Shape<'msh, 'mtrx> {
    fn aabb(&self) -> bvh::aabb::AABB {
        match self {
            Self::Triangle(triangle) => triangle.aabb(),
        }
    }
}

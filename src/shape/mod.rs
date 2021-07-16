mod plane;
mod sphere;

use crate::{
    interaction::SurfaceInteraction,
    mesh::{Mesh, Triangle},
    ray::Ray,
};
use bvh::aabb::Bounded;
use cgmath::{Matrix4, Point3};
use std::fmt::Debug;

use self::{plane::Plane, sphere::Sphere};

#[derive(Debug, Clone, Copy)]
pub enum Shape<'msh, 'mtrx> {
    Sphere(Sphere<'mtrx>),
    Plane(Plane<'mtrx>),
    Triangle(Triangle<'msh, 'mtrx>),
}

impl<'msh, 'mtrx> Shape<'msh, 'mtrx> {
    pub fn sphere(
        object_to_world: &'mtrx Matrix4<f32>,
        world_to_object: &'mtrx Matrix4<f32>,
        reverse_orientation: bool,
    ) -> Self {
        Self::Sphere(Sphere {
            object_to_world,
            world_to_object,
            reverse_orientation,
        })
    }

    pub fn plane(
        object_to_world: &'mtrx Matrix4<f32>,
        world_to_object: &'mtrx Matrix4<f32>,
        reverse_orientation: bool,
    ) -> Self {
        Self::Plane(Plane {
            object_to_world,
            world_to_object,
            reverse_orientation,
        })
    }

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
            Self::Sphere(sphere) => sphere.ray_intersection(ray),
            Self::Plane(plane) => plane.ray_intersection(ray),
            Self::Triangle(triangle) => triangle.ray_intersection(ray),
        }
    }
}

impl<'msh, 'mtrx> Bounded for Shape<'msh, 'mtrx> {
    fn aabb(&self) -> bvh::aabb::AABB {
        match self {
            Self::Sphere(_sphere) => todo!(),
            Self::Plane(_plane) => todo!(),
            Self::Triangle(triangle) => triangle.aabb(),
        }
    }
}

mod plane;
mod sphere;

use crate::{
    interaction::SurfaceInteraction,
    mesh::{Mesh, Triangle},
    ray::Ray,
};
use cgmath::{Matrix4, Point3};
use std::fmt::Debug;

/// A 3D object that can be transformed and intersected with rays.
#[derive(Debug)]
pub struct Object<'msh, 'mtrx> {
    object_to_world: &'mtrx Matrix4<f32>,
    world_to_object: &'mtrx Matrix4<f32>,
    reverse_orientation: bool,
    geometry: Geometry<'msh, 'mtrx>,
}

impl<'msh, 'mtrx> Object<'msh, 'mtrx> {
    pub fn sphere(
        object_to_world: &'mtrx Matrix4<f32>,
        world_to_object: &'mtrx Matrix4<f32>,
        reverse_orientation: bool,
    ) -> Self {
        Self {
            object_to_world,
            world_to_object,
            reverse_orientation,
            geometry: Geometry::Sphere(),
        }
    }

    pub fn plane(
        object_to_world: &'mtrx Matrix4<f32>,
        world_to_object: &'mtrx Matrix4<f32>,
        reverse_orientation: bool,
    ) -> Self {
        Self {
            object_to_world,
            world_to_object,
            reverse_orientation,
            geometry: Geometry::Plane(),
        }
    }

    pub fn triangle(mesh: &'mtrx Mesh, index_in_mesh: usize) -> Self {
        Self {
            object_to_world: mesh.object_to_world,
            world_to_object: mesh.world_to_object,
            reverse_orientation: mesh.reverse_orientation,
            geometry: Geometry::Triangle(mesh.triangle_at(index_in_mesh)),
        }
    }

    pub fn ray_intersections(&self, ray: &Ray) -> Vec<(f32, SurfaceInteraction)> {
        self.geometry.ray_intersections(
            ray,
            self.object_to_world,
            self.world_to_object,
            self.reverse_orientation,
        )
    }
}

/// A description of 3D geometry.
#[derive(Debug)]
enum Geometry<'msh, 'mtrx> {
    Sphere(),
    Plane(),
    Triangle(Triangle<'msh, 'mtrx>),
}

impl<'msh, 'mtrx> Geometry<'msh, 'mtrx> {
    fn ray_intersections(
        &self,
        ray: &Ray,
        object_to_world: &Matrix4<f32>,
        world_to_object: &Matrix4<f32>,
        reverse_orientation: bool,
    ) -> Vec<(f32, SurfaceInteraction)> {
        match self {
            Self::Sphere() => sphere::ray_intersections(
                ray,
                object_to_world,
                world_to_object,
                reverse_orientation,
            ),
            Self::Plane() => {
                plane::ray_intersections(ray, object_to_world, world_to_object, reverse_orientation)
            }
            Self::Triangle(triangle) => match triangle.ray_intersection(ray) {
                Some((t, interaction)) => vec![(t, interaction)],
                None => vec![],
            },
        }
    }
}

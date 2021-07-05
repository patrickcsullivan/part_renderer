mod plane;
mod sphere;

use crate::{interaction::SurfaceInteraction, ray::Ray};
use cgmath::Matrix4;
use std::fmt::Debug;

/// A transformed 3D object.
#[derive(Debug)]
pub struct Object<'mtrx> {
    object_to_world: &'mtrx Matrix4<f32>,
    world_to_object: &'mtrx Matrix4<f32>,
    reverse_orientation: bool,
    geometry: Geometry,
}

impl<'mtrx> Object<'mtrx> {
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
enum Geometry {
    Sphere(),
    Plane(),
}

impl Geometry {
    fn ray_intersections(
        &self,
        ray: &Ray,
        object_to_world: &Matrix4<f32>,
        world_to_object: &Matrix4<f32>,
        reverse_orientation: bool,
    ) -> Vec<(f32, SurfaceInteraction)> {
        match self {
            Geometry::Sphere() => sphere::ray_intersections(
                ray,
                object_to_world,
                world_to_object,
                reverse_orientation,
            ),
            Geometry::Plane() => {
                plane::ray_intersections(ray, object_to_world, world_to_object, reverse_orientation)
            }
        }
    }
}

mod plane;
mod sphere;
mod triangle;

use crate::{interaction::SurfaceInteraction, ray::Ray};
use cgmath::{Matrix4, Point3};
use std::fmt::Debug;

/// A 3D object that can be transformed and intersected with rays.
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

    pub fn triangle(
        object_to_world: &'mtrx Matrix4<f32>,
        world_to_object: &'mtrx Matrix4<f32>,
        reverse_orientation: bool,
        p1: Point3<f32>,
        p2: Point3<f32>,
        p3: Point3<f32>,
    ) -> Self {
        Self {
            object_to_world,
            world_to_object,
            reverse_orientation,
            geometry: Geometry::Triangle(triangle::Triangle::new(p1, p2, p3)),
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
    Triangle(triangle::Triangle),
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
            Self::Sphere() => sphere::ray_intersections(
                ray,
                object_to_world,
                world_to_object,
                reverse_orientation,
            ),
            Self::Plane() => {
                plane::ray_intersections(ray, object_to_world, world_to_object, reverse_orientation)
            }
            Self::Triangle(triangle) => triangle.ray_intersections(
                ray,
                object_to_world,
                world_to_object,
                reverse_orientation,
            ),
        }
    }
}

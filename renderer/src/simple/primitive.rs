use super::material::Material;
use crate::interaction::SurfaceInteraction;
use crate::number;
use crate::ray::Ray;
use crate::triangle::Triangle;
use bvh::bvh::BVH;
use bvh::{aabb::Bounded, bounding_hierarchy::BHShape};
use mesh::Mesh;

/// Combines a shape and a reference to a material. This is the basic primitive
/// used in the construction of primitives aggregates.
#[derive(Clone, Copy)]
pub struct Primitive<'msh> {
    pub shape: Triangle<'msh>,
    pub material: Material,

    /// Tracks the index of the primitives in a bounding volume
    /// hierarchy if it is stored in one.
    bvh_node_index: usize,
}

impl<'msh> Primitive<'msh> {
    pub fn new(shape: Triangle<'msh>, material: Material) -> Self {
        Self {
            shape,
            material,
            bvh_node_index: 0,
        }
    }
}

impl<'msh> Bounded for Primitive<'msh> {
    fn aabb(&self) -> bvh::aabb::AABB {
        self.shape.aabb()
    }
}

impl<'msh> BHShape for Primitive<'msh> {
    fn set_bh_node_index(&mut self, index: usize) {
        self.bvh_node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.bvh_node_index
    }
}

// An aggregate of primitives, each of which contains a shape and a material.
pub enum PrimitiveAggregate<'msh> {
    Primitive(Primitive<'msh>),
    Vector(Vec<PrimitiveAggregate<'msh>>),
    Bvh(Vec<Primitive<'msh>>, BVH),
}

impl<'msh> PrimitiveAggregate<'msh> {
    pub fn primitive(shape: Triangle<'msh>, material: Material) -> Self {
        Self::Primitive(Primitive::new(shape, material))
    }

    // Find the first primitive the ray intersects. Return the parametric value
    // at the intersection, a reference to the primitive, and a description of
    // the primitive-ray interaction.
    pub fn ray_intersection(
        &self,
        ray: &Ray,
    ) -> Option<(f32, Primitive<'msh>, SurfaceInteraction)> {
        match self {
            PrimitiveAggregate::Primitive(p) => p
                .shape
                .ray_intersection(ray)
                .map(|(t, interaction)| (t, *p, interaction)),
            PrimitiveAggregate::Vector(ps) => ps
                .iter()
                .filter_map(|r| {
                    r.ray_intersection(ray)
                        .map(|(t, p, interaction)| (t, p, interaction))
                })
                .min_by(|(t1, _, _), (t2, _, _)| number::f32::total_cmp(t1, t2)),
            PrimitiveAggregate::Bvh(ps, bvh) => {
                let hit_primitives = bvh.traverse(&ray.into(), ps);
                hit_primitives
                    .iter()
                    .filter_map(|&&p| {
                        p.shape
                            .ray_intersection(ray)
                            .map(|(t, interaction)| (t, p, interaction))
                    })
                    .min_by(|(t1, _, _), (t2, _, _)| number::f32::total_cmp(t1, t2))
            }
        }
    }

    pub fn from_mesh(mesh: &'msh Mesh, material: Material) -> Self {
        let mut primitives: Vec<Primitive> = mesh
            .triangles()
            .into_iter()
            .map(|t| Primitive::new(Triangle(t), material))
            .collect();
        let bvh = BVH::build(&mut primitives);
        Self::Bvh(primitives, bvh)
    }
}

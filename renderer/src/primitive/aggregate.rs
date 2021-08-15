use super::Primitive;
use crate::number;
use crate::{
    interaction::SurfaceInteraction,
    material::Material,
    ray::Ray,
    shape::{Mesh, Shape},
};
use bvh::bvh::BVH;

// An aggregate of primitives, each of which contains a shape and a material.
pub enum PrimitiveAggregate<'msh, 'mtrx, 'mtrl> {
    Primitive(Primitive<'msh, 'mtrx, 'mtrl>),
    Vector(Vec<PrimitiveAggregate<'msh, 'mtrx, 'mtrl>>),
    Bvh(Vec<Primitive<'msh, 'mtrx, 'mtrl>>, BVH),
}

impl<'msh, 'mtrx, 'mtrl> PrimitiveAggregate<'msh, 'mtrx, 'mtrl> {
    pub fn primitive(
        shape: Shape<'msh, 'mtrx>,
        material: &'mtrl (dyn Material + Send + Sync),
    ) -> Self {
        Self::Primitive(Primitive::new(shape, material))
    }

    // Find the first primitive the ray intersects. Return the parametric value
    // at the intersection, a reference to the primitive, and a description of
    // the primitive-ray interaction.
    pub fn ray_intersection(
        &self,
        ray: &Ray,
    ) -> Option<(f32, Primitive<'msh, 'mtrx, 'mtrl>, SurfaceInteraction)> {
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

    pub fn from_mesh(
        mesh: &'msh Mesh<'mtrx>,
        material: &'mtrl (dyn Material + Send + Sync),
    ) -> Self {
        let mut primitives: Vec<Primitive> = mesh
            .triangles()
            .into_iter()
            .map(|t| Primitive::new(Shape::Triangle(t), material))
            .collect();
        let bvh = BVH::build(&mut primitives);
        Self::Bvh(primitives, bvh)
    }
}

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
    pub fn primitive(shape: Shape<'msh, 'mtrx>, material: &'mtrl Material) -> Self {
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

    pub fn from_mesh(mesh: &'msh Mesh<'mtrx>, material: &'mtrl Material) -> Self {
        let mut primitives: Vec<Primitive> = mesh
            .triangles()
            .into_iter()
            .map(|t| Primitive::new(Shape::Triangle(t), material))
            .collect();
        let bvh = BVH::build(&mut primitives);
        Self::Bvh(primitives, bvh)
    }
}

#[cfg(test)]
mod ray_intersections_tests {
    use crate::{
        color::RgbSpectrum, geometry::matrix::identity4, material::Material,
        primitive::PrimitiveAggregate, ray::Ray, shape::Shape, test::ApproxEq,
    };
    use cgmath::{Matrix4, Point3, Transform, Vector3};

    #[test]
    fn ray_intersects_spheres() -> Result<(), String> {
        let identity = identity4();
        let scale = Matrix4::from_scale(0.5);
        let inv_scale = scale.inverse_transform().unwrap();
        let material = Material::new(
            RgbSpectrum::from_rgb(0.8, 1.0, 0.6),
            0.0,
            0.7,
            0.2,
            0.0,
            0.0,
        );
        let sphere1 = Shape::sphere(&identity, &identity, false);
        let sphere2 = Shape::sphere(&scale, &inv_scale, false);
        let primitive1 = PrimitiveAggregate::primitive(sphere1, &material);
        let primitive2 = PrimitiveAggregate::primitive(sphere2, &material);
        let renderable = PrimitiveAggregate::Vector(vec![primitive1, primitive2]);
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        if let Some((t, _, _)) = renderable.ray_intersection(&ray) {
            assert!(t.approx_eq(&4.0));
            // Other intersections would be at 4.5, 5.5, 6.0.
            Ok(())
        } else {
            Err("Expected to find intersection.".to_string())
        }
    }
}
